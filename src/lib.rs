mod additional_geometry;
mod geometry;
mod pathbuilder;
mod tessellate;
mod types;
mod vertex;

#[no_mangle]
pub extern "C" fn LyonVersion() -> u32 {
    return (1 << 24) | (0 << 16) | (1);
}
