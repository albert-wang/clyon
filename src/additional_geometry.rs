// These are reimplementations of functions found in lyon_path/builder.rs
use std::f32::consts::PI;

use lyon::{
    geom::{Arc, QuadraticBezierSegment},
    math::Angle,
    math::Point,
    math::{point, vector},
    path::{traits::SvgPathBuilder, Polygon},
};

use crate::types::{InternalBuilder, LyonPoint};

pub fn add_rectangle(p: *mut InternalBuilder, min: LyonPoint, max: LyonPoint) {
    assert!(!p.is_null());

    let rect = Polygon {
        points: &[
            min.into(),
            point(max.x, min.y),
            max.into(),
            point(min.x, max.y),
        ],
        closed: true,
    };

    let builder = unsafe { &mut (*p) };
    builder.add_polygon(rect);
}

pub fn add_ellipse(p: *mut InternalBuilder, center: LyonPoint, r_x: f32, r_y: f32, x_rotation: f32) {
    let arc = Arc {
        center: center.into(),
        radii: vector(r_x, r_y),
        x_rotation: Angle::radians(x_rotation),
        start_angle: Angle::radians(0.0),
        sweep_angle: Angle::radians(2.0 * PI),
    };

    assert!(!p.is_null());
    let builder = unsafe { &mut (*p) };

    builder.move_to(arc.sample(0.0));

    arc.for_each_quadratic_bezier(&mut |curve: &QuadraticBezierSegment<f32>| {
        builder.quadratic_bezier_to(curve.ctrl, curve.to);
    });

    builder.close()
}

pub fn add_circle(p: *mut InternalBuilder, c: LyonPoint, radius: f32) {
    let radius = radius.abs();
    let dir = 1.0;
    let center: Point = c.into();

    let builder = unsafe { &mut (*p) };

    // https://spencermortensen.com/articles/bezier-circle/
    const CONSTANT_FACTOR: f32 = 0.55191505;
    let d = radius * CONSTANT_FACTOR;

    builder.move_to(center + vector(-radius, 0.0));

    let ctrl_0 = center + vector(-radius, -d * dir);
    let ctrl_1 = center + vector(-d, -radius * dir);
    let mid = center + vector(0.0, -radius * dir);
    builder.cubic_bezier_to(ctrl_0, ctrl_1, mid);

    let ctrl_0 = center + vector(d, -radius * dir);
    let ctrl_1 = center + vector(radius, -d * dir);
    let mid = center + vector(radius, 0.0);
    builder.cubic_bezier_to(ctrl_0, ctrl_1, mid);

    let ctrl_0 = center + vector(radius, d * dir);
    let ctrl_1 = center + vector(d, radius * dir);
    let mid = center + vector(0.0, radius * dir);
    builder.cubic_bezier_to(ctrl_0, ctrl_1, mid);

    let ctrl_0 = center + vector(-d, radius * dir);
    let ctrl_1 = center + vector(-radius, d * dir);
    let mid = center + vector(-radius, 0.0);
    builder.cubic_bezier_to(ctrl_0, ctrl_1, mid);

    builder.close();
}

pub fn add_rounded_rectangle(
    p: *mut InternalBuilder,
    min: LyonPoint,
    max: LyonPoint,
    border_radius: f32,
) {
    let w = max.x - min.x;
    let h = max.y - min.y;

    assert!(w >= 0.0 && h >= 0.0);
    let x_min = min.x;
    let y_min = min.y;
    let x_max = max.x;
    let y_max = max.y;

    let min_wh = w.min(h);
    let mut tl = border_radius.abs().min(min_wh);
    let mut tr = border_radius.abs().min(min_wh);
    let mut bl = border_radius.abs().min(min_wh);
    let mut br = border_radius.abs().min(min_wh);

    // clamp border radii if they don't fit in the rectangle.
    if tl + tr > w {
        let x = (tl + tr - w) * 0.5;
        tl -= x;
        tr -= x;
    }
    if bl + br > w {
        let x = (bl + br - w) * 0.5;
        bl -= x;
        br -= x;
    }
    if tr + br > h {
        let x = (tr + br - h) * 0.5;
        tr -= x;
        br -= x;
    }
    if tl + bl > h {
        let x = (tl + bl - h) * 0.5;
        tl -= x;
        bl -= x;
    }

    // https://spencermortensen.com/articles/bezier-circle/
    const CONSTANT_FACTOR: f32 = 0.55191505;

    let tl_d = tl * CONSTANT_FACTOR;
    let tl_corner = point(x_min, y_min);

    let tr_d = tr * CONSTANT_FACTOR;
    let tr_corner = point(x_max, y_min);

    let br_d = br * CONSTANT_FACTOR;
    let br_corner = point(x_max, y_max);

    let bl_d = bl * CONSTANT_FACTOR;
    let bl_corner = point(x_min, y_max);

    let points = [
        point(x_min, y_min + tl),           // begin
        tl_corner + vector(0.0, tl - tl_d), // control
        tl_corner + vector(tl - tl_d, 0.0), // control
        tl_corner + vector(tl, 0.0),        // end
        point(x_max - tr, y_min),
        tr_corner + vector(-tr + tr_d, 0.0),
        tr_corner + vector(0.0, tr - tr_d),
        tr_corner + vector(0.0, tr),
        point(x_max, y_max - br),
        br_corner + vector(0.0, -br + br_d),
        br_corner + vector(-br + br_d, 0.0),
        br_corner + vector(-br, 0.0),
        point(x_min + bl, y_max),
        bl_corner + vector(bl - bl_d, 0.0),
        bl_corner + vector(0.0, -bl + bl_d),
        bl_corner + vector(0.0, -bl),
    ];

    assert!(!p.is_null());
    let builder = unsafe { &mut (*p) };

    builder.move_to(points[0]);
    if tl > 0.0 {
        builder.cubic_bezier_to(points[1], points[2], points[3]);
    }
    builder.line_to(points[4]);
    if tl > 0.0 {
        builder.cubic_bezier_to(points[5], points[6], points[7]);
    }
    builder.line_to(points[8]);
    if br > 0.0 {
        builder.cubic_bezier_to(points[9], points[10], points[11]);
    }
    builder.line_to(points[12]);
    if bl > 0.0 {
        builder.cubic_bezier_to(points[13], points[14], points[15]);
    }

    builder.close()
}
