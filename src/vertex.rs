#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vertex
{
    pub position: [f32; 2],
    pub original_position: [f32; 2],
    pub normal: [f32; 2],
    pub color: u32,

    // 0 - text, 1 - filled shape, 2 - stroked line,
    pub primitive_type: f32,

    // Index into a fill array
    pub fill_ind: f32,

    // Index of the shape
    pub shape_ind: f32,
}