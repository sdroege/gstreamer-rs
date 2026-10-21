// This example shows how to compile and run a HIP kernel using GstHip.
// It initializes an array on the CPU, uploads it to GPU memory, and runs
// a kernel that computes output[i] = input[i] * 2 + 1 on a dedicated stream.
// After waiting for completion, it reads the output back to the CPU and
// verifies the results.
//
// Requires the vendor runtime and runtime compiler (HIPRTC or NVRTC).

use anyhow::{anyhow, ensure, Context, Result};
use byte_slice_cast::*;
use gst_hip::prelude::*;

const COUNT: u32 = 16;
const BLOCK_SIZE: u32 = 64;

fn main() -> Result<()> {
    gst::init()?;

    // Create device 0 with automatic vendor selection
    let device = gst_hip::HipDevice::new(gst_hip::HipVendor::Unknown, 0)
        .ok_or_else(|| anyhow!("Couldn't create HIP device; check the vendor runtime"))?;

    let vendor = device.vendor();
    let device_id = device.device_id();

    println!("Using HIP device: vendor={vendor:?}, device_id={device_id}");

    // Create a dedicated stream for kernel execution
    let stream = gst_hip::HipStream::new(vendor, device_id)
        .ok_or_else(|| anyhow!("Couldn't create HIP stream"))?;

    // Compile the HIP kernel module from source.
    let module = gst_hip::HipKernelModule::from_source(
        &device,
        // A simple arithmetic kernel: output = input * 2 + 1
        r#"
        extern "C" __global__
        void
        transform(const unsigned int *input, unsigned int *output, unsigned int count)
        {
            unsigned int i = blockIdx.x * blockDim.x + threadIdx.x;
            if (i < count)
                output[i] = input[i] * 2u + 1u;
        }
        "#,
        &[],
    )?
    .ok_or_else(|| anyhow!("Couldn't compile HIP kernel; check HIPRTC/NVRTC"))?;

    let size = COUNT as usize * std::mem::size_of::<u32>();
    let allocator = gst_hip::HipAllocator::new();

    // Allocate input and output HIP memory
    let mut input = allocator
        .alloc_bytes(&device, size)
        .ok_or_else(|| anyhow!("Couldn't allocate HIP input memory"))?
        .downcast_memory::<gst_hip::HipMemory>()
        .map_err(|_| anyhow!("Expected HIP input memory"))?;

    let mut output = allocator
        .alloc_bytes(&device, size)
        .ok_or_else(|| anyhow!("Couldn't allocate HIP output memory"))?
        .downcast_memory::<gst_hip::HipMemory>()
        .map_err(|_| anyhow!("Expected HIP output memory"))?;
    {
        // Map the input memory for CPU writing and initialize it with values 0..COUNT as u32
        let mut map = input.get_mut().unwrap().map_writable()?;
        for (i, value) in map.as_mut_slice_of::<u32>()?.iter_mut().enumerate() {
            *value = i as u32;
        }
    }

    // Upload the CPU-written input before passing its device pointer to the kernel
    {
        let _map = input.map_readable_hip()?;
    }

    // Mark the output as GPU-written so a subsequent CPU map downloads it.
    {
        let _map = output.get_mut().unwrap().map_writable_hip()?;
    }

    let mut args = gst_hip::HipKernelArgs::new();
    // SAFETY: Arguments match transform(), and both allocations and the module
    // remain alive until the stream has completed, including launch failures
    let launch_result = unsafe {
        args.add_memory(&input, 0)?;
        args.add_memory(&output, 0)?;
        args.add(COUNT);
        module.launch(
            "transform",
            Some(&stream),
            [COUNT.div_ceil(BLOCK_SIZE), 1, 1],
            [BLOCK_SIZE, 1, 1],
            &args,
        )
    };
    ensure!(stream.synchronize(), "Couldn't synchronize HIP stream");
    launch_result.context("Couldn't launch transform kernel")?;

    {
        // Map the output memory for CPU reading and verify the results.
        // On CPU map, GstHipMemory will download the GPU device memory
        // into its own host-accessible memory.
        let map = output.map_readable()?;
        let values = map.as_slice_of::<u32>()?;
        for (i, &value) in values.iter().enumerate() {
            let expected = i as u32 * 2 + 1;
            ensure!(
                value == expected,
                "Result {i}: expected {expected}, got {value}"
            );
        }

        println!("GPU output: {values:?}");
        println!("All {COUNT} results match");
    }

    Ok(())
}
