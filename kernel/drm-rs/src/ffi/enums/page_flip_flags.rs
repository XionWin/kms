bitflags! {
    #[repr(C)]
    pub struct PageFlipFlags: u32 {
        const FLIP_EVENT = 0x01;
        const FLIP_ASYNC = 0x02;
        const FLIP_FLAGS = 0x01 | 0x02;
    }
}