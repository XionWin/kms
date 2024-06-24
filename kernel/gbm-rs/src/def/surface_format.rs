iterable_enum! {
    #[repr(u32)]
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy)]
    pub enum SurfaceFormat {
        BIG_ENDIAN = 1u32 << 31,
        INVALID = 0u32,
        C8 = (b'C' as u32) | (b'8' as u32) << 8 | (b' ' as u32) << 16 | (b' ' as u32) << 24,
        R8 = (b'R' as u32) | (b'8' as u32) << 8 | (b' ' as u32) << 16 | (b' ' as u32) << 24,
        R16 = (b'R' as u32) | (b'1' as u32) << 8 | (b'6' as u32) << 16 | (b' ' as u32) << 24,
        RG88 = (b'R' as u32) | (b'G' as u32) << 8 | (b'8' as u32) << 16 | (b'8' as u32) << 24,
        GR88 = (b'G' as u32) | (b'R' as u32) << 8 | (b'8' as u32) << 16 | (b'8' as u32) << 24,
        RG1616 = (b'R' as u32) | (b'G' as u32) << 8 | (b'3' as u32) << 16 | (b'2' as u32) << 24,
        GR1616 = (b'G' as u32) | (b'R' as u32) << 8 | (b'3' as u32) << 16 | (b'2' as u32) << 24,
        RGB332 = (b'R' as u32) | (b'G' as u32) << 8 | (b'B' as u32) << 16 | (b'8' as u32) << 24,
        BGR233 = (b'B' as u32) | (b'G' as u32) << 8 | (b'R' as u32) << 16 | (b'8' as u32) << 24,
        XRGB4444 = (b'X' as u32) | (b'R' as u32) << 8 | (b'1' as u32) << 16 | (b'2' as u32) << 24,
        XBGR4444 = (b'X' as u32) | (b'B' as u32) << 8 | (b'1' as u32) << 16 | (b'2' as u32) << 24,
        RGBX4444 = (b'R' as u32) | (b'X' as u32) << 8 | (b'1' as u32) << 16 | (b'2' as u32) << 24,
        BGRX4444 = (b'B' as u32) | (b'X' as u32) << 8 | (b'1' as u32) << 16 | (b'2' as u32) << 24,
    
        ARGB4444 = (b'A' as u32) | (b'R' as u32) << 8 | (b'1' as u32) << 16 | (b'2' as u32) << 24,
        ABGR4444 = (b'A' as u32) | (b'B' as u32) << 8 | (b'1' as u32) << 16 | (b'2' as u32) << 24,
        RGBA4444 = (b'R' as u32) | (b'A' as u32) << 8 | (b'1' as u32) << 16 | (b'2' as u32) << 24,
        BGRA4444 = (b'B' as u32) | (b'A' as u32) << 8 | (b'1' as u32) << 16 | (b'2' as u32) << 24,
    
        XRGB1555 = (b'X' as u32) | (b'R' as u32) << 8 | (b'1' as u32) << 16 | (b'5' as u32) << 24,
        XBGR1555 = (b'X' as u32) | (b'B' as u32) << 8 | (b'1' as u32) << 16 | (b'5' as u32) << 24,
        RGBX5551 = (b'R' as u32) | (b'X' as u32) << 8 | (b'1' as u32) << 16 | (b'5' as u32) << 24,
        BGRX5551 = (b'B' as u32) | (b'X' as u32) << 8 | (b'1' as u32) << 16 | (b'5' as u32) << 24,
    
        ARGB1555 = (b'A' as u32) | (b'R' as u32) << 8 | (b'1' as u32) << 16 | (b'5' as u32) << 24,
        ABGR1555 = (b'A' as u32) | (b'B' as u32) << 8 | (b'1' as u32) << 16 | (b'5' as u32) << 24,
        RGBA5551 = (b'R' as u32) | (b'A' as u32) << 8 | (b'1' as u32) << 16 | (b'5' as u32) << 24,
        BGRA5551 = (b'B' as u32) | (b'A' as u32) << 8 | (b'1' as u32) << 16 | (b'5' as u32) << 24,
    
        RGB565 = (b'R' as u32) | (b'G' as u32) << 8 | (b'1' as u32) << 16 | (b'6' as u32) << 24,
        BGR565 = (b'B' as u32) | (b'G' as u32) << 8 | (b'1' as u32) << 16 | (b'6' as u32) << 24,
        RGB888 = (b'R' as u32) | (b'G' as u32) << 8 | (b'2' as u32) << 16 | (b'4' as u32) << 24,
        BGR888 = (b'B' as u32) | (b'G' as u32) << 8 | (b'2' as u32) << 16 | (b'4' as u32) << 24,
        XRGB8888 = (b'X' as u32) | (b'R' as u32) << 8 | (b'2' as u32) << 16 | (b'4' as u32) << 24,
        XBGR8888 = (b'X' as u32) | (b'B' as u32) << 8 | (b'2' as u32) << 16 | (b'4' as u32) << 24,
        RGBX8888 = (b'R' as u32) | (b'X' as u32) << 8 | (b'2' as u32) << 16 | (b'4' as u32) << 24,
        BGRX8888 = (b'B' as u32) | (b'X' as u32) << 8 | (b'2' as u32) << 16 | (b'4' as u32) << 24,
    
        ARGB8888 = (b'A' as u32) | (b'R' as u32) << 8 | (b'2' as u32) << 16 | (b'4' as u32) << 24,
        ABGR8888 = (b'A' as u32) | (b'B' as u32) << 8 | (b'2' as u32) << 16 | (b'4' as u32) << 24,
        RGBA8888 = (b'R' as u32) | (b'A' as u32) << 8 | (b'2' as u32) << 16 | (b'4' as u32) << 24,
        BGRA8888 = (b'B' as u32) | (b'A' as u32) << 8 | (b'2' as u32) << 16 | (b'4' as u32) << 24,
    
        XRGB2101010 = (b'X' as u32) | (b'R' as u32) << 8 | (b'3' as u32) << 16 | (b'0' as u32) << 24,
        XBGR2101010 = (b'X' as u32) | (b'B' as u32) << 8 | (b'3' as u32) << 16 | (b'0' as u32) << 24,
        RGBX1010102 = (b'R' as u32) | (b'X' as u32) << 8 | (b'3' as u32) << 16 | (b'0' as u32) << 24,
        BGRX1010102 = (b'B' as u32) | (b'X' as u32) << 8 | (b'3' as u32) << 16 | (b'0' as u32) << 24,
    
        ARGB2101010 = (b'A' as u32) | (b'R' as u32) << 8 | (b'3' as u32) << 16 | (b'0' as u32) << 24,
        ABGR2101010 = (b'A' as u32) | (b'B' as u32) << 8 | (b'3' as u32) << 16 | (b'0' as u32) << 24,
        RGBA1010102 = (b'R' as u32) | (b'A' as u32) << 8 | (b'3' as u32) << 16 | (b'0' as u32) << 24,
        BGRA1010102 = (b'B' as u32) | (b'A' as u32) << 8 | (b'3' as u32) << 16 | (b'0' as u32) << 24,
        XRGB16161616F = (b'X' as u32) | (b'R' as u32) << 8 | (b'4' as u32) << 16 | (b'H' as u32) << 24,
        XBGR16161616F = (b'X' as u32) | (b'B' as u32) << 8 | (b'4' as u32) << 16 | (b'H' as u32) << 24,
    
        ARGB16161616F = (b'A' as u32) | (b'R' as u32) << 8 | (b'4' as u32) << 16 | (b'H' as u32) << 24,
        ABGR16161616F = (b'A' as u32) | (b'B' as u32) << 8 | (b'4' as u32) << 16 | (b'H' as u32) << 24,
        YUYV = (b'Y' as u32) | (b'U' as u32) << 8 | (b'Y' as u32) << 16 | (b'V' as u32) << 24,
        YVYU = (b'Y' as u32) | (b'V' as u32) << 8 | (b'Y' as u32) << 16 | (b'U' as u32) << 24,
        UYVY = (b'U' as u32) | (b'Y' as u32) << 8 | (b'V' as u32) << 16 | (b'Y' as u32) << 24,
        VYUY = (b'V' as u32) | (b'Y' as u32) << 8 | (b'U' as u32) << 16 | (b'Y' as u32) << 24,
    
        AYUV = (b'A' as u32) | (b'Y' as u32) << 8 | (b'U' as u32) << 16 | (b'V' as u32) << 24,
        XYUV8888 = (b'X' as u32) | (b'Y' as u32) << 8 | (b'U' as u32) << 16 | (b'V' as u32) << 24,
        VUY888 = (b'V' as u32) | (b'U' as u32) << 8 | (b'2' as u32) << 16 | (b'4' as u32) << 24,
        VUY101010 = (b'V' as u32) | (b'U' as u32) << 8 | (b'3' as u32) << 16 | (b'0' as u32) << 24,
        Y210 = (b'Y' as u32) | (b'2' as u32) << 8 | (b'1' as u32) << 16 | (b'0' as u32) << 24,
        Y212 = (b'Y' as u32) | (b'2' as u32) << 8 | (b'1' as u32) << 16 | (b'2' as u32) << 24,
        Y216 = (b'Y' as u32) | (b'2' as u32) << 8 | (b'1' as u32) << 16 | (b'6' as u32) << 24,
        Y410 = (b'Y' as u32) | (b'4' as u32) << 8 | (b'1' as u32) << 16 | (b'0' as u32) << 24,
        Y412 = (b'Y' as u32) | (b'4' as u32) << 8 | (b'1' as u32) << 16 | (b'2' as u32) << 24,
        Y416 = (b'Y' as u32) | (b'4' as u32) << 8 | (b'1' as u32) << 16 | (b'6' as u32) << 24,
    
        XVYU2101010 = (b'X' as u32) | (b'V' as u32) << 8 | (b'3' as u32) << 16 | (b'0' as u32) << 24,
        XVYU12_16161616 = (b'X' as u32) | (b'V' as u32) << 8 | (b'3' as u32) << 16 | (b'6' as u32) << 24,
        XVYU16161616 = (b'X' as u32) | (b'V' as u32) << 8 | (b'4' as u32) << 16 | (b'8' as u32) << 24,
    
        Y0L0 = (b'Y' as u32) | (b'0' as u32) << 8 | (b'L' as u32) << 16 | (b'0' as u32) << 24,
    
        X0L0 = (b'X' as u32) | (b'0' as u32) << 8 | (b'L' as u32) << 16 | (b'0' as u32) << 24,
        Y0L2 = (b'Y' as u32) | (b'0' as u32) << 8 | (b'L' as u32) << 16 | (b'2' as u32) << 24,
    
        X0L2 = (b'X' as u32) | (b'0' as u32) << 8 | (b'L' as u32) << 16 | (b'2' as u32) << 24,
        YUV420_8BIT = (b'Y' as u32) | (b'U' as u32) << 8 | (b'0' as u32) << 16 | (b'8' as u32) << 24,
        YUV420_10BIT = (b'Y' as u32) | (b'U' as u32) << 8 | (b'1' as u32) << 16 | (b'0' as u32) << 24,
        XRGB8888_A8 = (b'X' as u32) | (b'R' as u32) << 8 | (b'A' as u32) << 16 | (b'8' as u32) << 24,
        XBGR8888_A8 = (b'X' as u32) | (b'B' as u32) << 8 | (b'A' as u32) << 16 | (b'8' as u32) << 24,
        RGBX8888_A8 = (b'R' as u32) | (b'X' as u32) << 8 | (b'A' as u32) << 16 | (b'8' as u32) << 24,
        BGRX8888_A8 = (b'B' as u32) | (b'X' as u32) << 8 | (b'A' as u32) << 16 | (b'8' as u32) << 24,
        RGB888_A8 = (b'R' as u32) | (b'8' as u32) << 8 | (b'A' as u32) << 16 | (b'8' as u32) << 24,
        BGR888_A8 = (b'B' as u32) | (b'8' as u32) << 8 | (b'A' as u32) << 16 | (b'8' as u32) << 24,
        RGB565_A8 = (b'R' as u32) | (b'5' as u32) << 8 | (b'A' as u32) << 16 | (b'8' as u32) << 24,
        BGR565_A8 = (b'B' as u32) | (b'5' as u32) << 8 | (b'A' as u32) << 16 | (b'8' as u32) << 24,
        NV12 = (b'N' as u32) | (b'V' as u32) << 8 | (b'1' as u32) << 16 | (b'2' as u32) << 24,
        NV21 = (b'N' as u32) | (b'V' as u32) << 8 | (b'2' as u32) << 16 | (b'1' as u32) << 24,
        NV16 = (b'N' as u32) | (b'V' as u32) << 8 | (b'1' as u32) << 16 | (b'6' as u32) << 24,
        NV61 = (b'N' as u32) | (b'V' as u32) << 8 | (b'6' as u32) << 16 | (b'1' as u32) << 24,
        NV24 = (b'N' as u32) | (b'V' as u32) << 8 | (b'2' as u32) << 16 | (b'4' as u32) << 24,
        NV42 = (b'N' as u32) | (b'V' as u32) << 8 | (b'4' as u32) << 16 | (b'2' as u32) << 24
    }
}
