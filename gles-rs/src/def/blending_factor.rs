pub enum BlendingFactor {
    //
    // Summary:
    //     [requires: EXT_draw_buffers_indexed, or NV_blend_equation_advanced, OES_draw_buffers_indexed]
    //     Original was GL_ZERO = 0
    Zero = 0,
    //
    // Summary:
    //     [requires: EXT_draw_buffers_indexed, or OES_draw_buffers_indexed] Original was
    //     GL_SRC_COLOR = 0x0300
    SrcColor = 768,
    //
    // Summary:
    //     [requires: EXT_draw_buffers_indexed, or OES_draw_buffers_indexed] Original was
    //     GL_ONE_MINUS_SRC_COLOR = 0x0301
    OneMinusSrcColor = 769,
    //
    // Summary:
    //     [requires: EXT_draw_buffers_indexed, or OES_draw_buffers_indexed] Original was
    //     GL_SRC_ALPHA = 0x0302
    SrcAlpha = 770,
    //
    // Summary:
    //     [requires: EXT_draw_buffers_indexed, or OES_draw_buffers_indexed] Original was
    //     GL_ONE_MINUS_SRC_ALPHA = 0x0303
    OneMinusSrcAlpha = 771,
    //
    // Summary:
    //     [requires: EXT_draw_buffers_indexed, or OES_draw_buffers_indexed] Original was
    //     GL_DST_ALPHA = 0x0304
    DstAlpha = 772,
    //
    // Summary:
    //     [requires: EXT_draw_buffers_indexed, or OES_draw_buffers_indexed] Original was
    //     GL_ONE_MINUS_DST_ALPHA = 0x0305
    OneMinusDstAlpha = 773,
    //
    // Summary:
    //     [requires: EXT_draw_buffers_indexed, or OES_draw_buffers_indexed] Original was
    //     GL_DST_COLOR = 0x0306
    DstColor = 774,
    //
    // Summary:
    //     [requires: EXT_draw_buffers_indexed, or OES_draw_buffers_indexed] Original was
    //     GL_ONE_MINUS_DST_COLOR = 0x0307
    OneMinusDstColor = 775,
    //
    // Summary:
    //     [requires: EXT_draw_buffers_indexed, or OES_draw_buffers_indexed] Original was
    //     GL_SRC_ALPHA_SATURATE = 0x0308
    SrcAlphaSaturate = 776,
    //
    // Summary:
    //     [requires: EXT_draw_buffers_indexed, or OES_draw_buffers_indexed] Original was
    //     GL_CONSTANT_COLOR = 0x8001
    ConstantColor = 32769,
    //
    // Summary:
    //     [requires: EXT_draw_buffers_indexed, or OES_draw_buffers_indexed] Original was
    //     GL_ONE_MINUS_CONSTANT_COLOR = 0x8002
    OneMinusConstantColor = 32770,
    //
    // Summary:
    //     [requires: EXT_draw_buffers_indexed, or OES_draw_buffers_indexed] Original was
    //     GL_CONSTANT_ALPHA = 0x8003
    ConstantAlpha = 32771,
    //
    // Summary:
    //     [requires: EXT_draw_buffers_indexed, or OES_draw_buffers_indexed] Original was
    //     GL_ONE_MINUS_CONSTANT_ALPHA = 0x8004
    OneMinusConstantAlpha = 32772,
    //
    // Summary:
    //     Original was GL_SRC1_ALPHA = 0x8589
    Src1Alpha = 34185,
    //
    // Summary:
    //     Original was GL_SRC1_COLOR = 0x88F9
    Src1Color = 35065,
    //
    // Summary:
    //     [requires: EXT_draw_buffers_indexed, or OES_draw_buffers_indexed] Original was
    //     GL_ONE = 1
    One = 1
}