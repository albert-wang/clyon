#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vertex
{
    pub position: [f32; 2],
    pub uv: [f32; 4],
    pub color: u32,
    pub custom: [f32; 3]
}