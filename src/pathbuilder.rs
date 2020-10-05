extern crate lyon;

use lyon::path::BuilderWithAttributes;
use lyon::path::math::point;
use lyon::path::Path;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct InputVertex 
{
    pub position: [f32; 2],
    pub uv: [f32; 2],
    pub color: [f32; 4],
    pub custom: [f32; 3],
}

// Path stuff
#[no_mangle]
pub extern "C" fn CreatePathBuilder() -> *mut BuilderWithAttributes {
    let builder = Path::builder_with_attributes(9);
    Box::into_raw(Box::new(builder))
}

fn input_vertex_to_attrs(v: InputVertex) -> [f32; 9] {
    [
        v.uv[0], v.uv[1],
        v.color[0], v.color[1], v.color[2], v.color[3], 
        v.custom[0], v.custom[1], v.custom[2]
    ]
}

#[no_mangle]
pub extern "C" fn PathBuilder_MoveTo(p: *mut BuilderWithAttributes, v: InputVertex) {
    assert!(!p.is_null());
    unsafe { (*p).move_to(point(v.position[0],v.position[1]), &input_vertex_to_attrs(v)) };
}

#[no_mangle]
pub extern "C" fn PathBuilder_LineTo(p: *mut BuilderWithAttributes, v: InputVertex) {
    assert!(!p.is_null());
    unsafe { (*p).line_to(point(v.position[0],v.position[1]), &input_vertex_to_attrs(v)) };
}

#[no_mangle]
pub extern "C" fn PathBuilder_QuadraticBeizerTo(
    p: *mut BuilderWithAttributes, 
    cx: f32, cy: f32,
    v: InputVertex
) {
    assert!(!p.is_null());
    unsafe { (*p).quadratic_bezier_to(point(cx,cy), point(v.position[0],v.position[1]), &input_vertex_to_attrs(v)) };
}

#[no_mangle]
pub extern "C" fn PathBuilder_CubicBeizerTo(
    p: *mut BuilderWithAttributes, 
    cx: f32, cy: f32,
    c2x: f32, c2y: f32,
    v: InputVertex
) {
    assert!(!p.is_null());
    unsafe { (*p).cubic_bezier_to(point(cx,cy), point(c2x, c2y), point(v.position[0],v.position[1]), &input_vertex_to_attrs(v)) };
}

#[no_mangle]
pub extern "C" fn PathBuilder_Build(p: *mut BuilderWithAttributes) -> *mut Path {
    assert!(!p.is_null());

    let builder = unsafe { Box::from_raw(p) };
    let path = builder.build();

    Box::into_raw(Box::new(path))
}

#[no_mangle]
pub extern "C" fn FreePath(p: *mut Path) {
    unsafe { Box::from_raw(p) };
}

