use lyon::path::math::{point, vector};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct LyonVector {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct LyonPoint {
    pub x: f32,
    pub y: f32,
}

impl From<LyonPoint> for lyon::math::Point {
    fn from(p: LyonPoint) -> lyon::math::Point {
        return point(p.x, p.y);
    }
}

impl From<LyonVector> for lyon::math::Vector {
    fn from(v: LyonVector) -> lyon::math::Vector {
        return vector(v.x, v.y);
    }
}

use lyon::path::builder::WithSvg;
use lyon::path::path::BuilderImpl;

pub type InternalBuilder = WithSvg<BuilderImpl>;
