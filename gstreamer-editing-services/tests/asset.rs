// Take a look at the license at the top of the repository in the LICENSE file.

use gstreamer_editing_services as ges;
use gstreamer_editing_services::prelude::*;

fn init() {
    use std::sync::Once;
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        gst::init().unwrap();
        ges::init().unwrap();
    });
}

#[test]
fn test_asset_request_generic() {
    init();

    let asset = ges::Asset::request::<ges::TestClip>(None).unwrap();

    assert_eq!(asset.extractable_type(), ges::TestClip::static_type());
    let clip = asset.extract().unwrap();
    assert!(clip.is::<ges::TestClip>());

    // Mark TestClip as needing reload
    assert!(ges::Asset::needs_reload::<ges::TestClip>(None));
}

#[test]
fn test_asset_request_async() {
    init();

    let main_context = glib::MainContext::default();
    let main_loop = glib::MainLoop::new(Some(&main_context), false);

    // Test requesting an asset asynchronously
    ges::Asset::request_async::<ges::TestClip, _>(
        None,
        gio::Cancellable::NONE,
        glib::clone!(
            #[strong]
            main_loop,
            move |result| {
                match result {
                    Ok(asset) => {
                        assert_eq!(asset.extractable_type(), ges::TestClip::static_type());
                        let clip = asset.extract().unwrap();
                        assert!(clip.is::<ges::TestClip>());
                    }
                    Err(err) => {
                        panic!("Failed to load asset: {}", err);
                    }
                }
                main_loop.quit();
            }
        ),
    );

    main_loop.run();
}

#[cfg(feature = "v1_30")]
#[test]
fn test_asset_request_full_untrusted_bin_description() {
    init();

    let asset = ges::Asset::request_full::<ges::Effect>(
        Some("volume"),
        ges::AssetRequestFlags::NO_UNTRUSTED_BIN_DESCRIPTION,
    )
    .unwrap()
    .unwrap();
    assert_eq!(asset.extractable_type(), ges::Effect::static_type());

    let err = ges::Asset::request_full::<ges::Effect>(
        Some("fdsink"),
        ges::AssetRequestFlags::NO_UNTRUSTED_BIN_DESCRIPTION,
    )
    .unwrap_err();
    assert_eq!(err.kind::<ges::Error>(), Some(ges::Error::UntrustedElement));
}

#[cfg(feature = "v1_30")]
#[test]
fn test_asset_request_full_future_untrusted_bin_description() {
    init();

    let main_context = glib::MainContext::new();
    main_context.block_on(async {
        let asset = ges::Asset::request_full_future::<ges::Effect>(
            Some("volume"),
            ges::AssetRequestFlags::NO_UNTRUSTED_BIN_DESCRIPTION,
        )
        .await
        .unwrap();
        assert_eq!(asset.extractable_type(), ges::Effect::static_type());

        let err = ges::Asset::request_full_future::<ges::Effect>(
            Some("fdsink"),
            ges::AssetRequestFlags::NO_UNTRUSTED_BIN_DESCRIPTION,
        )
        .await
        .unwrap_err();
        assert_eq!(err.kind::<ges::Error>(), Some(ges::Error::UntrustedElement));
    });
}
