#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ConnectorType {
    Unknown = 0,
    VGA = 1,
    DVII = 2,
    DVID = 3,
    DVIA = 4,
    Composite = 5,
    SVIDEO = 6,
    LVDS = 7,
    Component = 8,
    PinDIN9 = 9,
    DisplayPort = 10,
    HDMIA = 11,
    HDMIB = 12,
    TV = 13,
    EDP = 14,
    VIRTUAL = 15,
    DSI = 16,
    DPI = 17,
}