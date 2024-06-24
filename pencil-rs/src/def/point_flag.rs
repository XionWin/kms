bitflags! {
    #[derive(Debug, Default, Clone, Copy)]
    pub struct PointFlags: u32 {
        const PT_CORNER = 0x1;
        const PT_LEFT = 0x2;
        const PT_BEVEL = 0x4;
        const PR_INNERBEVEL	= 0x8;
    }
}