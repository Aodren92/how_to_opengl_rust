pub mod event;
use gl33::global_loader::load_global_gl;

type InitFlag = u32;
type SDLWindowID = u32;
type SDLKeyboardID = u32;
type SDLScancode = u32;
type SDLKeycode = u32;
type SDLKeymod = u16;


pub const SDL_INIT_AUDIO: InitFlag      = 0x00000010;
pub const SDL_INIT_VIDEO: InitFlag      = 0x00000020;
pub const SDL_INIT_JOYSTICK: InitFlag   = 0x00000200;
pub const SDL_INIT_HAPTIC: InitFlag     = 0x00001000;
pub const SDL_INIT_GAMEPAD: InitFlag    = 0x00002000;
pub const SDL_INIT_EVENTS: InitFlag     = 0x00004000;
pub const SDL_INIT_SENSOR: InitFlag     = 0x00008000;
pub const SDL_INIT_CAMERA: InitFlag     = 0x00010000;
pub const SDL_INIT_EVERYTHING: InitFlag = SDL_INIT_AUDIO | SDL_INIT_VIDEO | SDL_INIT_JOYSTICK | SDL_INIT_HAPTIC | SDL_INIT_GAMEPAD | SDL_INIT_EVENTS | SDL_INIT_SENSOR | SDL_INIT_CAMERA;


#[allow(dead_code)]
pub const SDL_GL_RED_SIZE: u32                      = 0;    /**< the minimum number of bits for the red channel of the color buffer; defaults to 8. */
#[allow(dead_code)]
pub const SDL_GL_GREEN_SIZE: u32                    = 1;    /**< the minimum number of bits for the green channel of the color buffer; defaults to 8. */
#[allow(dead_code)]
pub const SDL_GL_BLUE_SIZE: u32                     = 2;    /**< the minimum number of bits for the blue channel of the color buffer; defaults to 8. */
#[allow(dead_code)]
pub const SDL_GL_ALPHA_SIZE: u32                    = 3;    /**< the minimum number of bits for the alpha channel of the color buffer; defaults to 8. */
#[allow(dead_code)]
pub const SDL_GL_BUFFER_SIZE: u32                   = 4;    /**< the minimum number of bits for frame buffer size; defaults to 0. */
#[allow(dead_code)]
pub const SDL_GL_DOUBLEBUFFER: u32                  = 5;    /**< whether the output is single or double buffered; defaults to double buffering on. */
#[allow(dead_code)]
pub const SDL_GL_DEPTH_SIZE: u32                    = 6;    /**< the minimum number of bits in the depth buffer; defaults to 16. */
#[allow(dead_code)]
pub const SDL_GL_STENCIL_SIZE: u32                  = 7;    /**< the minimum number of bits in the stencil buffer; defaults to 0. */
#[allow(dead_code)]
pub const SDL_GL_ACCUM_RED_SIZE: u32                = 8;    /**< the minimum number of bits for the red channel of the accumulation buffer; defaults to 0. */
#[allow(dead_code)]
pub const SDL_GL_ACCUM_GREEN_SIZE: u32              = 9;    /**< the minimum number of bits for the green channel of the accumulation buffer; defaults to 0. */
#[allow(dead_code)]
pub const SDL_GL_ACCUM_BLUE_SIZE: u32               = 10;   /**< the minimum number of bits for the blue channel of the accumulation buffer; defaults to 0. */
#[allow(dead_code)]
pub const SDL_GL_ACCUM_ALPHA_SIZE: u32              = 11;   /**< the minimum number of bits for the alpha channel of the accumulation buffer; defaults to 0. */
#[allow(dead_code)]
pub const SDL_GL_STEREO: u32                        = 12;   /**< whether the output is stereo 3D; defaults to off. */
#[allow(dead_code)]
pub const SDL_GL_MULTISAMPLEBUFFERS: u32            = 13;   /**< the number of buffers used for multisample anti-aliasing; defaults to 0. */
#[allow(dead_code)]
pub const SDL_GL_MULTISAMPLESAMPLES: u32            = 14;   /**< the number of samples used around the current pixel used for multisample anti-aliasing. */
#[allow(dead_code)]
pub const SDL_GL_ACCELERATED_VISUAL: u32            = 15;   /**< set to 1 to require hardware acceleration, set to 0 to force software rendering; defaults to allow either. */
#[allow(dead_code)]
pub const SDL_GL_RETAINED_BACKING: u32              = 16;   /**< not used (deprecated). */
#[allow(dead_code)]
pub const SDL_GL_CONTEXT_MAJOR_VERSION: u32         = 17;   /**< OpenGL context major version. */
#[allow(dead_code)]
pub const SDL_GL_CONTEXT_MINOR_VERSION: u32         = 18;   /**< OpenGL context minor version. */
#[allow(dead_code)]
pub const SDL_GL_CONTEXT_FLAGS: u32                 = 19;   /**< some combination of 0 or more of elements of the SDL_GLContextFlag enumeration; defaults to 0. */
#[allow(dead_code)]
pub const SDL_GL_CONTEXT_PROFILE_MASK: u32          = 20;   /**< type of GL context (Core, Compatibility, ES). See SDL_GLProfile; default value depends on platform. */
#[allow(dead_code)]
pub const SDL_GL_SHARE_WITH_CURRENT_CONTEXT: u32    = 21;   /**< OpenGL context sharing; defaults to 0. */
#[allow(dead_code)]
pub const SDL_GL_FRAMEBUFFER_SRGB_CAPABLE: u32      = 22;   /**< requests sRGB capable visual; defaults to 0. */
#[allow(dead_code)]
pub const SDL_GL_CONTEXT_RELEASE_BEHAVIOR: u32      = 23;   /**< sets context the release behavior. See SDL_GLContextReleaseFlag; defaults to FLUSH. */
#[allow(dead_code)]
pub const SDL_GL_CONTEXT_RESET_NOTIFICATION: u32    = 24;   /**< set context reset notification. See SDL_GLContextResetNotification; defaults to NO_NOTIFICATION. */
#[allow(dead_code)]
pub const SDL_GL_CONTEXT_NO_ERROR: u32              = 25;
#[allow(dead_code)]
pub const SDL_GL_FLOATBUFFERS: u32                  = 26;
#[allow(dead_code)]
pub const SDL_GL_EGL_PLATFORM: u32                  = 27;


#[allow(dead_code)]
pub const SDL_GL_CONTEXT_DEBUG_FLAG: u32                = 0x0001;
#[allow(dead_code)]
pub const SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG: u32   = 0x0002;
#[allow(dead_code)]
pub const SDL_GL_CONTEXT_ROBUST_ACCESS_FLAG: u32        = 0x0004;
#[allow(dead_code)]
pub const SDL_GL_CONTEXT_RESET_ISOLATION_FLAG: u32      = 0x0008;

// SDL WINDOW FLAG
type SdlWindowFlags = u64;
#[allow(dead_code)]
pub const SDL_WINDOW_FULLSCREEN: SdlWindowFlags            = 0x0000000000000001;    /**< window is in fullscreen mode */
#[allow(dead_code)]
pub const SDL_WINDOW_OPENGL: SdlWindowFlags                = 0x0000000000000002;    /**< window usable with OpenGL context */
#[allow(dead_code)]
pub const SDL_WINDOW_OCCLUDED: SdlWindowFlags              = 0x0000000000000004;    /**< window is occluded */
#[allow(dead_code)]
pub const SDL_WINDOW_HIDDEN: SdlWindowFlags                = 0x0000000000000008;    /**< window is neither mapped onto the desktop nor shown in the taskbar/dock/window list; SDL_ShowWindow() is required for it to become visible */
#[allow(dead_code)]
pub const SDL_WINDOW_BORDERLESS: SdlWindowFlags            = 0x0000000000000010;    /**< no window decoration */
#[allow(dead_code)]
pub const SDL_WINDOW_RESIZABLE: SdlWindowFlags             = 0x0000000000000020;    /**< window can be resized */
#[allow(dead_code)]
pub const SDL_WINDOW_MINIMIZED: SdlWindowFlags             = 0x0000000000000040;    /**< window is minimized */
#[allow(dead_code)]
pub const SDL_WINDOW_MAXIMIZED: SdlWindowFlags             = 0x0000000000000080;    /**< window is maximized */
#[allow(dead_code)]
pub const SDL_WINDOW_MOUSE_GRABBED: SdlWindowFlags         = 0x0000000000000100;    /**< window has grabbed mouse input */
#[allow(dead_code)]
pub const SDL_WINDOW_INPUT_FOCUS: SdlWindowFlags           = 0x0000000000000200;    /**< window has input focus */
#[allow(dead_code)]
pub const SDL_WINDOW_MOUSE_FOCUS: SdlWindowFlags           = 0x0000000000000400;    /**< window has mouse focus */
#[allow(dead_code)]
pub const SDL_WINDOW_EXTERNAL: SdlWindowFlags              = 0x0000000000000800;    /**< window not created by SDL */
#[allow(dead_code)]
pub const SDL_WINDOW_MODAL: SdlWindowFlags                 = 0x0000000000001000;    /**< window is modal */
#[allow(dead_code)]
pub const SDL_WINDOW_HIGH_PIXEL_DENSITY: SdlWindowFlags    = 0x0000000000002000;    /**< window uses high pixel density back buffer if possible */
#[allow(dead_code)]
pub const SDL_WINDOW_MOUSE_CAPTURE: SdlWindowFlags         = 0x0000000000004000;    /**< window has mouse captured (unrelated to MOUSE_GRABBED) */
#[allow(dead_code)]
pub const SDL_WINDOW_MOUSE_RELATIVE_MODE: SdlWindowFlags   = 0x0000000000008000;    /**< window has relative mode enabled */
#[allow(dead_code)]
pub const SDL_WINDOW_ALWAYS_ON_TOP: SdlWindowFlags         = 0x0000000000010000;    /**< window should always be above others */
#[allow(dead_code)]
pub const SDL_WINDOW_UTILITY: SdlWindowFlags               = 0x0000000000020000;    /**< window should be treated as a utility window, not showing in the task bar and window list */
#[allow(dead_code)]
pub const SDL_WINDOW_TOOLTIP: SdlWindowFlags               = 0x0000000000040000;    /**< window should be treated as a tooltip and does not get mouse or keyboard focus, requires a parent window */
#[allow(dead_code)]
pub const SDL_WINDOW_POPUP_MENU: SdlWindowFlags            = 0x0000000000080000;    /**< window should be treated as a popup menu, requires a parent window */
#[allow(dead_code)]
pub const SDL_WINDOW_KEYBOARD_GRABBED: SdlWindowFlags      = 0x0000000000100000;    /**< window has grabbed keyboard input */
#[allow(dead_code)]
pub const SDL_WINDOW_VULKAN: SdlWindowFlags                = 0x0000000010000000;    /**< window usable for Vulkan surface */
#[allow(dead_code)]
pub const SDL_WINDOW_METAL: SdlWindowFlags                 = 0x0000000020000000;    /**< window usable for Metal view */
#[allow(dead_code)]
pub const SDL_WINDOW_TRANSPARENT: SdlWindowFlags           = 0x0000000040000000;    /**< window with transparent buffer */
#[allow(dead_code)]
pub const SDL_WINDOW_NOT_FOCUSABLE: SdlWindowFlags         = 0x0000000080000000;    /**< window should not be focusable */

#[repr(C)]
pub struct SDL_Window {
    _data: (),
    _marker:
        core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>, 
}


#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct SdlGlcontext(pub *mut std::os::raw::c_void);
impl SdlGlcontext {
  /// Checks if the context pointer is null.
  pub fn is_null(self) -> bool {
    self.0.is_null()
  }
}

unsafe extern "C" {


    // init
    fn SDL_Init(flags: InitFlag) -> bool;
    fn SDL_CreateWindow(title: *const  std::ffi::c_char , w: i32, h: i32, flags: u64) -> *mut SDL_Window;

    // GL functions
    fn SDL_GL_SetAttribute(attr: u32, value: std::os::raw::c_int) -> bool;
    fn SDL_GL_CreateContext(window: *mut SDL_Window) -> SdlGlcontext;
    fn SDL_GL_MakeCurrent(window: *mut SDL_Window, context: SdlGlcontext) -> bool;
    fn SDL_GL_GetProcAddress(proc: *const i8) -> *mut std::os::raw::c_void;
    pub fn SDL_GL_SwapWindow(window: *mut SDL_Window) -> bool;


    //param interval 0 for immediate updates, 1 for updates synchronized with
    //               the vertical retrace, -1 for adaptive vsync.
    fn SDL_GL_SetSwapInterval(interval: std::os::raw::c_int) -> bool;

    pub fn SDL_Delay(ms: std::os::raw::c_int);
    // error
    pub fn SDL_GetError() -> *const std::ffi::c_char;
}


pub struct SDL {
    pub window:         *mut SDL_Window,
    pub gl_context:     SdlGlcontext,
}


impl SDL {

    pub fn print_error(message: String) {
        let ret:        *const std::ffi::c_char = unsafe { SDL_GetError() } ;
        let c_str:      &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(ret) };
        let str_slice:  &str = c_str.to_str().unwrap();
        println!("{message}: {str_slice}");
    }

    pub fn init(flags: InitFlag) -> Self {
        let window;
        let gl_context: SdlGlcontext;
        unsafe {
            SDL_Init(flags);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 3);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, 1);


            #[cfg(target_os = "macos")]
            {
                SDL_GL_SetAttribute(SDL_GL_CONTEXT_FLAGS, 2);
            }
            let title = std::ffi::CString::new("How to opengl").unwrap();
            //XXX check window
            window = SDL_CreateWindow(title.as_ptr(), 450, 600, SDL_WINDOW_OPENGL | SDL_WINDOW_RESIZABLE | SDL_WINDOW_MAXIMIZED |  SDL_WINDOW_INPUT_FOCUS);
            gl_context = SDL_GL_CreateContext(window);
            if gl_context.is_null() {
                Self::print_error(String::from("SDL_GL_CreateContext"));
                std::process::exit(1);
            }
            if SDL_GL_MakeCurrent(window, gl_context.clone()) == false {
                println!("Error :");
                std::process::exit(1);
            }
            load_global_gl(&|name: *const u8| {
                let cstr = std::ffi::CStr::from_ptr(name as *const std::ffi::c_char);
                SDL_GL_GetProcAddress(cstr.as_ptr()) as *const std::ffi::c_void
            });
            // XXX remove magic number
            SDL_GL_SetSwapInterval(1);
        }
        SDL {
            window,
            gl_context,
        }
    }
}
