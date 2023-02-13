use lyon::geom::*;
use lyon::path::builder::SvgPathBuilder;
use lyon::path::math::{vector, Angle};
use lyon::path::Path;

use crate::additional_geometry;
use crate::types::{InternalBuilder, LyonPoint, LyonVector};

// Path stuff
#[no_mangle]
pub extern "C" fn LyonCreatePathBuilder() -> *mut InternalBuilder {
    let builder = Path::builder();
    let svg = builder.with_svg();
    Box::into_raw(Box::new(svg))
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_MoveTo(p: *mut InternalBuilder, v: LyonPoint) {
    assert!(!p.is_null());

    let builder = unsafe { &mut (*p) };
    builder.move_to(v.into());
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_LineTo(p: *mut InternalBuilder, v: LyonPoint) {
    assert!(!p.is_null());

    let builder = unsafe { &mut (*p) };
    builder.line_to(v.into());
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_QuadraticBeizerTo(
    p: *mut InternalBuilder,
    c: LyonPoint,
    v: LyonPoint,
) {
    assert!(!p.is_null());

    let builder = unsafe { &mut (*p) };
    builder.quadratic_bezier_to(c.into(), v.into());
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_SmoothQuadraticBeizerTo(p: *mut InternalBuilder, v: LyonPoint) {
    assert!(!p.is_null());

    let builder = unsafe { &mut (*p) };
    builder.smooth_quadratic_bezier_to(v.into());
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_CubicBeizerTo(
    p: *mut InternalBuilder,
    c: LyonPoint,
    c2: LyonPoint,
    v: LyonPoint,
) {
    assert!(!p.is_null());

    let builder = unsafe { &mut (*p) };
    builder.cubic_bezier_to(c.into(), c2.into(), v.into());
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_SmoothCubicBeizerTo(
    p: *mut InternalBuilder,
    c2: LyonPoint,
    v: LyonPoint,
) {
    assert!(!p.is_null());
    let builder = unsafe { &mut (*p) };

    builder.smooth_cubic_bezier_to(c2.into(), v.into());
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_Arc(
    p: *mut InternalBuilder,
    center: LyonPoint,
    radius_x: f32,
    radius_y: f32,
    sweep_angle: f32,
    x_rotation: f32,
) {
    assert!(!p.is_null());

    let builder = unsafe { &mut (*p) };
    builder.arc(
        center.into(),
        vector(radius_x, radius_y),
        Angle::radians(sweep_angle),
        Angle::radians(x_rotation),
    );
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_ArcTo(
    p: *mut InternalBuilder,
    to: LyonPoint,
    radius_x: f32,
    radius_y: f32,
    rotation: f32,
    large_arc: i32,
    sweep: i32,
) {
    assert!(!p.is_null());

    let flags = ArcFlags {
        large_arc: match large_arc {
            0 => false,
            1 => true,
            _ => false,
        },
        sweep: match sweep {
            0 => false,
            1 => true,
            _ => false,
        },
    };

    let builder = unsafe { &mut (*p) };
    builder.arc_to(
        vector(radius_x, radius_y),
        Angle::radians(rotation),
        flags,
        to.into(),
    );
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_HorizontalLineTo(p: *mut InternalBuilder, x: f32) {
    assert!(!p.is_null());
    let builder = unsafe { &mut (*p) };

    builder.horizontal_line_to(x);
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_VerticalLineTo(p: *mut InternalBuilder, y: f32) {
    assert!(!p.is_null());
    let builder = unsafe { &mut (*p) };

    builder.vertical_line_to(y);
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_RelativeMoveTo(p: *mut InternalBuilder, to: LyonVector) {
    assert!(!p.is_null());
    let builder = unsafe { &mut (*p) };

    builder.relative_move_to(to.into());
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_RelativeLineTo(p: *mut InternalBuilder, to: LyonVector) {
    assert!(!p.is_null());
    let builder = unsafe { &mut (*p) };

    builder.relative_line_to(to.into());
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_RelativeQuadraticBeizerTo(
    p: *mut InternalBuilder,
    ctrl: LyonVector,
    to: LyonVector,
) {
    assert!(!p.is_null());
    let builder = unsafe { &mut (*p) };

    builder.relative_quadratic_bezier_to(ctrl.into(), to.into());
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_RelativeSmoothQuadraticBeizerTo(
    p: *mut InternalBuilder,
    v: LyonVector,
) {
    assert!(!p.is_null());

    let builder = unsafe { &mut (*p) };
    builder.smooth_relative_quadratic_bezier_to(v.into());
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_RelativeCubicBeizerTo(
    p: *mut InternalBuilder,
    ctrl: LyonVector,
    ctrl2: LyonVector,
    to: LyonVector,
) {
    assert!(!p.is_null());
    let builder = unsafe { &mut (*p) };

    builder.relative_cubic_bezier_to(ctrl.into(), ctrl2.into(), to.into());
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_RelativeSmoothCubicBeizerTo(
    p: *mut InternalBuilder,
    c2: LyonVector,
    v: LyonVector,
) {
    assert!(!p.is_null());
    let builder = unsafe { &mut (*p) };

    builder.smooth_relative_cubic_bezier_to(c2.into(), v.into());
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_RelativeArcTo(
    p: *mut InternalBuilder,
    to: LyonVector,
    r_x: f32,
    r_y: f32,
    x_rotation: f32,
    large_arc: i32,
    sweep: i32,
) {
    assert!(!p.is_null());
    let builder = unsafe { &mut (*p) };

    let flags = ArcFlags {
        large_arc: match large_arc {
            0 => false,
            1 => true,
            _ => false,
        },
        sweep: match sweep {
            0 => false,
            1 => true,
            _ => false,
        },
    };

    builder.relative_arc_to(
        vector(r_x, r_y),
        Angle::radians(x_rotation),
        flags,
        to.into(),
    );
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_Reserve(p: *mut InternalBuilder, endpoints: u64, control: u64) {
    assert!(!p.is_null());
    let builder = unsafe { &mut (*p) };

    builder.reserve(endpoints as usize, control as usize);
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_Close(p: *mut InternalBuilder) {
    assert!(!p.is_null());
    let builder = unsafe { &mut (*p) };
    builder.close();
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_Build(p: *mut InternalBuilder) -> *mut Path {
    assert!(!p.is_null());

    let builder = unsafe { Box::from_raw(p) };
    let path = (*builder).build();

    Box::into_raw(Box::new(path))
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_GetCurrentPosition(p: *mut InternalBuilder) -> LyonPoint {
    assert!(!p.is_null());
    let builder = unsafe { &mut (*p) };

    let p = builder.current_position();
    return LyonPoint {
        x: p.x, 
        y: p.y
    };
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_AddRect(p: *mut InternalBuilder, min: LyonPoint, max: LyonPoint) {
    additional_geometry::add_rectangle(p, min, max)
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_AddCircle(p: *mut InternalBuilder, center: LyonPoint, radius: f32) {
    additional_geometry::add_circle(p, center, radius);
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_AddEllipse(p: *mut InternalBuilder, center: LyonPoint, r_x: f32, r_y: f32, x_rotation: f32) {
    additional_geometry::add_ellipse(p, center, r_x, r_y, x_rotation);
}

#[no_mangle]
pub extern "C" fn LyonPathBuilder_AddRoundedRect(p: *mut InternalBuilder,  min: LyonPoint, max: LyonPoint, border_radius: f32) {
    additional_geometry::add_rounded_rectangle(p, min, max, border_radius);
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct LyonRect {
    pub lower_left: [f32; 2],
    pub upper_right: [f32; 2],
}

#[no_mangle]
pub extern "C" fn LyonPathBoundingRect(p: *mut Path) -> LyonRect {
    let path = unsafe { &mut (*p) };
    let rect = lyon::algorithms::aabb::bounding_box(path.iter());

    LyonRect {
        lower_left: rect.min.to_array(),
        upper_right: (rect.max).to_array(),
    }
}

#[no_mangle]
pub extern "C" fn LyonFreePath(p: *mut Path) {
    unsafe { Box::from_raw(p) };
}
