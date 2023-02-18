mod additional_geometry;
mod geometry;
mod pathbuilder;
mod tessellate;
mod types;
mod vertex;

#[no_mangle]
pub extern fn LyonVersion() -> u32 {
    return (1 << 24) | (0 << 16) | (1);
}

#[no_mangle]
pub extern fn LyonInfo(info: u32, _: *mut *const i8) {
    match info {
        _ => {
            return;
        }
    }
}

#[no_mangle]
pub extern fn LyonFreeString(input_err: *mut i8) {
    if input_err.is_null() {
        return;
    }

    unsafe { std::ffi::CString::from_raw(input_err) };
}