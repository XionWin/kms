#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum SubPixel {
    Unknown = 1,
    HorizontalRgb = 2,
    HorizontalBgr = 3,
    VerticalRgb = 4,
    VerticalBgr = 5,
    None = 6,
}