mod additional_geometry;
mod geometry;
mod pathbuilder;
mod tessellate;
mod types;
mod vertex;

// Matches LyonInformationType in clyon.h.
const INFO_BUILD_TIME: u32 = 0;

// clyon's own version, packed as (major << 24) | (minor << 16) | patch.
#[no_mangle]
pub extern fn LyonVersion() -> u32 {
    let part = |s: &str| s.parse::<u32>().unwrap_or(0);

    (part(env!("CARGO_PKG_VERSION_MAJOR")) << 24)
        | (part(env!("CARGO_PKG_VERSION_MINOR")) << 16)
        | part(env!("CARGO_PKG_VERSION_PATCH"))
}

#[no_mangle]
pub extern fn LyonInfo(info: u32, output: *mut *mut i8) {
    if output.is_null() {
        return;
    }

    let value = match info {
        INFO_BUILD_TIME => Some(env!("CLYON_BUILD_TIME")),
        _ => None,
    };

    unsafe {
        *output = match value {
            Some(v) => std::ffi::CString::new(v).unwrap_or_default().into_raw(),
            None => std::ptr::null_mut(),
        };
    }
}

#[no_mangle]
pub extern fn LyonFreeString(input: *mut i8) {
    if !input.is_null() {
        drop(unsafe { std::ffi::CString::from_raw(input) });
    }
}
