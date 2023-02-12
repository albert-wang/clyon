extern crate lyon;

use lyon::path::builder::{SvgPathBuilder, WithSvg};
use lyon::path::math::{ point, vector, Angle };
use lyon::path::Path;
use lyon::path::path::BuilderImpl;
use lyon::path::builder::Flattened;
use lyon::geom::*;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct InputVertex
{
    pub position: [f32; 2],
}

type ExposedFlatBuilder = WithSvg<Flattened<BuilderImpl>>;

// Path stuff
#[no_mangle]
pub extern "C" fn LyonCreatePathBuilder() -> *mut ExposedFlatBuilder {
    let builder = Path::builder().flattened(0.01);
    let svg = builder.with_svg();
    Box::into_raw(Box::new(svg))
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_MoveTo(p: *mut ExposedFlatBuilder, v: InputVertex) {
    assert!(!p.is_null());

    let builder = unsafe { &mut (*p) };
    builder.move_to(point(v.position[0], v.position[1]));
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_LineTo(p: *mut ExposedFlatBuilder, v: InputVertex) {
    assert!(!p.is_null());

    let builder = unsafe { &mut (*p) };
    builder.line_to(point(v.position[0], v.position[1]));
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_QuadraticBeizerTo(
    p: *mut ExposedFlatBuilder,
    cx: f32, cy: f32,
    v: InputVertex
) {
    assert!(!p.is_null());

    let builder = unsafe { &mut (*p) };
    builder.quadratic_bezier_to(point(cx,cy), point(v.position[0],v.position[1]));
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_Arc(
    p: *mut ExposedFlatBuilder,
    center: InputVertex,
    radius_x: f32, radius_y: f32,
    start_angle: f32,
    sweep_angle: f32,
    x_rotation: f32
) {
    assert!(!p.is_null());

    let builder = unsafe { &mut (*p) };
    let arc = Arc {
        center: point(center.position[0], center.position[1]),
        radii: vector(radius_x, radius_y),
        start_angle: Angle::radians(start_angle),
        sweep_angle: Angle::radians(sweep_angle),
        x_rotation: Angle::radians(x_rotation)
    };

    let mut first = true;
    arc.for_each_cubic_bezier(&mut |seg: &CubicBezierSegment<f32> | {
        if first
        {
            builder.move_to(seg.from);
            first = false;
        }

        builder.cubic_bezier_to(seg.ctrl1, seg.ctrl2, seg.to);
    })
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_ArcTo(
    p: *mut ExposedFlatBuilder,
    to: InputVertex,
    radius_x: f32, radius_y: f32,
    rotation: f32,
    large_arc: i32, sweep: i32
) {
    assert!(!p.is_null());

    let flags = ArcFlags {
        large_arc: match large_arc {
            0 => false,
            1 => true,
            _ => false
        },
        sweep: match sweep {
            0 => false,
            1 => true,
            _ => false
        }
    };

    let builder = unsafe { &mut (*p) };
    builder.arc_to(
        vector(radius_x, radius_y), 
        Angle::radians(rotation), 
        flags, 
        point(to.position[0], to.position[1])
    );
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_CubicBeizerTo(
    p: *mut ExposedFlatBuilder,
    cx: f32, cy: f32,
    c2x: f32, c2y: f32,
    v: InputVertex
) {
    assert!(!p.is_null());

    let builder = unsafe { &mut (*p) };
    builder.cubic_bezier_to(point(cx,cy), point(c2x, c2y), point(v.position[0],v.position[1]));
}


#[no_mangle]
pub extern "C" fn LyonPathBuilder_End(p: *mut ExposedFlatBuilder, close: bool) {
    assert!(!p.is_null());

    let builder = unsafe { &mut (*p) };
    
    if close {
        builder.close();
    }
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_Build(p: *mut ExposedFlatBuilder) -> *mut Path {
    assert!(!p.is_null());

    let builder = unsafe { Box::from_raw(p) };
    let path = (*builder).build();

    Box::into_raw(Box::new(path))
}


#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct LyonRect
{
    pub lower_left: [f32; 2],
    pub upper_right: [f32; 2]
}

#[no_mangle]
pub extern "C" fn LyonPathBoundingRect(p: *mut Path) -> LyonRect {
    let path = unsafe { &mut (*p) };
    let rect = lyon::algorithms::aabb::bounding_box(path.iter());

    LyonRect {
        lower_left: rect.min.to_array(),
        upper_right: (rect.max).to_array()
    }
}

#[no_mangle]
pub extern "C" fn LyonFreePath(p: *mut Path) {
    unsafe { Box::from_raw(p) };
}

