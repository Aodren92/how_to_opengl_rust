mod sdl;
mod opengl;
use gl33::global_loader::*;
use gl33::GL_TRIANGLES;


fn print_help(exit: i32) -> ! {
    let str = r"
        usage:
            1: print simple triangle
            2: print simple rectangle
            3: print simple rectangle but with EBO(Element Buffer Object)
        
            help:
                print this help

    ";

    println!("{str}");
    std::process::exit(exit);
}


fn main() {
    let sdl = sdl::SDL::init(sdl::SDL_INIT_EVERYTHING);
    let triangle: opengl::GlTriangle;
    match std::env::args().nth(1) {
        Some(arg) => {
                match arg.as_str() {
                    "1"    => {
                        triangle = opengl::draw_simple_triangle();
                    },
                    "2"    => {
                        triangle = opengl::draw_simple_rectangle();
                    },
                    "3"    => {
                        triangle = opengl::draw_simple_rectangle_with_indices();
                    },
                    "4"    => {
                        triangle = opengl::draw_simple_triangle_color();
                    },
                    "help" => {
                        print_help(0);
                    }
                    _ => { 
                        print_help(1);
                    }
                }
        }
        None => {
            print_help(1);
        }
    }

loop {

        unsafe {
            let mut raw = std::mem::MaybeUninit::uninit();
            if sdl::event::SDL_PollEvent(raw.as_mut_ptr()) == true {
                sdl::event::parse_event(raw.assume_init());
            }
            glClear(gl33::GL_COLOR_BUFFER_BIT);
            glClearColor(0.9, 0.3, 0.5, 0.5);

            match &triangle.opt_indices {
                Some(indices) => {
                    glDrawElements(GL_TRIANGLES, indices.len() as i32, gl33::GL_UNSIGNED_INT, std::ptr::null());

                },
                None => glDrawArrays(GL_TRIANGLES, 0, triangle.vertices.len() as i32)

            }

            sdl::SDL_GL_SwapWindow(sdl.window);
            sdl::SDL_Delay(20);
        }
    }
    // XXX destroy window
    // XXX sdl quit
}
