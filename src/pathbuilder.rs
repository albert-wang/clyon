extern crate lyon;

use lyon_path::path::BuilderWithAttributes;
use lyon::path::math::{ point, vector };
use lyon::path::Path;
use lyon::geom::*;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct InputVertex
{
    pub position: [f32; 2],
}

pub struct Builder
{
    builder: BuilderWithAttributes,
    current_position: lyon::math::Point,
    starting_position: lyon::math::Point,
}

// Path stuff
#[no_mangle]
pub extern "C" fn LyonCreatePathBuilder() -> *mut Builder {
    let bwa = Path::builder_with_attributes(2);
    let builder = Builder{
        builder: bwa, 
        current_position: point(0.0, 0.0),
        starting_position: point(0.0, 0.0),
    };

    Box::into_raw(Box::new(builder))
}

fn input_vertex_to_attrs(_: InputVertex) -> [f32; 2] {
    [0.0, 0.0]
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_Begin(p: *mut Builder, v: InputVertex) {
    assert!(!p.is_null());

    let builder = unsafe { &mut (*p) };
    let c = point(v.position[0], v.position[1]);

    builder.builder.begin(c, &input_vertex_to_attrs(v));
    builder.current_position = c;
    builder.starting_position = c;
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_End(p: *mut Builder, c: bool) {
    assert!(!p.is_null());

    let builder = unsafe { &mut (*p) };
    builder.builder.end(c);
    if c {
        builder.current_position = builder.starting_position;
    }
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_LineTo(p: *mut Builder, v: InputVertex) {
    assert!(!p.is_null());

    let builder = unsafe { &mut (*p) };
    let c = point(v.position[0], v.position[1]);

    builder.builder.line_to(c, &input_vertex_to_attrs(v));
    builder.current_position = c;
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_QuadraticBeizerTo(
    p: *mut Builder,
    cx: f32, cy: f32,
    v: InputVertex
) {
    assert!(!p.is_null());

    let builder = unsafe { &mut (*p) };
    let c = point(v.position[0], v.position[1]);
    
    builder.builder.quadratic_bezier_to(point(cx,cy), c, &input_vertex_to_attrs(v));
    builder.current_position = c;
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_Arc(
    p: *mut Builder,
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
            builder.builder.line_to(seg.from, &input_vertex_to_attrs(center));
            first = false;
        }

        builder.builder.cubic_bezier_to(seg.ctrl1, seg.ctrl2, seg.to, &input_vertex_to_attrs(center));
        builder.current_position = seg.to;
    })
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_ArcTo(
    p: *mut Builder,
    to: InputVertex,
    radius_x: f32, radius_y: f32,
    rotation: f32,
    large_arc: i32, sweep: i32
) {
    assert!(!p.is_null());

    let builder = unsafe { &mut (*p) };

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

    let t = point(to.position[0], to.position[1]);

    let arc = SvgArc {
        from: builder.current_position,
        to: t,
        radii: vector(radius_x, radius_y),
        x_rotation: Angle::radians(rotation),
        flags: flags
    };

    arc.for_each_cubic_bezier(&mut |seg: &CubicBezierSegment<f32>| {
        builder.builder.cubic_bezier_to(seg.ctrl1, seg.ctrl2, seg.to, &input_vertex_to_attrs(to));
    });

    builder.current_position = t;
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_CubicBeizerTo(
    p: *mut Builder,
    cx: f32, cy: f32,
    c2x: f32, c2y: f32,
    v: InputVertex
) {
    assert!(!p.is_null());

    let builder = unsafe { &mut (*p) };
    let t = point(v.position[0], v.position[1]);

    builder.builder.cubic_bezier_to(point(cx,cy), point(c2x, c2y), t, &input_vertex_to_attrs(v));
    builder.current_position = t;
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_Build(p: *mut Builder) -> *mut Path {
    assert!(!p.is_null());

    let builder = unsafe { Box::from_raw(p) };
    let path = builder.builder.build();

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
    let rect = lyon::algorithms::aabb::bounding_rect(path.iter());

    LyonRect {
        lower_left: rect.origin.to_array(),
        upper_right: (rect.origin + rect.size).to_array()
    }
}

#[no_mangle]
pub extern "C" fn LyonFreePath(p: *mut Path) {
    unsafe { Box::from_raw(p) };
}

