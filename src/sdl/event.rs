use crate::sdl;


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

pub fn parse_event(event: SDL_Event) {
    let event_type = unsafe { event.type_ };

    if event_type == 0x100 {
        std::process::exit(1);
    } else if event_type == 0x300 {
        let _sdl_keyboard_event_: SDL_KeyboardEvent = unsafe { event.keyboard_event };
    }
}
