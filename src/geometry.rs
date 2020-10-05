extern crate lyon;
use lyon::tessellation::*;

use crate::vertex::Vertex;

#[no_mangle]
pub extern "C" fn Geometry_VerticesLength16(p: *mut VertexBuffers<Vertex, u16>) -> u32 {
    unsafe {
        (*p).vertices.len() as u32
    }
}

#[no_mangle]
pub extern "C" fn Geometry_IndicesLength16(p: *mut VertexBuffers<Vertex, u16>) -> u32 {
    unsafe {
        (*p).indices.len() as u32
    }
}

#[no_mangle]
pub extern "C" fn Geometry_VerticesData16(p: *mut VertexBuffers<Vertex, u16>) -> *const f32 {
    unsafe {
        (*p).vertices.as_ptr() as *const f32
    }
}

#[no_mangle]
pub extern "C" fn Geometry_IndicesData16(p: *mut VertexBuffers<Vertex, u16>) -> *const u16 {
    unsafe {
        (*p).indices.as_ptr()
    }
}

#[no_mangle]
pub extern "C" fn Geometry_VerticesLength32(p: *mut VertexBuffers<Vertex, u32>) -> u32 {
    unsafe {
        (*p).vertices.len() as u32
    }
}

#[no_mangle]
pub extern "C" fn Geometry_IndicesLength32(p: *mut VertexBuffers<Vertex, u32>) -> u32 {
    unsafe {
        (*p).indices.len() as u32
    }
}

#[no_mangle]
pub extern "C" fn Geometry_VerticesData32(p: *mut VertexBuffers<Vertex, u32>) -> *const f32 {
    unsafe {
        (*p).vertices.as_ptr() as *const f32
    }
}

#[no_mangle]
pub extern "C" fn Geometry_IndicesData32(p: *mut VertexBuffers<Vertex, u32>) -> *const u32 {
    unsafe {
        (*p).indices.as_ptr()
    }
}