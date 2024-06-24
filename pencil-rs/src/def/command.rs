use super::{Point, Solidity};

#[derive(Debug)]
pub enum Command {
    MoveTo(Point),
    LineTo(Point),
    BezierTo(Point, Point, Point),
    Close,
    Solidity(Solidity),
}