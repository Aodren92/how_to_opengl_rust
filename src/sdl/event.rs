use crate::sdl;


pub const SDL_EVENT_QUIT: u32 = 0x100; 
pub const SDL_EVENT_KEY_DOWN : u32 = 0x300; 

pub const SDLK_RIGHT:   u32 = 0x4000004f; /**< SDL_SCANCODE_TO_KEYCODE(SDL_SCANCODE_RIGHT) */
pub const SDLK_LEFT:    u32 = 0x40000050; /**< SDL_SCANCODE_TO_KEYCODE(SDL_SCANCODE_LEFT) */
pub const SDLK_DOWN:    u32 = 0x40000051; /**< SDL_SCANCODE_TO_KEYCODE(SDL_SCANCODE_DOWN) */
pub const SDLK_UP:      u32 = 0x40000052; /**< SDL_SCANCODE_TO_KEYCODE(SDL_SCANCODE_UP) */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDL_KeyboardEvent
{
    pub type_:          u32,
    pub reserved:       u32,
    pub timestamp:      u64,
    pub window_id:      sdl::SDLWindowID,
    pub which:          sdl::SDLKeyboardID,
    pub scancode:       sdl::SDLScancode,
    pub key:            sdl::SDLKeycode,
    pub r#mod:          sdl::SDLKeymod,
    raw:                u16,
    down:               bool,
    repeat:             bool,
}


#[repr(C)]
pub union SDL_Event {
    pub type_:          u32,
    pub keyboard_event: SDL_KeyboardEvent, 
    pub padding:        [u8; 128],
}

#[link(name = "SDL3")]
unsafe extern "C" {
    pub fn SDL_PollEvent(event: *mut SDL_Event) -> bool;
}

impl SDL_Event {
    pub fn new() -> *mut SDL_Event {
        &mut SDL_Event {
            type_ : 0,
        }
    }
}



pub fn parse_event(event: SDL_Event) -> Option<sdl::SDLKeycode> {
    let event_type = unsafe { event.type_ };

    if event_type == SDL_EVENT_QUIT {
        std::process::exit(1);
    } else if event_type == SDL_EVENT_KEY_DOWN {
        let sdl_keyboard_event: SDL_KeyboardEvent = unsafe { event.keyboard_event };
        return match sdl_keyboard_event.key {
            SDLK_UP     => {
                println!("SDLK_UP");
                Some(SDLK_UP)
            },
            SDLK_DOWN   => {
                println!("SDLK_DOWN");
                Some(SDLK_DOWN)
            },
            SDLK_LEFT   => {
                println!("SDLK_LEFT");
                Some(SDLK_LEFT)
            },
            SDLK_RIGHT  => {
                println!("SDLK_RIGHT");
                Some(SDLK_RIGHT)
            },
            _           => None,
        };
    } 
    None
}
