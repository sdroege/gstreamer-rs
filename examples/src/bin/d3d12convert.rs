// This example shows how to use D3D12Convert object.
// It operates the following pipeline:

// {appsrc} - {d3d12videosink}

// The application renders static color on application's own D3D12 RGBA texture.
// Then the texture is converted to NV12 texture using D3D12Convert object.
// Finally the converted NV12 textures are pushed to pipeline using appsrc.

use anyhow::Error;
use derive_more::derive::{Display, Error};
use gst::prelude::*;
use gst_d3d12::prelude::*;

use windows::{
    core::Interface,
    Win32::Graphics::{Direct3D12::*, Dxgi::Common::*},
};

#[derive(Debug, Display, Error)]
#[display("Received error from {src}: {error} (debug: {debug:?})")]
struct ErrorMessage {
    src: glib::GString,
    error: glib::Error,
    debug: Option<glib::GString>,
}

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

struct CmdResources {
    device: gst_d3d12::D3D12Device,
    cl: ID3D12GraphicsCommandList,
    ca_pool: gst_d3d12::D3D12CmdAllocPool,
    source_rtv_heap: ID3D12DescriptorHeap,
    source_tex: ID3D12Resource,
    source_tex_buf: gst::Buffer,
    fence_data_pool: gst_d3d12::D3D12FenceDataPool,
    fence: ID3D12Fence,
    fence_val: u64,
}

impl Drop for CmdResources {
    fn drop(&mut self) {
        // Make sure all GPU tasks scheduled by us were completed
        if self.fence_val > 0 {
            self.device
                .fence_wait(D3D12_COMMAND_LIST_TYPE_DIRECT, self.fence_val)
                .unwrap();
        }
    }
}

fn create_cmd_resources(device: &gst_d3d12::D3D12Device) -> CmdResources {
    let device_handle = device.device_handle().cast::<ID3D12Device4>().unwrap();

    // Creates command list and command allocator pool
    let cl = unsafe {
        device_handle
            .CreateCommandList1(
                0,
                D3D12_COMMAND_LIST_TYPE_DIRECT,
                D3D12_COMMAND_LIST_FLAG_NONE,
            )
            .unwrap()
    };

    let ca_pool = gst_d3d12::D3D12CmdAllocPool::new(&device_handle, D3D12_COMMAND_LIST_TYPE_DIRECT);

    // Our source texture. We will clear this texture with variable colors
    // then it will be converted to other format using D3D12Converter object
    let mut source_tex: Option<ID3D12Resource> = None;
    unsafe {
        device_handle
            .CreateCommittedResource(
                &D3D12_HEAP_PROPERTIES {
                    Type: D3D12_HEAP_TYPE_DEFAULT,
                    ..Default::default()
                },
                D3D12_HEAP_FLAG_NONE,
                &D3D12_RESOURCE_DESC {
                    Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
                    Alignment: 0,
                    Width: WIDTH as u64,
                    Height: HEIGHT,
                    DepthOrArraySize: 1,
                    MipLevels: 1,
                    SampleDesc: DXGI_SAMPLE_DESC {
                        Count: 1,
                        Quality: 0,
                    },
                    Layout: D3D12_TEXTURE_LAYOUT_UNKNOWN,
                    Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                    Flags: D3D12_RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS
                        | D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET,
                },
                D3D12_RESOURCE_STATE_COMMON,
                None,
                &mut source_tex,
            )
            .unwrap()
    }
    let source_tex = source_tex.unwrap();

    // Wrap our texture using memory/buffer, so that it can be used
    // as an input of D3D12Converter
    let mem = gst_d3d12::D3D12Allocator::alloc_wrapped(device, &source_tex, 0).unwrap();
    let source_rtv_heap = {
        let dmem = mem.downcast_memory_ref::<gst_d3d12::D3D12Memory>().unwrap();
        // GstD3D12Memory will create RTV heap automatically
        dmem.render_target_view_heap().unwrap()
    };

    let mut source_tex_buf = gst::Buffer::new();
    {
        let buf = source_tex_buf.get_mut().unwrap();
        buf.append_memory(mem);
    }

    // Pool of garbage collection storage
    let fence_data_pool = gst_d3d12::D3D12FenceDataPool::new();

    let fence = device.fence_handle(D3D12_COMMAND_LIST_TYPE_DIRECT).unwrap();

    CmdResources {
        device: device.clone(),
        cl,
        ca_pool,
        source_rtv_heap,
        source_tex,
        source_tex_buf,
        fence_data_pool,
        fence,
        fence_val: 0,
    }
}

fn create_pipeline() -> Result<gst::Pipeline, Error> {
    let device = gst_d3d12::D3D12Device::new(0).unwrap();

    let mut resources = create_cmd_resources(&device);

    let in_info = gst_video::VideoInfo::builder(gst_video::VideoFormat::Rgba, WIDTH, HEIGHT)
        .fps(gst::Fraction::new(2, 1))
        .build()
        .unwrap();
    let out_info = gst_video::VideoInfo::builder(gst_video::VideoFormat::Nv12, WIDTH, HEIGHT)
        .fps(gst::Fraction::new(2, 1))
        .build()
        .unwrap();
    let mut out_caps = out_info.to_caps().unwrap();
    {
        let features = gst::CapsFeatures::new(["memory:D3D12Memory"]);
        let out_caps = out_caps.get_mut().unwrap();
        out_caps.set_features_simple(Some(features));
    }

    // setup D3D12Convert's destination buffer pool
    let pool = gst_d3d12::D3D12BufferPool::new(&device);
    {
        let mut config = pool.config();
        config.set_params(Some(&out_caps), out_info.size() as u32, 0, 0);

        let d3d12_params = gst_d3d12::D3D12AllocationParams::new(
            &device,
            &out_info,
            gst_d3d12::D3D12AllocationFlags::D3d12AllocationFlagDefault,
            // GstD3D12 requires simultaneous-access enabled texture
            D3D12_RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS |
            // Enable RTV
            D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET,
            D3D12_HEAP_FLAG_NONE,
        )
        .unwrap();

        config.set_d3d12_allocation_params(&d3d12_params);
        pool.set_config(config).unwrap();
        pool.set_active(true).unwrap();
    }

    // Create converter
    let null_cmd_queue: Option<gst_d3d12::D3D12CmdQueue> = None;
    let conv = gst_d3d12::D3D12Converter::new(
        &device,
        null_cmd_queue.as_ref(),
        &in_info,
        &out_info,
        None, // Conversion without blending
        None,
        None,
    )
    .unwrap();

    // Create pipeline
    let pipeline = gst::Pipeline::default();

    let appsrc = gst_app::AppSrc::builder()
        .caps(&out_caps)
        .format(gst::Format::Time)
        .build();

    let sink = gst::ElementFactory::make("d3d12videosink").build().unwrap();

    pipeline.add_many([appsrc.upcast_ref(), &sink]).unwrap();
    appsrc.link(&sink).unwrap();

    let mut i = 0;
    appsrc.set_callbacks(
        gst_app::AppSrcCallbacks::builder()
            .need_data(move |appsrc, _| {
                // We only produce 100 frames
                if i == 100 {
                    let _ = appsrc.end_of_stream();
                    return;
                }

                println!("Producing frame {i}");

                // Calculate color to clear
                let r = if i % 2 == 0 { 0f32 } else { 1f32 };
                let g = if i % 3 == 0 { 0f32 } else { 255f32 };
                let b = if i % 5 == 0 { 0f32 } else { 255f32 };
                let a = 1f32;

                // Initialize command list
                let gst_ca = resources.ca_pool.acquire().unwrap();
                let ca = gst_ca.handle();

                let cl = resources.cl.clone();
                unsafe {
                    ca.Reset().unwrap();
                    cl.Reset(&ca, None).unwrap();
                }

                // Clear source rtv
                unsafe {
                    let rtv_handle = resources
                        .source_rtv_heap
                        .GetCPUDescriptorHandleForHeapStart();

                    // COMMON -> render target state transition is not needed, since
                    // this resource is allocated with SIMULTANEOUS_ACCESS flag

                    cl.ClearRenderTargetView(rtv_handle, &[r, g, b, a], None);

                    // But needs RTV -> SRV state transition.
                    // D3D12Converter expects SIMULTANEOUS_ACCESS enabled + COMMON
                    // or shader resource state
                    let barrier = D3D12_RESOURCE_BARRIER {
                        Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
                        Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
                        Anonymous: D3D12_RESOURCE_BARRIER_0 {
                            Transition: std::mem::ManuallyDrop::new(
                                D3D12_RESOURCE_TRANSITION_BARRIER {
                                    pResource: { std::mem::transmute_copy(&resources.source_tex) },
                                    StateBefore: D3D12_RESOURCE_STATE_RENDER_TARGET,
                                    StateAfter: D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE,
                                    Subresource: D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
                                },
                            ),
                        },
                    };

                    cl.ResourceBarrier(&[barrier]);
                }

                let fence_data = resources.fence_data_pool.acquire().unwrap();
                let mut buffer = pool.acquire_buffer(None).unwrap();

                // Records command
                conv.convert_buffer(&resources.source_tex_buf, &buffer, &fence_data, &cl, false)
                    .unwrap();

                unsafe {
                    cl.Close().unwrap();
                }

                let cl = cl.cast::<ID3D12CommandList>().unwrap();

                // Submit recorded commands
                let fence_val = resources
                    .device
                    .execute_command_lists(D3D12_COMMAND_LIST_TYPE_DIRECT, &[Some(cl)])
                    .unwrap();

                resources.fence_val = fence_val;

                // Capture resources that should not be freed until completed.
                // Captured resources here will be freed on GstD3D12's background
                // GC thread
                fence_data.push(move || {
                    println!("Destroying {:?}", gst_ca);
                    drop(gst_ca);
                });

                {
                    let buffer = buffer.get_mut().unwrap();
                    buffer.set_pts(i * 500 * gst::ClockTime::MSECOND);

                    let mem = buffer.peek_memory_mut(0).unwrap();
                    let dmem = mem.downcast_memory_mut::<gst_d3d12::D3D12Memory>().unwrap();

                    // Set fence and fence value to memory
                    dmem.set_fence(Some(&resources.fence), fence_val, false);

                    // Map writable to mark GPU -> CPU download is needed if any
                    let map = dmem.map_readable_d3d12().unwrap();
                    drop(map);
                }

                // Schedule GC
                resources
                    .device
                    .set_fence_notify(D3D12_COMMAND_LIST_TYPE_DIRECT, fence_val, move || {
                        println!("Destroying {:?}", fence_data);
                        drop(fence_data);
                    })
                    .unwrap();

                i += 1;

                // appsrc already handles the error here
                let _ = appsrc.push_buffer(buffer);
            })
            .build(),
    );

    Ok(pipeline)
}

fn main_loop(pipeline: gst::Pipeline) -> Result<(), Error> {
    pipeline.set_state(gst::State::Playing)?;

    let bus = pipeline
        .bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                pipeline.set_state(gst::State::Null)?;
                return Err(ErrorMessage {
                    src: msg
                        .src()
                        .map(|s| s.path_string())
                        .unwrap_or_else(|| glib::GString::from("UNKNOWN")),
                    error: err.error(),
                    debug: err.debug(),
                }
                .into());
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null)?;

    Ok(())
}

fn main() {
    gst::init().unwrap();

    match create_pipeline().and_then(main_loop) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {e}"),
    }
}
