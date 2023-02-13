use lyon::tessellation::*;

use crate::vertex::Vertex;

#[no_mangle]
pub extern "C" fn LyonGeometry16_VerticesLength(p: *mut VertexBuffers<Vertex, u16>) -> u32 {
    unsafe { (*p).vertices.len() as u32 }
}

#[no_mangle]
pub extern "C" fn LyonGeometry16_IndicesLength(p: *mut VertexBuffers<Vertex, u16>) -> u32 {
    unsafe { (*p).indices.len() as u32 }
}

#[no_mangle]
pub extern "C" fn LyonGeometry16_VerticesData(p: *mut VertexBuffers<Vertex, u16>) -> *const f32 {
    unsafe { (*p).vertices.as_ptr() as *const f32 }
}

#[no_mangle]
pub extern "C" fn LyonGeometry16_IndicesData(p: *mut VertexBuffers<Vertex, u16>) -> *const u16 {
    unsafe { (*p).indices.as_ptr() }
}

#[no_mangle]
pub extern "C" fn LyonGeometry32_VerticesLength(p: *mut VertexBuffers<Vertex, u32>) -> u32 {
    unsafe { (*p).vertices.len() as u32 }
}

#[no_mangle]
pub extern "C" fn LyonGeometry32_IndicesLength(p: *mut VertexBuffers<Vertex, u32>) -> u32 {
    unsafe { (*p).indices.len() as u32 }
}

#[no_mangle]
pub extern "C" fn LyonGeometry32_VerticesData(p: *mut VertexBuffers<Vertex, u32>) -> *const f32 {
    unsafe { (*p).vertices.as_ptr() as *const f32 }
}

#[no_mangle]
pub extern "C" fn LyonGeometry32_IndicesData(p: *mut VertexBuffers<Vertex, u32>) -> *const u32 {
    unsafe { (*p).indices.as_ptr() }
}
