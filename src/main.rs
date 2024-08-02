#![cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]

pub mod ui {
    slint::include_modules!();
}

mod app_main;

fn main() -> Result<(), slint::PlatformError> {
    app_main::run()
}
