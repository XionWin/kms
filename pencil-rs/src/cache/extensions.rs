use crate::def::VPoint;

pub fn poly_area(pts: &[VPoint]) -> f32 {
    let mut area = 0.0;
    for i in 2..pts.len() {
        let a = &pts[0];
        let b = &pts[i - 1];
        let c = &pts[i];
        area += triangle_area(a, b, c);
    }
    area * 0.5
}

pub fn triangle_area(a: &VPoint, b: &VPoint, c: &VPoint) -> f32 {
    let a = &a.location;
    let b = &b.location;
    let c = &c.location;
    let abx = b.x - a.x;
    let aby = b.y - a.y;
    let acx = c.x - a.x;
    let acy = c.y - a.y;
    acx * aby - abx * acy
}

pub fn poly_reverse(pts: &mut [VPoint]) {
    let mut i = 0;
    let mut j = pts.len() as i32 - 1;
    while i < j {
        pts.swap(i as usize, j as usize);
        i += 1;
        j -= 1;
    }
}