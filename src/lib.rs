#![cfg(any(target_os = "android", target_arch = "wasm32"))]

pub mod ui {
    slint::include_modules!();
}

mod app_main;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(android_app: slint::android::AndroidApp) -> Result<(), slint::PlatformError> {
    slint::android::init(android_app).expect("Slint initialization failed");

    app_main::run()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    app_main::run().expect("Runtime error occured");
}
