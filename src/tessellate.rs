extern crate lyon;

use crate::vertex::Vertex;

use lyon::path::Path;
use lyon::path::math::Point;
use lyon::tessellation::*;
use std::ops::Add;

#[repr(C)]
pub struct CFillOptions {
    tolerance: f32, 
    fillRule: i32, 
    orientation: i32,
}

#[repr(C)]
pub struct CStrokeOptions {
    startCap: i32,
    endCap: i32, 
    join: i32, 
    width: f32, 
    applyWidth: i32,
}

// Converts a float array in rgba format to u32 in RRGGBBAA format
fn to_color32(c: &[f32]) -> u32 {
    let r = (c[0] * 255.0) as u32 & 0xFF;
    let g = (c[1] * 255.0) as u32 & 0xFF;
    let b = (c[2] * 255.0) as u32 & 0xFF;
    let a = (c[3] * 255.0) as u32 & 0xFF;

    (r << 24) | (g << 16) | (b << 8) | a
}

fn tesselate_fill<IndexType: Add + From<VertexId> + geometry_builder::MaxIndex>(p: *mut Path, copts: CFillOptions) -> *mut VertexBuffers<Vertex, IndexType> {
    assert!(!p.is_null());

    let path = unsafe { &*p };
    let mut tesselator = FillTessellator::new();

    let mut opts = FillOptions::default();
    if copts.tolerance > 0.0 {
        opts.tolerance = copts.tolerance
    }

    if copts.fillRule != 0 {
        opts.fill_rule = FillRule::NonZero
    }

    if copts.orientation != 0 {
        opts.sweep_orientation = Orientation::Horizontal
    }

    let mut geometry: VertexBuffers<Vertex, IndexType> = VertexBuffers::new();
    tesselator.tessellate_path(
        path, 
        &opts, 
        &mut BuffersBuilder::new(&mut geometry, |p: Point, mut fa: FillAttributes| {
            let attrs = fa.interpolated_attributes();
            Vertex{
                position: p.to_array(),
                uv: [attrs[0], attrs[1], 0.0, 0.0], 
                color: to_color32(&attrs[2..6]),
                custom: [attrs[6], attrs[7], attrs[8]],
            }
        })
    ).unwrap();

    Box::into_raw(Box::new(geometry))
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
        _ => LineJoin::Miter
    }
}

fn tesselate_stroke<IndexType: Add + From<VertexId> + geometry_builder::MaxIndex>(p: *mut Path, copts: CStrokeOptions) -> *mut VertexBuffers<Vertex, IndexType> {
    if p.is_null() {
        panic!("Null pointer passed into TessellateStroke")
    }

    let path = unsafe { &*p };
    let mut tesselator = StrokeTessellator::new();

    let mut opts = StrokeOptions::default();
    opts.start_cap = cap_from_integer(copts.startCap);
    opts.end_cap = cap_from_integer(copts.endCap);
    opts.line_join = join_from_integer(copts.join);
    opts.line_width = copts.width;
    opts.apply_line_width = copts.applyWidth != 0;

    let mut geometry: VertexBuffers<Vertex, IndexType> = VertexBuffers::new();
    tesselator.tessellate_path(
        path, 
        &StrokeOptions::default(), 
        &mut BuffersBuilder::new(&mut geometry, |p: Point, mut sa: StrokeAttributes| {
            let normal = { sa.normal() };
            let attrs = sa.interpolated_attributes();

            Vertex{
                position: p.to_array(),
                uv: [attrs[0], attrs[1], normal.x, normal.y], 
                color: to_color32(&attrs[2..6]),
                custom: [attrs[6], attrs[7], attrs[8]],
            }
        })
    ).unwrap();

    Box::into_raw(Box::new(geometry))
}
#[no_mangle]
pub extern "C" fn TessellateFill16(p: *mut Path, copts: CFillOptions) -> *mut VertexBuffers<Vertex, u16> {
    tesselate_fill(p, copts)
}

#[no_mangle]
pub extern "C" fn TessellateFill32(p: *mut Path, copts: CFillOptions) -> *mut VertexBuffers<Vertex, u32> {
    tesselate_fill(p, copts)
}

#[no_mangle]
pub extern "C" fn TessellateStroke16(p: *mut Path, copts: CStrokeOptions) -> *mut VertexBuffers<Vertex, u16> {
    tesselate_stroke(p, copts)
}

#[no_mangle]
pub extern "C" fn TessellateStroke32(p: *mut Path, copts: CStrokeOptions) -> *mut VertexBuffers<Vertex, u32> {
    tesselate_stroke(p, copts)
}

#[no_mangle]
pub extern "C" fn FreeGeometry16(p: *mut VertexBuffers<Vertex, u16>) {
    unsafe { Box::from_raw(p) };
}

#[no_mangle]
pub extern "C" fn FreeGeometry32(p: *mut VertexBuffers<Vertex, u32>) {
    unsafe { Box::from_raw(p) };
}