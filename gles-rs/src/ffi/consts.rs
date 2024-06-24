use libc::{c_uint, c_uchar};

// Constants

/* BeginMode */
pub const GL_POINTS:         c_uint = 0x0000 as c_uint;
pub const GL_LINES:          c_uint = 0x0001 as c_uint;
pub const GL_LINE_LOOP:      c_uint = 0x0002 as c_uint;
pub const GL_LINE_STRIP:     c_uint = 0x0003 as c_uint;
pub const GL_TRIANGLES:      c_uint = 0x0004 as c_uint;
pub const GL_TRIANGLE_STRIP: c_uint = 0x0005 as c_uint;
pub const GL_TRIANGLE_FAN:   c_uint = 0x0006 as c_uint;

pub const GL_DEPTH_BUFFER_BIT:   c_uint = 0x00000100 as c_uint;
pub const GL_STENCIL_BUFFER_BIT: c_uint = 0x00000400 as c_uint;
pub const GL_COLOR_BUFFER_BIT:   c_uint = 0x00004000 as c_uint;

/* BlendingFactorDest */
pub const GL_ZERO:                     c_uint = 0      as c_uint;
pub const GL_ONE:                      c_uint = 1      as c_uint;
pub const GL_SRC_COLOR:                c_uint = 0x0300 as c_uint;
pub const GL_ONE_MINUS_SRC_COLOR:      c_uint = 0x0301 as c_uint;
pub const GL_SRC_ALPHA:                c_uint = 0x0302 as c_uint;
pub const GL_ONE_MINUS_SRC_ALPHA:      c_uint = 0x0303 as c_uint;
pub const GL_DST_ALPHA:                c_uint = 0x0304 as c_uint;
pub const GL_ONE_MINUS_DST_ALPHA:      c_uint = 0x0305 as c_uint;

/* BlendingFactorSrc */
pub const GL_DST_COLOR:                c_uint = 0x0306 as c_uint;
pub const GL_ONE_MINUS_DST_COLOR:      c_uint = 0x0307 as c_uint;
pub const GL_SRC_ALPHA_SATURATE:       c_uint = 0x0308 as c_uint;

/* Boolean */
pub const GL_TRUE:                     c_uchar = 1 as c_uchar;
pub const GL_FALSE:                    c_uchar = 0 as c_uchar;

/* BlendEquationSeparate */
pub const GL_FUNC_ADD:                 c_uint = 0x8006 as c_uint;
pub const GL_BLEND_EQUATION:           c_uint = 0x8009 as c_uint;
pub const GL_BLEND_EQUATION_RGB:       c_uint = 0x8009 as c_uint;
pub const GL_BLEND_EQUATION_ALPHA:     c_uint = 0x883D as c_uint;

/* BlendSubtract */
pub const GL_FUNC_SUBTRACT:            c_uint = 0x800A as c_uint;
pub const GL_FUNC_REVERSE_SUBTRACT:    c_uint = 0x800B as c_uint;

/* Separate Blend Functions */
pub const GL_BLEND_DST_RGB:            c_uint = 0x80C8 as c_uint;
pub const GL_BLEND_SRC_RGB:            c_uint = 0x80C9 as c_uint;
pub const GL_BLEND_DST_ALPHA:          c_uint = 0x80CA as c_uint;
pub const GL_BLEND_SRC_ALPHA:          c_uint = 0x80CB as c_uint;
pub const GL_CONSTANT_COLOR:           c_uint = 0x8001 as c_uint;
pub const GL_ONE_MINUS_CONSTANT_COLOR: c_uint = 0x8002 as c_uint;
pub const GL_CONSTANT_ALPHA:           c_uint = 0x8003 as c_uint;
pub const GL_ONE_MINUS_CONSTANT_ALPHA: c_uint = 0x8004 as c_uint;
pub const GL_BLEND_COLOR:              c_uint = 0x8005 as c_uint;

/* Errors. */
pub const GL_NO_ERROR: c_uint = 0 as c_uint;
pub const GL_INVALID_ENUM: c_uint = 0x0500 as c_uint;
pub const GL_INVALID_VALUE: c_uint = 0x0501 as c_uint;
pub const GL_INVALID_OPERATION: c_uint = 0x0502 as c_uint;
pub const GL_STACK_OVERFLOW: c_uint = 0x0503 as c_uint;
pub const GL_STACK_UNDERFLOW: c_uint = 0x0504 as c_uint;
pub const GL_OUT_OF_MEMORY: c_uint = 0x0505 as c_uint;
pub const GL_INVALID_FRAMEBUFFER_OPERATION: c_uint = 0x0506 as c_uint;

/* DataType */
pub const GL_BYTE:           c_uint = 0x1400 as c_uint;
pub const GL_UNSIGNED_BYTE:  c_uint = 0x1401 as c_uint;
pub const GL_SHORT:          c_uint = 0x1402 as c_uint;
pub const GL_UNSIGNED_SHORT: c_uint = 0x1403 as c_uint;
pub const GL_INT:            c_uint = 0x1404 as c_uint;
pub const GL_UNSIGNED_INT:   c_uint = 0x1405 as c_uint;
pub const GL_FLOAT:          c_uint = 0x1406 as c_uint;
pub const GL_FIXED:          c_uint = 0x140C as c_uint;

/* EnableCap */
pub const GL_TEXTURE_2D:               c_uint = 0x0DE1 as c_uint;
pub const GL_CULL_FACE:                c_uint = 0x0B44 as c_uint;
pub const GL_BLEND:                    c_uint = 0x0BE2 as c_uint;
pub const GL_DITHER:                   c_uint = 0x0BD0 as c_uint;
pub const GL_STENCIL_TEST:             c_uint = 0x0B90 as c_uint;
pub const GL_DEPTH_TEST:               c_uint = 0x0B71 as c_uint;
pub const GL_SCISSOR_TEST:             c_uint = 0x0C11 as c_uint;
pub const GL_POLYGON_OFFSET_FILL:      c_uint = 0x8037 as c_uint;
pub const GL_SAMPLE_ALPHA_TO_COVERAGE: c_uint = 0x809E as c_uint;
pub const GL_SAMPLE_COVERAGE:          c_uint = 0x80A0 as c_uint;

/* Polygons */
pub const GL_POINT: c_uint = 0x1B00 as c_uint;
pub const GL_LINE: c_uint = 0x1B01 as c_uint;
pub const GL_FILL: c_uint = 0x1B02 as c_uint;
pub const GL_CW:  c_uint = 0x0900 as c_uint;
pub const GL_CCW: c_uint = 0x0901 as c_uint;
pub const GL_POLYGON_MODE: c_uint = 0x0B40 as c_uint;
pub const GL_POLYGON_SMOOTH: c_uint = 0x0B41 as c_uint;
pub const GL_POLYGON_STIPPLE: c_uint = 0x0B42 as c_uint;
pub const GL_EDGE_FLAG: c_uint = 0x0B43 as c_uint;

/* GetPName */
pub const GL_LINE_WIDTH:                    c_uint = 0x0B21 as c_uint;
pub const GL_ALIASED_POINT_SIZE_RANGE:      c_uint = 0x846D as c_uint;
pub const GL_ALIASED_LINE_WIDTH_RANGE:      c_uint = 0x846E as c_uint;
pub const GL_CULL_FACE_MODE:                c_uint = 0x0B45 as c_uint;
pub const GL_FRONT_FACE:                    c_uint = 0x0B46 as c_uint;
pub const GL_DEPTH_RANGE:                   c_uint = 0x0B70 as c_uint;
pub const GL_DEPTH_WRITEMASK:               c_uint = 0x0B72 as c_uint;
pub const GL_DEPTH_CLEAR_VALUE:             c_uint = 0x0B73 as c_uint;
pub const GL_DEPTH_FUNC:                    c_uint = 0x0B74 as c_uint;
pub const GL_STENCIL_CLEAR_VALUE:           c_uint = 0x0B91 as c_uint;
pub const GL_STENCIL_FUNC:                  c_uint = 0x0B92 as c_uint;
pub const GL_STENCIL_FAIL:                  c_uint = 0x0B94 as c_uint;
pub const GL_STENCIL_PASS_DEPTH_FAIL:       c_uint = 0x0B95 as c_uint;
pub const GL_STENCIL_PASS_DEPTH_PASS:       c_uint = 0x0B96 as c_uint;
pub const GL_STENCIL_REF:                   c_uint = 0x0B97 as c_uint;
pub const GL_STENCIL_VALUE_MASK:            c_uint = 0x0B93 as c_uint;
pub const GL_STENCIL_WRITEMASK:             c_uint = 0x0B98 as c_uint;
pub const GL_STENCIL_BACK_FUNC:             c_uint = 0x8800 as c_uint;
pub const GL_STENCIL_BACK_FAIL:             c_uint = 0x8801 as c_uint;
pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL:  c_uint = 0x8802 as c_uint;
pub const GL_STENCIL_BACK_PASS_DEPTH_PASS:  c_uint = 0x8803 as c_uint;
pub const GL_STENCIL_BACK_REF:              c_uint = 0x8CA3 as c_uint;
pub const GL_STENCIL_BACK_VALUE_MASK:       c_uint = 0x8CA4 as c_uint;
pub const GL_STENCIL_BACK_WRITEMASK:        c_uint = 0x8CA5 as c_uint;
pub const GL_VIEWPORT:                      c_uint = 0x0BA2 as c_uint;
pub const GL_SCISSOR_BOX:                   c_uint = 0x0C10 as c_uint;
/*      SCISSOR_TEST */
pub const GL_COLOR_CLEAR_VALUE:             c_uint = 0x0C22 as c_uint;
pub const GL_COLOR_WRITEMASK:               c_uint = 0x0C23 as c_uint;
pub const GL_UNPACK_ALIGNMENT:              c_uint = 0x0CF5 as c_uint;
pub const GL_PACK_ALIGNMENT:                c_uint = 0x0D05 as c_uint;
pub const GL_MAX_TEXTURE_SIZE:              c_uint = 0x0D33 as c_uint;
pub const GL_MAX_VIEWPORT_DIMS:             c_uint = 0x0D3A as c_uint;
pub const GL_SUBPIXEL_BITS:                 c_uint = 0x0D50 as c_uint;
pub const GL_RED_BITS:                      c_uint = 0x0D52 as c_uint;
pub const GL_GREEN_BITS:                    c_uint = 0x0D53 as c_uint;
pub const GL_BLUE_BITS:                     c_uint = 0x0D54 as c_uint;
pub const GL_ALPHA_BITS:                    c_uint = 0x0D55 as c_uint;
pub const GL_DEPTH_BITS:                    c_uint = 0x0D56 as c_uint;
pub const GL_STENCIL_BITS:                  c_uint = 0x0D57 as c_uint;
pub const GL_POLYGON_OFFSET_UNITS:          c_uint = 0x2A00 as c_uint;
/*      POLYGON_OFFSET_FILL */
pub const GL_POLYGON_OFFSET_FACTOR:         c_uint = 0x8038 as c_uint;
pub const GL_TEXTURE_BINDING_2D:            c_uint = 0x8069 as c_uint;
pub const GL_SAMPLE_BUFFERS:                c_uint = 0x80A8 as c_uint;
pub const GL_SAMPLES:                       c_uint = 0x80A9 as c_uint;
pub const GL_SAMPLE_COVERAGE_VALUE:         c_uint = 0x80AA as c_uint;
pub const GL_SAMPLE_COVERAGE_INVERT:        c_uint = 0x80AB as c_uint;

/* GetTarget */
pub const GL_UNPACK_ROW_LENGTH: c_uint = 0x0CF2 as c_uint;

/* PixelFormat */
pub const GL_DEPTH_COMPONENT: c_uint = 0x1902 as c_uint;
pub const GL_RED:             c_uint = 0x1903 as c_uint;
pub const GL_GREEN:           c_uint = 0x1904 as c_uint;
pub const GL_BLUE:            c_uint = 0x1905 as c_uint;
pub const GL_ALPHA:           c_uint = 0x1906 as c_uint;
pub const GL_RGB:             c_uint = 0x1907 as c_uint;
pub const GL_RGBA:            c_uint = 0x1908 as c_uint;

pub const GL_BGRA:            c_uint = 0x80e1 as c_uint;   // NB: Not OpenGL ES!
pub const GL_RGBA8:           c_uint = 0x8058 as c_uint;   // NB: Not OpenGL ES!

/* Packed Pixels */
pub const GL_UNSIGNED_INT_8_8_8_8_REV: c_uint = 0x8367 as c_uint; // NB: Not OpenGL ES!

/* Shaders */
pub const GL_FRAGMENT_SHADER:                  c_uint = 0x8B30 as c_uint;
pub const GL_VERTEX_SHADER:                    c_uint = 0x8B31 as c_uint;
pub const GL_MAX_VERTEX_ATTRIBS:               c_uint = 0x8869 as c_uint;
pub const GL_MAX_VERTEX_UNIFORM_VECTORS:       c_uint = 0x8DFB as c_uint;
pub const GL_MAX_VARYING_VECTORS:              c_uint = 0x8DFC as c_uint;
pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: c_uint = 0x8B4D as c_uint;
pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS:   c_uint = 0x8B4C as c_uint;
pub const GL_MAX_TEXTURE_IMAGE_UNITS:          c_uint = 0x8872 as c_uint;
pub const GL_MAX_FRAGMENT_UNIFORM_VECTORS:     c_uint = 0x8DFD as c_uint;
pub const GL_SHADER_TYPE:                      c_uint = 0x8B4F as c_uint;
pub const GL_DELETE_STATUS:                    c_uint = 0x8B80 as c_uint;
pub const GL_LINK_STATUS:                      c_uint = 0x8B82 as c_uint;
pub const GL_VALIDATE_STATUS:                  c_uint = 0x8B83 as c_uint;
pub const GL_ATTACHED_SHADERS:                 c_uint = 0x8B85 as c_uint;
pub const GL_ACTIVE_UNIFORMS:                  c_uint = 0x8B86 as c_uint;
pub const GL_ACTIVE_UNIFORM_MAX_LENGTH:        c_uint = 0x8B87 as c_uint;
pub const GL_ACTIVE_ATTRIBUTES:                c_uint = 0x8B89 as c_uint;
pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH:      c_uint = 0x8B8A as c_uint;
pub const GL_SHADING_LANGUAGE_VERSION:         c_uint = 0x8B8C as c_uint;
pub const GL_CURRENT_PROGRAM:                  c_uint = 0x8B8D as c_uint;

/* StencilFunction */
pub const GL_NEVER:    c_uint = 0x0200 as c_uint;
pub const GL_LESS:     c_uint = 0x0201 as c_uint;
pub const GL_EQUAL:    c_uint = 0x0202 as c_uint;
pub const GL_LEQUAL:   c_uint = 0x0203 as c_uint;
pub const GL_GREATER:  c_uint = 0x0204 as c_uint;
pub const GL_NOTEQUAL: c_uint = 0x0205 as c_uint;
pub const GL_GEQUAL:   c_uint = 0x0206 as c_uint;
pub const GL_ALWAYS:   c_uint = 0x0207 as c_uint;

pub const GL_VENDOR:     c_uint = 0x1F00 as c_uint;
pub const GL_RENDERER:   c_uint = 0x1F01 as c_uint;
pub const GL_VERSION:    c_uint = 0x1F02 as c_uint;
pub const GL_EXTENSIONS: c_uint = 0x1F03 as c_uint;

/* Shader Source */
pub const GL_COMPILE_STATUS:       c_uint = 0x8B81 as c_uint;
pub const GL_INFO_LOG_LENGTH:      c_uint = 0x8B84 as c_uint;
pub const GL_SHADER_SOURCE_LENGTH: c_uint = 0x8B88 as c_uint;
pub const GL_SHADER_COMPILER:      c_uint = 0x8DFA as c_uint;

/* Buffer Objects */
pub const GL_ARRAY_BUFFER:                 c_uint = 0x8892 as c_uint;
pub const GL_ELEMENT_ARRAY_BUFFER:         c_uint = 0x8893 as c_uint;
pub const GL_ARRAY_BUFFER_BINDING:         c_uint = 0x8894 as c_uint;
pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: c_uint = 0x8895 as c_uint;

pub const GL_STREAM_DRAW:  c_uint = 0x88E0 as c_uint;
pub const GL_STATIC_DRAW:  c_uint = 0x88E4 as c_uint;
pub const GL_DYNAMIC_DRAW: c_uint = 0x88E8 as c_uint;

/* CullFaceMode */
pub const GL_FRONT: c_uint =           0x0404 as c_uint;
pub const GL_BACK: c_uint =            0x0405 as c_uint;
pub const GL_FRONT_AND_BACK: c_uint =  0x0408 as c_uint;

/* TextureMagFilter */
pub const GL_NEAREST: c_uint = 0x2600 as c_uint;
pub const GL_LINEAR:  c_uint = 0x2601 as c_uint;

/* TextureParameterName */
pub const GL_TEXTURE_MAG_FILTER: c_uint = 0x2800 as c_uint;
pub const GL_TEXTURE_MIN_FILTER: c_uint = 0x2801 as c_uint;
pub const GL_TEXTURE_WRAP_S:     c_uint = 0x2802 as c_uint;
pub const GL_TEXTURE_WRAP_T:     c_uint = 0x2803 as c_uint;

/* TextureUnit */
pub const GL_TEXTURE0:       c_uint = 0x84C0 as c_uint;
pub const GL_TEXTURE1:       c_uint = 0x84C1 as c_uint;
pub const GL_TEXTURE2:       c_uint = 0x84C2 as c_uint;
pub const GL_TEXTURE3:       c_uint = 0x84C3 as c_uint;
pub const GL_TEXTURE4:       c_uint = 0x84C4 as c_uint;
pub const GL_TEXTURE5:       c_uint = 0x84C5 as c_uint;
pub const GL_TEXTURE6:       c_uint = 0x84C6 as c_uint;
pub const GL_TEXTURE7:       c_uint = 0x84C7 as c_uint;
pub const GL_TEXTURE8:       c_uint = 0x84C8 as c_uint;
pub const GL_TEXTURE9:       c_uint = 0x84C9 as c_uint;
pub const GL_TEXTURE10:      c_uint = 0x84CA as c_uint;
pub const GL_TEXTURE11:      c_uint = 0x84CB as c_uint;
pub const GL_TEXTURE12:      c_uint = 0x84CC as c_uint;
pub const GL_TEXTURE13:      c_uint = 0x84CD as c_uint;
pub const GL_TEXTURE14:      c_uint = 0x84CE as c_uint;
pub const GL_TEXTURE15:      c_uint = 0x84CF as c_uint;
pub const GL_TEXTURE16:      c_uint = 0x84D0 as c_uint;
pub const GL_TEXTURE17:      c_uint = 0x84D1 as c_uint;
pub const GL_TEXTURE18:      c_uint = 0x84D2 as c_uint;
pub const GL_TEXTURE19:      c_uint = 0x84D3 as c_uint;
pub const GL_TEXTURE20:      c_uint = 0x84D4 as c_uint;
pub const GL_TEXTURE21:      c_uint = 0x84D5 as c_uint;
pub const GL_TEXTURE22:      c_uint = 0x84D6 as c_uint;
pub const GL_TEXTURE23:      c_uint = 0x84D7 as c_uint;
pub const GL_TEXTURE24:      c_uint = 0x84D8 as c_uint;
pub const GL_TEXTURE25:      c_uint = 0x84D9 as c_uint;
pub const GL_TEXTURE26:      c_uint = 0x84DA as c_uint;
pub const GL_TEXTURE27:      c_uint = 0x84DB as c_uint;
pub const GL_TEXTURE28:      c_uint = 0x84DC as c_uint;
pub const GL_TEXTURE29:      c_uint = 0x84DD as c_uint;
pub const GL_TEXTURE30:      c_uint = 0x84DE as c_uint;
pub const GL_TEXTURE31:      c_uint = 0x84DF as c_uint;
pub const GL_ACTIVE_TEXTURE: c_uint = 0x84E0 as c_uint;

/* TextureWrapMode */
pub const GL_REPEAT:          c_uint = 0x2901 as c_uint;
pub const GL_CLAMP_TO_EDGE:   c_uint = 0x812F as c_uint;
pub const GL_MIRRORED_REPEAT: c_uint = 0x8370 as c_uint;

pub const GL_COLOR_ATTACHMENT0: c_uint = 0x8CE0 as c_uint;

pub const GL_FRAMEBUFFER_COMPLETE: c_uint = 0x8CD5 as c_uint;

// Framebuffer Object
pub const GL_FRAMEBUFFER:  c_uint = 0x8D40 as c_uint;
pub const GL_RENDERBUFFER: c_uint = 0x8D41 as c_uint;

// Extensions
pub const GL_TEXTURE_RECTANGLE_ARB: c_uint = 0x84F5 as c_uint;         // NB: Not OpenGL ES!

pub const GL_UNPACK_CLIENT_STORAGE_APPLE: c_uint = 0x85B2 as c_uint;   // NB: Not OpenGL ES!
pub const GL_TEXTURE_STORAGE_HINT_APPLE: c_uint = 0x85BC as c_uint;    // NB: Not OpenGL ES!
pub const GL_STORAGE_CACHED_APPLE: c_uint = 0x85BE as c_uint;          // NB: Not OpenGL ES!
pub const GL_STORAGE_SHARED_APPLE: c_uint = 0x85BF as c_uint;          // NB: Not OpenGL ES!
