use crate::def::{Point, PointFlags};

#[derive(Debug, Default, Clone, Copy)]
#[allow(dead_code)]
pub(crate) struct VPoint {
    pub(crate) location: Point,
    pub(crate) d: Point,
    pub(crate) len: f32,
    pub(crate) dm: Point,
    pub(crate) flags: PointFlags,
}