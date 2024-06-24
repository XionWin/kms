#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EncoderType
{
    NONE = 0,
    DAC = 1,
    TMDS = 2,
    LVDS = 3,
    TVDAC = 4,
    VIRTUAL = 5,
    DSI = 6,
    DPMST = 7,
    DPI = 8,
}