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

fn clear_error(output_err: *mut *mut i8) {
    if !output_err.is_null() {
        unsafe { *output_err = std::ptr::null_mut() };
    }
}

// Running out of 16-bit indices returns null without a message; the caller is expected
// to treat that as fatal. Other errors are written to output_err.
fn into_output<IndexType>(
    result: Result<(), TessellationError>,
    geometry: VertexBuffers<Vertex, IndexType>,
    output_err: *mut *mut i8,
) -> *mut VertexBuffers<Vertex, IndexType> {
    match result {
        Ok(_) => Box::into_raw(Box::new(geometry)),
        Err(TessellationError::GeometryBuilder(GeometryBuilderError::TooManyVertices)) => {
            std::ptr::null_mut()
        }
        Err(err) => {
            if !output_err.is_null() {
                let err_str = std::ffi::CString::new(err.to_string()).unwrap_or_default();
                unsafe { *output_err = err_str.into_raw() };
            }

            std::ptr::null_mut()
        }
    }
}

fn tesselate_fill<IndexType: Add + From<VertexId> + geometry_builder::MaxIndex>(
    p: *mut Path,
    copts: CFillOptions,
    output_err: *mut *mut i8,
) -> *mut VertexBuffers<Vertex, IndexType> {
    assert!(!p.is_null());
    clear_error(output_err);

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

    into_output(result, geometry, output_err)
}

// Values match LyonLineCap in clyon.h.
fn cap_from_integer(i: i32) -> LineCap {
    match i {
        1 => LineCap::Square,
        2 => LineCap::Round,
        _ => LineCap::Butt,
    }
}

// Values match LyonLineJoin in clyon.h.
fn join_from_integer(i: i32) -> LineJoin {
    match i {
        1 => LineJoin::MiterClip,
        2 => LineJoin::Round,
        3 => LineJoin::Bevel,
        _ => LineJoin::Miter,
    }
}

fn tesselate_stroke<IndexType: Add + From<VertexId> + geometry_builder::MaxIndex>(
    p: *mut Path,
    copts: CStrokeOptions,
    output_err: *mut *mut i8,
) -> *mut VertexBuffers<Vertex, IndexType> {
    assert!(!p.is_null());
    clear_error(output_err);

    let path = unsafe { &*p };
    let mut tesselator = StrokeTessellator::new();

    let mut opts = StrokeOptions::default();
    opts.start_cap = cap_from_integer(copts.start_cap);
    opts.end_cap = cap_from_integer(copts.end_cap);
    opts.line_join = join_from_integer(copts.join);
    opts.line_width = copts.width;
    if copts.tolerance > 0.0 {
        opts.tolerance = copts.tolerance
    }

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

    into_output(result, geometry, output_err)
}

#[no_mangle]
pub extern fn LyonTessellateFill16(
    p: *mut Path,
    copts: CFillOptions,
    output_err: *mut *mut i8
) -> *mut VertexBuffers<Vertex, u16> {
    tesselate_fill(p, copts, output_err)
}

#[no_mangle]
pub extern fn LyonTessellateFill32(
    p: *mut Path,
    copts: CFillOptions,
    output_err: *mut *mut i8
) -> *mut VertexBuffers<Vertex, u32> {
    tesselate_fill(p, copts, output_err)
}

#[no_mangle]
pub extern fn LyonTessellateStroke16(
    p: *mut Path,
    copts: CStrokeOptions,
    output_err: *mut *mut i8
) -> *mut VertexBuffers<Vertex, u16> {
    tesselate_stroke(p, copts, output_err)
}

#[no_mangle]
pub extern fn LyonTessellateStroke32(
    p: *mut Path,
    copts: CStrokeOptions,
    output_err: *mut *mut i8
) -> *mut VertexBuffers<Vertex, u32> {
    tesselate_stroke(p, copts, output_err)
}

#[no_mangle]
pub extern fn LyonFreeGeometry16(p: *mut VertexBuffers<Vertex, u16>) {
    if !p.is_null() {
        drop(unsafe { Box::from_raw(p) });
    }
}

#[no_mangle]
pub extern fn LyonFreeGeometry32(p: *mut VertexBuffers<Vertex, u32>) {
    if !p.is_null() {
        drop(unsafe { Box::from_raw(p) });
    }
}
