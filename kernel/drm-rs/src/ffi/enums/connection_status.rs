#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ConnectionStatus {
    Connected = 1,
    Disconnected = 2,
    Unknown = 3,
}