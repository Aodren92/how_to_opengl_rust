mod sdl;
mod opengl;
use gl33::global_loader::*;
use gl33::GL_TRIANGLES;

fn main() {
    let sdl = sdl::SDL::init(sdl::SDL_INIT_EVERYTHING);
    loop {

        opengl::triangle::draw_triangle();
        unsafe {
            let mut raw = std::mem::MaybeUninit::uninit();
            if sdl::event::SDL_PollEvent(raw.as_mut_ptr()) == true {
                sdl::event::parse_event(raw.assume_init());
            }

            glClear(gl33::GL_COLOR_BUFFER_BIT);
            glClearColor(0.9, 0.3, 0.5, 0.5);
            glDrawArrays(GL_TRIANGLES, 0, 3);
            sdl::SDL_GL_SwapWindow(sdl.window);
            sdl::SDL_Delay(20);
        }
    }
    // XXX destroy window
    // XXX sdl quit
}

