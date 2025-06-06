type SDLPixelFormat = u32;

#[allow(dead_code)]
pub const SDL_PIXELFORMAT_UNKNOWN: SDLPixelFormat = 0;
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_INDEX1LSB: SDLPixelFormat = 0x11100100;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_INDEX1, SDL_BITMAPORDER_4321, 0, 1, 0), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_INDEX1MSB: SDLPixelFormat = 0x11200100;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_INDEX1, SDL_BITMAPORDER_1234, 0, 1, 0), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_INDEX2LSB: SDLPixelFormat = 0x1c100200;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_INDEX2, SDL_BITMAPORDER_4321, 0, 2, 0), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_INDEX2MSB: SDLPixelFormat = 0x1c200200;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_INDEX2, SDL_BITMAPORDER_1234, 0, 2, 0), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_INDEX4LSB: SDLPixelFormat = 0x12100400;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_INDEX4, SDL_BITMAPORDER_4321, 0, 4, 0), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_INDEX4MSB: SDLPixelFormat = 0x12200400;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_INDEX4, SDL_BITMAPORDER_1234, 0, 4, 0), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_INDEX8: SDLPixelFormat = 0x13000801;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_INDEX8, 0, 0, 8, 1), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_RGB332: SDLPixelFormat = 0x14110801;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED8, SDL_PACKEDORDER_XRGB, SDL_PACKEDLAYOUT_332, 8, 1), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_XRGB4444: SDLPixelFormat = 0x15120c02;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_XRGB, SDL_PACKEDLAYOUT_4444, 12, 2), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_XBGR4444: SDLPixelFormat = 0x15520c02;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_XBGR, SDL_PACKEDLAYOUT_4444, 12, 2), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_XRGB1555: SDLPixelFormat = 0x15130f02;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_XRGB, SDL_PACKEDLAYOUT_1555, 15, 2), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_XBGR1555: SDLPixelFormat = 0x15530f02;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_XBGR, SDL_PACKEDLAYOUT_1555, 15, 2), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_ARGB4444: SDLPixelFormat = 0x15321002;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_ARGB, SDL_PACKEDLAYOUT_4444, 16, 2), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_RGBA4444: SDLPixelFormat = 0x15421002;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_RGBA, SDL_PACKEDLAYOUT_4444, 16, 2), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_ABGR4444: SDLPixelFormat = 0x15721002;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_ABGR, SDL_PACKEDLAYOUT_4444, 16, 2), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_BGRA4444: SDLPixelFormat = 0x15821002;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_BGRA, SDL_PACKEDLAYOUT_4444, 16, 2), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_ARGB1555: SDLPixelFormat = 0x15331002;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_ARGB, SDL_PACKEDLAYOUT_1555, 16, 2), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_RGBA5551: SDLPixelFormat = 0x15441002;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_RGBA, SDL_PACKEDLAYOUT_5551, 16, 2), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_ABGR1555: SDLPixelFormat = 0x15731002;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_ABGR, SDL_PACKEDLAYOUT_1555, 16, 2), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_BGRA5551: SDLPixelFormat = 0x15841002;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_BGRA, SDL_PACKEDLAYOUT_5551, 16, 2), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_RGB565: SDLPixelFormat = 0x15151002;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_XRGB, SDL_PACKEDLAYOUT_565, 16, 2), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_BGR565: SDLPixelFormat = 0x15551002;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_XBGR, SDL_PACKEDLAYOUT_565, 16, 2), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_RGB24: SDLPixelFormat = 0x17101803;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYU8, SDL_ARRAYORDER_RGB, 0, 24, 3), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_BGR24: SDLPixelFormat = 0x17401803;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYU8, SDL_ARRAYORDER_BGR, 0, 24, 3), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_XRGB8888: SDLPixelFormat = 0x16161804;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_XRGB, SDL_PACKEDLAYOUT_8888, 24, 4), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_RGBX8888: SDLPixelFormat = 0x16261804;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_RGBX, SDL_PACKEDLAYOUT_8888, 24, 4), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_XBGR8888: SDLPixelFormat = 0x16561804;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_XBGR, SDL_PACKEDLAYOUT_8888, 24, 4), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_BGRX8888: SDLPixelFormat = 0x16661804;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_BGRX, SDL_PACKEDLAYOUT_8888, 24, 4), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_ARGB8888: SDLPixelFormat = 0x16362004;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_ARGB, SDL_PACKEDLAYOUT_8888, 32, 4), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_RGBA8888: SDLPixelFormat = 0x16462004;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_RGBA, SDL_PACKEDLAYOUT_8888, 32, 4), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_ABGR8888: SDLPixelFormat = 0x16762004;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_ABGR, SDL_PACKEDLAYOUT_8888, 32, 4), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_BGRA8888: SDLPixelFormat = 0x16862004;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_BGRA, SDL_PACKEDLAYOUT_8888, 32, 4), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_XRGB2101010: SDLPixelFormat = 0x16172004;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_XRGB, SDL_PACKEDLAYOUT_2101010, 32, 4), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_XBGR2101010: SDLPixelFormat = 0x16572004;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_XBGR, SDL_PACKEDLAYOUT_2101010, 32, 4), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_ARGB2101010: SDLPixelFormat = 0x16372004;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_ARGB, SDL_PACKEDLAYOUT_2101010, 32, 4), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_ABGR2101010: SDLPixelFormat = 0x16772004;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_ABGR, SDL_PACKEDLAYOUT_2101010, 32, 4), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_RGB48: SDLPixelFormat = 0x18103006;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYU16, SDL_ARRAYORDER_RGB, 0, 48, 6), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_BGR48: SDLPixelFormat = 0x18403006;
#[allow(dead_code)]
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYU16, SDL_ARRAYORDER_BGR, 0, 48, 6), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_RGBA64: SDLPixelFormat = 0x18204008;
#[allow(dead_code)]
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYU16, SDL_ARRAYORDER_RGBA, 0, 64, 8), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_ARGB64: SDLPixelFormat = 0x18304008;
#[allow(dead_code)]
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYU16, SDL_ARRAYORDER_ARGB, 0, 64, 8), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_BGRA64: SDLPixelFormat = 0x18504008;
#[allow(dead_code)]
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYU16, SDL_ARRAYORDER_BGRA, 0, 64, 8), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_ABGR64: SDLPixelFormat = 0x18604008;
#[allow(dead_code)]
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYU16, SDL_ARRAYORDER_ABGR, 0, 64, 8), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_RGB48_FLOAT: SDLPixelFormat = 0x1a103006;
#[allow(dead_code)]
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF16, SDL_ARRAYORDER_RGB, 0, 48, 6), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_BGR48_FLOAT: SDLPixelFormat = 0x1a403006;
#[allow(dead_code)]
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF16, SDL_ARRAYORDER_BGR, 0, 48, 6), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_RGBA64_FLOAT: SDLPixelFormat = 0x1a204008;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF16, SDL_ARRAYORDER_RGBA, 0, 64, 8), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_ARGB64_FLOAT: SDLPixelFormat = 0x1a304008;
#[allow(dead_code)]
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF16, SDL_ARRAYORDER_ARGB, 0, 64, 8), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_BGRA64_FLOAT: SDLPixelFormat = 0x1a504008;
#[allow(dead_code)]
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF16, SDL_ARRAYORDER_BGRA, 0, 64, 8), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_ABGR64_FLOAT: SDLPixelFormat = 0x1a604008;
#[allow(dead_code)]
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF16, SDL_ARRAYORDER_ABGR, 0, 64, 8), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_RGB96_FLOAT: SDLPixelFormat = 0x1b10600c;
#[allow(dead_code)]
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF32, SDL_ARRAYORDER_RGB, 0, 96, 12), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_BGR96_FLOAT: SDLPixelFormat = 0x1b40600c;
#[allow(dead_code)]
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF32, SDL_ARRAYORDER_BGR, 0, 96, 12), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_RGBA128_FLOAT: SDLPixelFormat = 0x1b208010;
#[allow(dead_code)]
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF32, SDL_ARRAYORDER_RGBA, 0, 128, 16), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_ARGB128_FLOAT: SDLPixelFormat = 0x1b308010;
#[allow(dead_code)]
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF32, SDL_ARRAYORDER_ARGB, 0, 128, 16), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_BGRA128_FLOAT: SDLPixelFormat = 0x1b508010;
#[allow(dead_code)]
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF32, SDL_ARRAYORDER_BGRA, 0, 128, 16), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_ABGR128_FLOAT: SDLPixelFormat = 0x1b608010;
/* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF32, SDL_ARRAYORDER_ABGR, 0, 128, 16), */

#[allow(dead_code)]
pub const SDL_PIXELFORMAT_YV12: SDLPixelFormat = 0x32315659;      /**< Planar mode: Y + V + U  (3 planes) */
/* SDL_DEFINE_PIXELFOURCC('Y', 'V', '1', '2'), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_IYUV: SDLPixelFormat = 0x56555949;      /**< Planar mode: Y + U + V  (3 planes) */
#[allow(dead_code)]
/* SDL_DEFINE_PIXELFOURCC('I', 'Y', 'U', 'V'), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_YUY2: SDLPixelFormat = 0x32595559;      /**< Packed mode: Y0+U0+Y1+V0 (1 plane) */
#[allow(dead_code)]
/* SDL_DEFINE_PIXELFOURCC('Y', 'U', 'Y', '2'), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_UYVY: SDLPixelFormat = 0x59565955;      /**< Packed mode: U0+Y0+V0+Y1 (1 plane) */
#[allow(dead_code)]
/* SDL_DEFINE_PIXELFOURCC('U', 'Y', 'V', 'Y'), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_YVYU: SDLPixelFormat = 0x55595659;      /**< Packed mode: Y0+V0+Y1+U0 (1 plane) */
#[allow(dead_code)]
/* SDL_DEFINE_PIXELFOURCC('Y', 'V', 'Y', 'U'), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_NV12: SDLPixelFormat = 0x3231564e;      /**< Planar mode: Y + U/V interleaved  (2 planes) */
#[allow(dead_code)]
/* SDL_DEFINE_PIXELFOURCC('N', 'V', '1', '2'), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_NV21: SDLPixelFormat = 0x3132564e;      /**< Planar mode: Y + V/U interleaved  (2 planes) */
#[allow(dead_code)]
/* SDL_DEFINE_PIXELFOURCC('N', 'V', '2', '1'), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_P010: SDLPixelFormat = 0x30313050;      /**< Planar mode: Y + U/V interleaved  (2 planes) */
#[allow(dead_code)]
/* SDL_DEFINE_PIXELFOURCC('P', '0', '1', '0'), */
#[allow(dead_code)]
pub const SDL_PIXELFORMAT_EXTERNAL_OES: SDLPixelFormat = 0x2053454f;     /**< Android video texture format */
/* SDL_DEFINE_PIXELFOURCC('O', 'E', 'S', ' ') */

/* Aliases for RGBA byte arrays of color data, for the current platform */
//XXX
//#if SDL_BYTEORDER == SDL_BIG_ENDIAN
//SDL_PIXELFORMAT_RGBA32 = SDL_PIXELFORMAT_RGBA8888,
//SDL_PIXELFORMAT_ARGB32 = SDL_PIXELFORMAT_ARGB8888,
//SDL_PIXELFORMAT_BGRA32 = SDL_PIXELFORMAT_BGRA8888,
//SDL_PIXELFORMAT_ABGR32 = SDL_PIXELFORMAT_ABGR8888,
//SDL_PIXELFORMAT_RGBX32 = SDL_PIXELFORMAT_RGBX8888,
//SDL_PIXELFORMAT_XRGB32 = SDL_PIXELFORMAT_XRGB8888,
//SDL_PIXELFORMAT_BGRX32 = SDL_PIXELFORMAT_BGRX8888,
//SDL_PIXELFORMAT_XBGR32 = SDL_PIXELFORMAT_XBGR8888
//#else
//SDL_PIXELFORMAT_RGBA32 = SDL_PIXELFORMAT_ABGR8888,
//SDL_PIXELFORMAT_ARGB32 = SDL_PIXELFORMAT_BGRA8888,
//SDL_PIXELFORMAT_BGRA32 = SDL_PIXELFORMAT_ARGB8888,
//SDL_PIXELFORMAT_ABGR32 = SDL_PIXELFORMAT_RGBA8888,
//SDL_PIXELFORMAT_RGBX32 = SDL_PIXELFORMAT_XBGR8888,
//SDL_PIXELFORMAT_XRGB32 = SDL_PIXELFORMAT_BGRX8888,
//SDL_PIXELFORMAT_BGRX32 = SDL_PIXELFORMAT_XRGB8888,
//SDL_PIXELFORMAT_XBGR32 = SDL_PIXELFORMAT_RGBX8888
//#endif
//} SDLPixelFormat;
//typedef Uint32 SDL_SurfaceFlags;
type SDLSurfaceFlags = u32;

#[allow(dead_code)]
pub const SDL_SURFACE_PREALLOCATED: SDLSurfaceFlags     = 0x00000001; /**< Surface uses preallocated pixel memory */
#[allow(dead_code)]
pub const SDL_SURFACE_LOCK_NEEDED: SDLSurfaceFlags      = 0x00000002; /**< Surface needs to be locked to access pixels */
#[allow(dead_code)]
pub const SDL_SURFACE_LOCKED: SDLSurfaceFlags           = 0x00000004; /**< Surface is currently locked */
#[allow(dead_code)]
pub const SDL_SURFACE_SIMD_ALIGNED: SDLSurfaceFlags     = 0x00000008; /**< Surface uses pixel memory allocated with SDL_aligned_alloc() */
//
//struct SDL_Surface
//{
//    SDL_SurfaceFlags flags;     /**< The flags of the surface, read-only */
//    SDLPixelFormat format;     /**< The format of the surface, read-only */
//    int w;                      /**< The width of the surface, read-only. */
//    int h;                      /**< The height of the surface, read-only. */
//    int pitch;                  /**< The distance in bytes between rows of pixels, read-only */
//    void *pixels;               /**< A pointer to the pixels of the surface, the pixels are writeable if non-NULL */
//
//    int refcount;               /**< Application reference count, used when freeing surface */
//
//    void *reserved;             /**< Reserved for internal use */
//};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDLSurface
{
    pub flags:      SDLSurfaceFlags,
    pub format:     SDLPixelFormat,
    pub w:          i32,
    pub h:          i32,
    pub pitch:      i32,
    pub pixels:     *mut std::ffi::c_void,
}
