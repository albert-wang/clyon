pub const PRIMITIVE_TYPE_FILLED: u32 = 1;
pub const PRIMITIVE_TYPE_STROKED: u32 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 2],
    pub original_position: [f32; 2],
    pub normal: [f32; 2],
    pub color: u32,

    // 0 - text, 1 - filled shape, 2 - stroked line,
    pub primitive_type: u32,

    // Index into a fill array
    pub fill_ind: i32,

    // Index of the shape
    pub shape_ind: i32,
}
