use crate::vertex::{Vertex, PRIMITIVE_TYPE_FILLED, PRIMITIVE_TYPE_STROKED};

use lyon::path::Path;
use lyon::tessellation::*;
use std::ops::Add;

#[repr(C)]
pub struct CFillOptions {
    pub tolerance: f32,
    pub fill_rule: i32,
    pub orientation: i32,

    pub color: u32,
    pub fill_ind: i32,
    pub shape_ind: i32,
}

#[repr(C)]
pub struct CStrokeOptions {
    pub start_cap: i32,
    pub end_cap: i32,
    pub join: i32,
    pub width: f32,

    pub color: u32,
    pub fill_ind: i32,
    pub shape_ind: i32,

    pub tolerance: f32,
}

fn tesselate_fill<IndexType: Add + From<VertexId> + geometry_builder::MaxIndex>(
    p: *mut Path,
    copts: CFillOptions,
    output_err: *mut *const i8
) -> *mut VertexBuffers<Vertex, IndexType> {
    assert!(!p.is_null());

    let path = unsafe { &*p };
    let mut tesselator = FillTessellator::new();

    let mut opts = FillOptions::default();
    if copts.tolerance > 0.0 {
        opts.tolerance = copts.tolerance
    }

    if copts.fill_rule != 0 {
        opts.fill_rule = FillRule::NonZero
    }

    if copts.orientation != 0 {
        opts.sweep_orientation = Orientation::Horizontal
    }

    let mut geometry: VertexBuffers<Vertex, IndexType> = VertexBuffers::new();
    let result = tesselator
        .tessellate_path(
            path,
            &opts,
            &mut BuffersBuilder::new(&mut geometry, |v: FillVertex| {
                let p = v.position();

                Vertex {
                    position: [p.x, p.y],
                    original_position: [p.x, p.y],
                    normal: [0.0, 0.0],
                    color: copts.color,
                    primitive_type: PRIMITIVE_TYPE_FILLED,
                    fill_ind: copts.fill_ind,
                    shape_ind: copts.shape_ind,
                }
            }),
        );

    match result {
        Ok(_) => {
            // Happy path, returns the pointer to the generated geometry.
            return Box::into_raw(Box::new(geometry));
        }, 
        Err(TessellationError::GeometryBuilder(err)) => {            
            if err == GeometryBuilderError::TooManyVertices {
                // Too many vertices. This should be caught and handled somewhere else.
                return std::ptr::null_mut();
            } else {
                let err_str = std::ffi::CString::new(err.to_string()).unwrap();
                unsafe { *output_err  = err_str.into_raw() };

                return std::ptr::null_mut();
            }
        }, 
        Err(other) => {
            let err_str = std::ffi::CString::new(other.to_string()).unwrap();
            unsafe { *output_err  = err_str.into_raw() };

            return std::ptr::null_mut();
        }
    };
}

fn cap_from_integer(i: i32) -> LineCap {
    match i {
        0 => LineCap::Butt,
        1 => LineCap::Round,
        2 => LineCap::Square,
        _ => LineCap::Butt,
    }
}

fn join_from_integer(i: i32) -> LineJoin {
    match i {
        0 => LineJoin::Miter,
        1 => LineJoin::MiterClip,
        2 => LineJoin::Bevel,
        3 => LineJoin::Round,
        _ => LineJoin::Miter,
    }
}

fn tesselate_stroke<IndexType: Add + From<VertexId> + geometry_builder::MaxIndex>(
    p: *mut Path,
    copts: CStrokeOptions,
    output_err: *mut *const i8
) -> *mut VertexBuffers<Vertex, IndexType> {
    if p.is_null() {
        panic!("Null pointer path passed into TessellateStroke")
    }

    if output_err.is_null() {
        panic!("Null pointer err passed into TesselateStroke")
    }

    unsafe { *output_err = std::ptr::null_mut() };

    let path = unsafe { &*p };
    let mut tesselator = StrokeTessellator::new();

    let mut opts = StrokeOptions::default();
    opts.start_cap = cap_from_integer(copts.start_cap);
    opts.end_cap = cap_from_integer(copts.end_cap);
    opts.line_join = join_from_integer(copts.join);
    opts.line_width = copts.width;
    opts.tolerance = copts.tolerance;

    let mut geometry: VertexBuffers<Vertex, IndexType> = VertexBuffers::new();
    let result = tesselator
        .tessellate_path(
            path,
            &opts,
            &mut BuffersBuilder::new(&mut geometry, |v: StrokeVertex| {
                let normal = v.normal();
                let p = v.position();

                Vertex {
                    position: [p.x, p.y],
                    original_position: [p.x, p.y],
                    normal: [normal.x, normal.y],
                    color: copts.color,
                    primitive_type: PRIMITIVE_TYPE_STROKED,
                    fill_ind: copts.fill_ind,
                    shape_ind: copts.shape_ind,
                }
            }),
        );

    match result {
        Ok(_) => {
            // Happy path, returns the pointer to the generated geometry.
            return Box::into_raw(Box::new(geometry));
        }, 
        Err(TessellationError::GeometryBuilder(err)) => {            
            if err == GeometryBuilderError::TooManyVertices {
                // Too many vertices. This should be caught and handled somewhere else.
                return std::ptr::null_mut();
            } else {
                let err_str = std::ffi::CString::new(err.to_string()).unwrap();
                unsafe { *output_err  = err_str.into_raw() };

                return std::ptr::null_mut();
            }
        }, 
        Err(other) => {
            let err_str = std::ffi::CString::new(other.to_string()).unwrap();
            unsafe { *output_err  = err_str.into_raw() };

            return std::ptr::null_mut();
        }
    };
}

#[no_mangle]
pub extern fn LyonTessellateFill16(
    p: *mut Path,
    copts: CFillOptions,
    output_err: *mut *const i8
) -> *mut VertexBuffers<Vertex, u16> {
    tesselate_fill(p, copts, output_err)
}

#[no_mangle]
pub extern fn LyonTessellateFill32(
    p: *mut Path,
    copts: CFillOptions,
    output_err: *mut *const i8
) -> *mut VertexBuffers<Vertex, u32> {
    tesselate_fill(p, copts, output_err)
}

#[no_mangle]
pub extern fn LyonTessellateStroke16(
    p: *mut Path,
    copts: CStrokeOptions,
    output_err: *mut *const i8
) -> *mut VertexBuffers<Vertex, u16> {
    tesselate_stroke(p, copts, output_err)
}

#[no_mangle]
pub extern fn LyonTessellateStroke32(
    p: *mut Path,
    copts: CStrokeOptions,
    output_err: *mut *const i8
) -> *mut VertexBuffers<Vertex, u32> {
    tesselate_stroke(p, copts, output_err)
}

#[no_mangle]
pub extern fn LyonFreeGeometry16(p: *mut VertexBuffers<Vertex, u16>) {
    unsafe { Box::from_raw(p) };
}

#[no_mangle]
pub extern fn LyonFreeGeometry32(p: *mut VertexBuffers<Vertex, u32>) {
    unsafe { Box::from_raw(p) };
}
