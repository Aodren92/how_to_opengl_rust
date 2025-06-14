mod sdl;
mod opengl;
mod camera;


fn print_help(exit: i32) -> ! {
    let str = r"
        usage:
            1:  print simple triangle
            2:  print simple rectangle
            3:  print simple rectangle but with EBO(Element Buffer Object)
            4:  print simple triangle color
            5:  print simple triangle using uniform
            6:  print simple triangle fragment interpollation
            7:  print simple triangle texture
            8:  print simple rectangle texture
            9:  print simple rectangle with 2 textures overlaping
            10: print simple rectangle with a 90rad rotation
            11: print simple rectangle with a 90rad rotation over time
            12: print simple rectangle as floor
            13: print a rotating cube
            14: draw 10 cubes
            15: draw 10 cubes rotate
            help:
                print this help
    ";
    println!("{str}");
    std::process::exit(exit);
}


fn main() {
    let sdl = sdl::SDL::init(sdl::SDL_INIT_EVERYTHING);
    let mut shader: opengl::shader::Shader;

    let funcs: [fn() -> opengl::shader::Shader; 16] = [
        opengl::draw_simple_triangle,
        opengl::draw_simple_rectangle,
        opengl::draw_simple_rectangle_with_indices,
        opengl::draw_simple_triangle_color,
        opengl::draw_simple_triangle_uniform,
        opengl::draw_simple_triangle_fragment_interpollation,
        opengl::draw_simple_triangle_texture,
        opengl::draw_simple_rectangle_texture,
        opengl::draw_simple_rectangle_happy_face_texture,
        opengl::draw_simple_rectangle_transform,
        opengl::draw_simple_rectangle_transform_rotate_over_time,
        opengl::draw_rectangle_on_floor,
        opengl::draw_cube,
        opengl::draw_10_cubes,
        opengl::draw_10_cubes_rotate,
        opengl::draw_10_cubes_move_camera,
    ];

    match std::env::args().nth(1) {
        Some(arg) => {
            match arg.parse::<usize>() {
                Ok(index) => {
                    if index > 0 && index <= funcs.len() {
                        shader = funcs[index - 1]();
                    } else {
                        print_help(1);
                    }
                },
                Err(_) => {
                    match arg.as_str() {
                        "help"  => {
                            print_help(0);
                        }
                        _       => { 
                            print_help(1);
                        }
                    }
                }
            }
        }
        None => {
            print_help(1);
        }
    }

let mut camera = camera::Camera::init();
let mut delta_time;
let mut last_frame = 0.0;

loop {

    let current_frame = unsafe { sdl::SDL_GetTicks() as f32 / 1000.0 } as f32;
    delta_time = current_frame - last_frame;
    last_frame = current_frame;
        unsafe {
            let mut raw = std::mem::MaybeUninit::uninit();
            if sdl::event::SDL_PollEvent(raw.as_mut_ptr()) == true {
                let event = sdl::event::parse_event(raw.assume_init());
                println!("{event:?}");
                camera.update(event, delta_time as f32);
            }
            shader.draw(&sdl, &camera);
            sdl::SDL_GL_SwapWindow(sdl.window);
            sdl::SDL_Delay(20);
        }
    }
    // XXX destroy window
    // XXX sdl quit
}
