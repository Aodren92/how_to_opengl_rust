pub mod shader;
use nalgebra_glm;
use gl33::global_loader::*;
use crate::sdl;
use crate::camera;


pub fn draw_simple_triangle() -> shader::Shader {

    let vertices = Vec::from([
            -0.5, -0.5,  0.0,
             0.5, -0.5,  0.0,
             0.0,  0.5,  0.0
    ]);

    let mut shader: shader::Shader = shader::Shader{ 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        texture_2:              0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment.frag"),
        texture_src:            None,
        texture_src_2:          None,
        vertices:               vertices,
        strides:                3 * std::mem::size_of::<f32>() as i32,
        strides_color:          0,
        offset_color:           0,
        strides_texture:        0,
        offset_texture:         0,
        opt_indices:            None,
        transform:              None,
        draw:                   None,
    };
    shader.load();
    return shader;
}

pub fn draw_simple_triangle_color() -> shader::Shader {

    let vertices = Vec::from([
            -0.5, -0.5,  0.0,
             0.5, -0.5,  0.0,
             0.0,  0.5,  0.0
    ]);

    let mut shader : shader::Shader = shader::Shader{ 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        texture_2:              0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex_color.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment_color.frag"),
        texture_src:            None,
        texture_src_2:          None,
        vertices:               vertices,
        strides:                3 * std::mem::size_of::<f32>() as i32,
        strides_color:          0,
        offset_color:           0,
        strides_texture:        0,
        offset_texture:         0,
        opt_indices:            None,
        transform:              None,
        draw:                   None,
    };
    shader.load();
    return shader;
}


pub fn draw_simple_rectangle() -> shader::Shader {

    let vertices = Vec::from([
             0.5,  0.5,  0.0,
             0.5, -0.5,  0.0,
            -0.5,  0.5,  0.0,
             0.5, -0.5,  0.0,
            -0.5, -0.5,  0.0,
            -0.5,  0.5,  0.0,
    ]);

    let mut shader: shader::Shader = shader::Shader{ 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        texture_2:              0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment.frag"),
        texture_src:            None,
        texture_src_2:          None,
        vertices:               vertices,
        strides:                3 * std::mem::size_of::<f32>() as i32,
        strides_color:          0,
        offset_color:           0,
        strides_texture:        0,
        offset_texture:         0,
        opt_indices:            None,
        transform:              None,
        draw:                   None,
    };
    shader.load();
    return shader;
}

pub fn draw_simple_rectangle_with_indices() -> shader::Shader {

    let vertices = Vec::from([
            //positions
             0.5,  0.5, 0.0,
             0.5, -0.5, 0.0,
            -0.5, -0.5, 0.0,
            -0.5,  0.5, 0.0,
    ]);

    let indices = Vec::from([
                            0, 1, 3,
                            1, 2, 3
    ]);

    let mut shader: shader::Shader = shader::Shader{ 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        texture_2:              0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment.frag"),
        texture_src:            None,
        texture_src_2:          None,
        vertices:               vertices,
        strides:                3 * std::mem::size_of::<f32>() as i32,
        strides_color:          0,
        offset_color:           0,
        strides_texture:        0,
        offset_texture:         0,
        opt_indices:            Some(indices),
        transform:              None,
        draw:                   None,
    };
    shader.load();
    return shader;
}

pub fn draw_simple_triangle_uniform() -> shader::Shader {

    let vertices = Vec::from([
            //positions
             -0.5, -0.5, 0.0,
              0.5, -0.5, 0.0,
              0.0,  0.5, 0.0,
    ]);


    let mut shader: shader::Shader = shader::Shader { 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        texture_2:              0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment_uniform.frag"),
        texture_src:            None,
        texture_src_2:          None,
        vertices:               vertices,
        strides:                3 * std::mem::size_of::<f32>() as i32,
        strides_color:          0,
        offset_color:           0,
        strides_texture:        0,
        offset_texture:         0,
        opt_indices:            None,
        transform:              Some(simple_color_change),
        draw:                   None,
    };
    shader.load();
    return shader;
}

pub fn draw_simple_triangle_fragment_interpollation() -> shader::Shader {

    let vertices = Vec::from([
            //positions     //colors
             0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 
            -0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
             0.0,  0.5, 0.0, 0.0, 0.0, 1.0,
    ]);


    let mut shader: shader::Shader = shader::Shader { 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        texture_2:              0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex_interpollation.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment_interpollation.frag"),
        texture_src:            None,
        texture_src_2:          None,
        vertices:               vertices,
        strides:                6 * std::mem::size_of::<f32>() as i32,
        strides_color:          6 * std::mem::size_of::<f32>() as i32,
        offset_color:           (3 * std::mem::size_of::<f32>()) as i32,
        strides_texture:        0,
        offset_texture:         0,
        opt_indices:            None,
        draw:                   None,
        transform:              None,
        //r#type:                 shader::TriangleType::UNIFORM,
    };
    shader.load();
    return shader;
}

pub fn draw_simple_triangle_texture() -> shader::Shader {

    let vertices = Vec::from([
            //positions         //colors            // textTures color
             0.5, -0.5, 0.0,    1.0, 0.0, 0.0,      1.0, 1.0,
            -0.5, -0.5, 0.0,    0.0, 1.0, 0.0,      1.0, 0.0,
             0.0,  0.5, 0.0,    0.0, 0.0, 1.0,      0.0, 1.0
    ]);

    let mut shader: shader::Shader = shader::Shader { 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        texture_2:              0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex_texture.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment_texture.frag"),
        texture_src:            Some(String::from("assets/wall.jpg")),
        texture_src_2:          None,
        vertices:               vertices,
        strides:                8 * std::mem::size_of::<f32>() as i32,
        strides_color:          8 * std::mem::size_of::<f32>() as i32,
        offset_color:           (3 * std::mem::size_of::<f32>()) as i32,
        strides_texture:        8 * std::mem::size_of::<f32>() as i32,
        offset_texture:         (6 * std::mem::size_of::<f32>()) as i32,
        opt_indices:            None,
        transform:              None,
        draw:                   None,
        //r#type:                 shader::TriangleType::NORMAL,
    };
    shader.load();
    return shader;
}

pub fn draw_simple_rectangle_texture() -> shader::Shader {

    let vertices = Vec::from([
            //positions         //colors            // textTures color
             0.5,  0.5, 0.0,    1.0, 0.0, 0.0,      1.0, 1.0,
             0.5, -0.5, 0.0,    0.0, 1.0, 0.0,      1.0, 0.0,
            -0.5, -0.5, 0.0,    0.0, 0.0, 1.0,      0.0, 0.0,
            -0.5,  0.5, 0.0,    1.0, 1.0, 0.0,      0.0, 1.0,
    ]);

    let indices = Vec::from([
                0, 1, 3,
                1, 2, 3
    ]);

    let mut shader: shader::Shader = shader::Shader { 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        texture_2:              0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex_texture.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment_texture.frag"),
        texture_src:            Some(String::from("assets/wall.jpg")),
        //texture_src:            Some(String::from("assets/awesomeface.png")),
        texture_src_2:          None,
        vertices:               vertices,
        strides:                8 * std::mem::size_of::<f32>() as i32,
        strides_color:          8 * std::mem::size_of::<f32>() as i32,
        offset_color:           (3 * std::mem::size_of::<f32>()) as i32,
        strides_texture:        8 * std::mem::size_of::<f32>() as i32,
        offset_texture:         (6 * std::mem::size_of::<f32>()) as i32,
        opt_indices:            Some(indices),
        transform:              None,
        draw:                   None,
    };
    shader.load();
    return shader;
}

pub fn draw_simple_rectangle_happy_face_texture() -> shader::Shader {

    let vertices = Vec::from([
            //positions         //colors            // textTures color
             0.5,  0.5, 0.0,    1.0, 0.0, 0.0,      1.0, 1.0,
             0.5, -0.5, 0.0,    0.0, 1.0, 0.0,      1.0, 0.0,
            -0.5, -0.5, 0.0,    0.0, 0.0, 1.0,      0.0, 0.0,
            -0.5,  0.5, 0.0,    1.0, 1.0, 0.0,      0.0, 1.0,
    ]);

    let indices = Vec::from([
                0, 1, 3,
                1, 2, 3
    ]);

    let mut shader: shader::Shader = shader::Shader { 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        texture_2:              0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex_texture.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment_double_texture.frag"),
        texture_src:            Some(String::from("assets/container.jpg")),
        texture_src_2:          Some(String::from("assets/awesomeface.png")),
        vertices:               vertices,
        strides:                8 * std::mem::size_of::<f32>() as i32,
        strides_color:          8 * std::mem::size_of::<f32>() as i32,
        offset_color:           (3 * std::mem::size_of::<f32>()) as i32,
        strides_texture:        8 * std::mem::size_of::<f32>() as i32,
        offset_texture:         (6 * std::mem::size_of::<f32>()) as i32,
        opt_indices:            Some(indices),
        transform:              None,
        draw:                   None,
    };
    shader.load();
    return shader;
}


pub fn draw_simple_rectangle_transform() -> shader::Shader {

    let vertices = Vec::from([
            //positions         //colors            // textTures color
             0.5,  0.5, 0.0,    1.0, 0.0, 0.0,      1.0, 1.0,
             0.5, -0.5, 0.0,    0.0, 1.0, 0.0,      1.0, 0.0,
            -0.5, -0.5, 0.0,    0.0, 0.0, 1.0,      0.0, 0.0,
            -0.5,  0.5, 0.0,    1.0, 1.0, 0.0,      0.0, 1.0,
    ]);

    let indices = Vec::from([
                0, 1, 3,
                1, 2, 3
    ]);


    let mut shader: shader::Shader = shader::Shader { 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        texture_2:              0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex_texture_transform.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment_double_texture.frag"),
        texture_src:            Some(String::from("assets/container.jpg")),
        texture_src_2:          Some(String::from("assets/awesomeface.png")),
        vertices:               vertices,
        strides:                8 * std::mem::size_of::<f32>() as i32,
        strides_color:          8 * std::mem::size_of::<f32>() as i32,
        offset_color:           (3 * std::mem::size_of::<f32>()) as i32,
        strides_texture:        8 * std::mem::size_of::<f32>() as i32,
        offset_texture:         (6 * std::mem::size_of::<f32>()) as i32,
        opt_indices:            Some(indices),
        transform:              Some(simple_transform_and_rotate),
        draw:                   None,
    };
    shader.load();
    return shader;
}

pub fn draw_simple_rectangle_transform_rotate_over_time() -> shader::Shader {

    let vertices = Vec::from([
            //positions         //colors            // textTures color
             0.5,  0.5, 0.0,    1.0, 0.0, 0.0,      1.0, 1.0,
             0.5, -0.5, 0.0,    0.0, 1.0, 0.0,      1.0, 0.0,
            -0.5, -0.5, 0.0,    0.0, 0.0, 1.0,      0.0, 0.0,
            -0.5,  0.5, 0.0,    1.0, 1.0, 0.0,      0.0, 1.0,
    ]);

    let indices = Vec::from([
                0, 1, 3,
                1, 2, 3
    ]);


    let mut shader: shader::Shader = shader::Shader { 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        texture_2:              0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex_texture_transform.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment_double_texture.frag"),
        texture_src:            Some(String::from("assets/container.jpg")),
        texture_src_2:          Some(String::from("assets/awesomeface.png")),
        vertices:               vertices,
        strides:                8 * std::mem::size_of::<f32>() as i32,
        strides_color:          8 * std::mem::size_of::<f32>() as i32,
        offset_color:           (3 * std::mem::size_of::<f32>()) as i32,
        strides_texture:        8 * std::mem::size_of::<f32>() as i32,
        offset_texture:         (6 * std::mem::size_of::<f32>()) as i32,
        opt_indices:            Some(indices),
        transform:              Some(simple_transform_and_rotate_over_time),
        draw:                   None,
    };
    shader.load();
    return shader;
}

pub fn simple_transform_and_rotate_over_time(shader: &shader::Shader, _sdl: &sdl::SDL) {


    let mut trans = nalgebra_glm::Mat4::identity();
    trans = nalgebra_glm::translate(&trans, &nalgebra_glm::vec3(0.5, -0.5, 0.5));

    unsafe {
        let time_value = (sdl::SDL_GetTicks() as f32 / 1000.0) as f32;
        trans = nalgebra_glm::rotate(&trans, time_value, &nalgebra_glm::vec3(0.0, 0.0, 1.0));
        let transform_loc = glGetUniformLocation(shader.shader_program,
                                                 std::ffi::CString::new("transform").unwrap().as_ptr() as *const u8,
                                                 );
        glUniformMatrix4fv(transform_loc, 1, 0, trans.as_ptr());
    }
}

pub fn simple_transform_and_rotate(shader: &shader::Shader, _sdl: &sdl::SDL) {

    let mut trans = nalgebra_glm::Mat4::identity();
    trans = nalgebra_glm::rotate(&trans, 90.0_f32.to_radians(), &nalgebra_glm::vec3(0.0, 0.0, 1.0));
    trans = nalgebra_glm::scale(&trans, &nalgebra_glm::vec3(0.5, 0.5, 0.5));
    unsafe {
        let transform_loc = glGetUniformLocation(shader.shader_program,
                                                 std::ffi::CString::new("transform").unwrap().as_ptr() as *const u8,
                                                 );
        glUniformMatrix4fv(transform_loc, 1, 0, trans.as_ptr());
    }
}

pub fn simple_color_change(shader: &shader::Shader, _sdl: &sdl::SDL) {

    unsafe {
        let time_value = (sdl::SDL_GetTicks() as f32 / 1000.0) as f32;
        let green_value = time_value.sin() / 2.0 + 0.5;
        let vertex_color_location = glGetUniformLocation(shader.shader_program, std::ffi::CString::new("ourColor").unwrap().as_ptr() as *const u8);
        glUniform4f(vertex_color_location, 0.0, green_value as f32, 0.0, 1.0);
    }
}

/******************************************************************************
 **                             3D
******************************************************************************/

pub fn draw_rectangle_on_floor() -> shader::Shader {

    let vertices = Vec::from([
            //positions         //colors            // textTures color
             0.5,  0.5, 0.0,    1.0, 0.0, 0.0,      1.0, 1.0,
             0.5, -0.5, 0.0,    0.0, 1.0, 0.0,      1.0, 0.0,
            -0.5, -0.5, 0.0,    0.0, 0.0, 1.0,      0.0, 0.0,
            -0.5,  0.5, 0.0,    1.0, 1.0, 0.0,      0.0, 1.0,
    ]);

    let indices = Vec::from([
                0, 1, 3,
                1, 2, 3
    ]);


    let mut shader: shader::Shader = shader::Shader { 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        texture_2:              0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex_3D.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment_double_texture.frag"),
        texture_src:            Some(String::from("assets/container.jpg")),
        texture_src_2:          Some(String::from("assets/awesomeface.png")),
        vertices:               vertices,
        strides:                8 * std::mem::size_of::<f32>() as i32,
        strides_color:          8 * std::mem::size_of::<f32>() as i32,
        offset_color:           (3 * std::mem::size_of::<f32>()) as i32,
        strides_texture:        8 * std::mem::size_of::<f32>() as i32,
        offset_texture:         (6 * std::mem::size_of::<f32>()) as i32,
        opt_indices:            Some(indices),
        transform:              Some(transform_on_floor),
        draw:                   None,
    };
    shader.load();
    return shader;
}

pub fn transform_on_floor(shader: &shader::Shader, sdl: &sdl::SDL) {

    let trans = nalgebra_glm::Mat4::identity();
    let model = nalgebra_glm::rotate(&trans, -55.0_f32.to_radians(), &nalgebra_glm::vec3(1.0, 0.0, 0.0));
    let view = nalgebra_glm::translate(&trans, &nalgebra_glm::vec3(0.0, 0.0, -3.0));
    //XXX get size of the screen


    unsafe {
        let mut w: i32 = 0;
        let mut h: i32 = 0;
        sdl::SDL_GetWindowSize(sdl.window, &mut w as *mut i32, &mut h as *mut i32);
        let projection = nalgebra_glm::perspective(45.0_f32.to_radians(), w as f32 / h as f32, 0.1, 100.0);
        let model_loc = glGetUniformLocation(shader.shader_program,
                                                 std::ffi::CString::new("model").unwrap().as_ptr() as *const u8,
                                                 );
        glUniformMatrix4fv(model_loc, 1, 0, model.as_ptr());
        let view_loc = glGetUniformLocation(shader.shader_program,
                                                 std::ffi::CString::new("view").unwrap().as_ptr() as *const u8,
                                                 );
        glUniformMatrix4fv(view_loc, 1, 0, view.as_ptr());
        let projection_loc = glGetUniformLocation(shader.shader_program,
                                                 std::ffi::CString::new("projection").unwrap().as_ptr() as *const u8,
                                                 );
        glUniformMatrix4fv(projection_loc, 1, 0, projection.as_ptr());
    }
}

pub fn draw_cube() -> shader::Shader {

    let vertices = Vec::from([
    -0.5, -0.5, -0.5,  0.0, 0.0,
     0.5, -0.5, -0.5,  1.0, 0.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
    -0.5,  0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 0.0,

    -0.5, -0.5,  0.5,  0.0, 0.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
     0.5,  0.5,  0.5,  1.0, 1.0,
     0.5,  0.5,  0.5,  1.0, 1.0,
    -0.5,  0.5,  0.5,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,

    -0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5,  0.5, -0.5,  1.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,
    -0.5,  0.5,  0.5,  1.0, 0.0,

     0.5,  0.5,  0.5,  1.0, 0.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
     0.5, -0.5, -0.5,  0.0, 1.0,
     0.5, -0.5, -0.5,  0.0, 1.0,
     0.5, -0.5,  0.5,  0.0, 0.0,
     0.5,  0.5,  0.5,  1.0, 0.0,

    -0.5, -0.5, -0.5,  0.0, 1.0,
     0.5, -0.5, -0.5,  1.0, 1.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,

    -0.5,  0.5, -0.5,  0.0, 1.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
     0.5,  0.5,  0.5,  1.0, 0.0,
     0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5,  0.5,  0.5,  0.0, 0.0,
    -0.5,  0.5, -0.5,  0.0, 1.0

    ]);


    let mut shader: shader::Shader = shader::Shader { 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        texture_2:              0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex_3D.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment_double_texture.frag"),
        texture_src:            Some(String::from("assets/container.jpg")),
        texture_src_2:          Some(String::from("assets/awesomeface.png")),
        vertices:               vertices,
        strides:                5 * std::mem::size_of::<f32>() as i32,
        strides_color:          0 as i32,
        offset_color:           0 as i32,
        strides_texture:        5 * std::mem::size_of::<f32>() as i32,
        offset_texture:         (3 * std::mem::size_of::<f32>()) as i32,
        opt_indices:            None,
        transform:              Some(rotate_cube),
        draw:                   None,
    };
    unsafe {
        glEnable(gl33::GL_DEPTH_TEST);
    }
    shader.load();
    return shader;
}

pub fn rotate_cube(shader: &shader::Shader, sdl: &sdl::SDL) {


    let trans = nalgebra_glm::Mat4::identity();
    let model = nalgebra_glm::rotate(&trans, -55.0_f32.to_radians(), &nalgebra_glm::vec3(1.0, 0.0, 0.0));
    let view = nalgebra_glm::translate(&trans, &nalgebra_glm::vec3(0.0, 0.0, -3.0));
    
    //XXX get size of the screen

    unsafe {
        let time_value = (sdl::SDL_GetTicks() as f32 / 1000.0) as f32;
        let model = nalgebra_glm::rotate(&model, time_value * 50.0_f32.to_radians(), &nalgebra_glm::vec3(0.5, 1.0, 0.0));
        let mut w: i32 = 0;
        let mut h: i32 = 0;
        sdl::SDL_GetWindowSize(sdl.window, &mut w as *mut i32, &mut h as *mut i32);
        let projection = nalgebra_glm::perspective(45.0_f32.to_radians(), w as f32 / h as f32, 0.1, 100.0);
        let model_loc = glGetUniformLocation(shader.shader_program,
                                                 std::ffi::CString::new("model").unwrap().as_ptr() as *const u8,
                                                 );
        glUniformMatrix4fv(model_loc, 1, 0, model.as_ptr());
        let view_loc = glGetUniformLocation(shader.shader_program,
                                                 std::ffi::CString::new("view").unwrap().as_ptr() as *const u8,
                                                 );
        glUniformMatrix4fv(view_loc, 1, 0, view.as_ptr());
        let projection_loc = glGetUniformLocation(shader.shader_program,
                                                 std::ffi::CString::new("projection").unwrap().as_ptr() as *const u8,
                                                 );
        glUniformMatrix4fv(projection_loc, 1, 0, projection.as_ptr());
    }
}

pub fn draw_10_cubes() -> shader::Shader {

    let vertices = Vec::from([
    -0.5, -0.5, -0.5,  0.0, 0.0,
     0.5, -0.5, -0.5,  1.0, 0.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
    -0.5,  0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 0.0,

    -0.5, -0.5,  0.5,  0.0, 0.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
     0.5,  0.5,  0.5,  1.0, 1.0,
     0.5,  0.5,  0.5,  1.0, 1.0,
    -0.5,  0.5,  0.5,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,

    -0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5,  0.5, -0.5,  1.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,
    -0.5,  0.5,  0.5,  1.0, 0.0,

     0.5,  0.5,  0.5,  1.0, 0.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
     0.5, -0.5, -0.5,  0.0, 1.0,
     0.5, -0.5, -0.5,  0.0, 1.0,
     0.5, -0.5,  0.5,  0.0, 0.0,
     0.5,  0.5,  0.5,  1.0, 0.0,

    -0.5, -0.5, -0.5,  0.0, 1.0,
     0.5, -0.5, -0.5,  1.0, 1.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,

    -0.5,  0.5, -0.5,  0.0, 1.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
     0.5,  0.5,  0.5,  1.0, 0.0,
     0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5,  0.5,  0.5,  0.0, 0.0,
    -0.5,  0.5, -0.5,  0.0, 1.0

    ]);


    let mut shader: shader::Shader = shader::Shader { 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        texture_2:              0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex_3D.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment_double_texture.frag"),
        texture_src:            Some(String::from("assets/container.jpg")),
        texture_src_2:          Some(String::from("assets/awesomeface.png")),
        vertices:               vertices,
        strides:                5 * std::mem::size_of::<f32>() as i32,
        strides_color:          0 as i32,
        offset_color:           0 as i32,
        strides_texture:        5 * std::mem::size_of::<f32>() as i32,
        offset_texture:         (3 * std::mem::size_of::<f32>()) as i32,
        opt_indices:            None,
        transform:              None,
        draw:                   Some(draw_10_cubes_),
    };
    unsafe {
        glEnable(gl33::GL_DEPTH_TEST);
    }
    shader.load();
    return shader;
}


pub fn draw_10_cubes_(shader: &mut shader::Shader, sdl: &sdl::SDL, _camera: &camera::Camera) {
    let cube_positions = [
        nalgebra_glm::vec3( 0.0,  0.0,  0.0), 
        nalgebra_glm::vec3( 2.0,  5.0, -15.0), 
        nalgebra_glm::vec3(-1.5, -2.2, -2.5),  
        nalgebra_glm::vec3(-3.8, -2.0, -12.3),  
        nalgebra_glm::vec3( 2.4, -0.4, -3.5),  
        nalgebra_glm::vec3(-1.7,  3.0, -7.5),  
        nalgebra_glm::vec3( 1.3, -2.0, -2.5),  
        nalgebra_glm::vec3( 1.5,  2.0, -2.5), 
        nalgebra_glm::vec3( 1.5,  0.2, -1.5), 
        nalgebra_glm::vec3(-1.3,  1.0, -1.5)  
    ];

    let trans = nalgebra_glm::Mat4::identity();
   // let model = nalgebra_glm::rotate(&trans, -55.0_f32.to_radians(), &nalgebra_glm::vec3(1.0, 0.0, 0.0));
    let view = nalgebra_glm::translate(&trans, &nalgebra_glm::vec3(0.0, 0.0, -3.0));
    for i in 0..10 {
        unsafe {
            //glm::mat4 model = glm::mat4(1.0f);
            //float angle = 20.0f * i; 
            //model = glm::rotate(model, glm::radians(angle), glm::vec3(1.0f, 0.3f, 0.5f));
            //ourShader.setMat4("model", model);


            //XXX get size of the screen

            let model = nalgebra_glm::translate(&trans, &cube_positions[i]);
            let model = nalgebra_glm::rotate(&model, i as f32 * 20.0_f32.to_radians(), &nalgebra_glm::vec3(0.5, 1.0, 0.0));
            let mut w: i32 = 0;
            let mut h: i32 = 0;

            sdl::SDL_GetWindowSize(sdl.window, &mut w as *mut i32, &mut h as *mut i32);

            let projection = nalgebra_glm::perspective(45.0_f32.to_radians(), w as f32 / h as f32, 0.1, 100.0);


            let model_loc = glGetUniformLocation(shader.shader_program,
                                                 std::ffi::CString::new("model").unwrap().as_ptr() as *const u8,
                                                 );
            glUniformMatrix4fv(model_loc, 1, 0, model.as_ptr());
            let view_loc = glGetUniformLocation(shader.shader_program,
                                                std::ffi::CString::new("view").unwrap().as_ptr() as *const u8,
                                                );
            glUniformMatrix4fv(view_loc, 1, 0, view.as_ptr());
            let projection_loc = glGetUniformLocation(shader.shader_program,
                                                      std::ffi::CString::new("projection").unwrap().as_ptr() as *const u8,
                                                      );
            glUniformMatrix4fv(projection_loc, 1, 0, projection.as_ptr());
            glDrawArrays(gl33::GL_TRIANGLES, 0, shader.vertices.len() as i32)
        }
    }
}

pub fn draw_10_cubes_rotate() -> shader::Shader {

    let vertices = Vec::from([
    -0.5, -0.5, -0.5,  0.0, 0.0,
     0.5, -0.5, -0.5,  1.0, 0.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
    -0.5,  0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 0.0,

    -0.5, -0.5,  0.5,  0.0, 0.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
     0.5,  0.5,  0.5,  1.0, 1.0,
     0.5,  0.5,  0.5,  1.0, 1.0,
    -0.5,  0.5,  0.5,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,

    -0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5,  0.5, -0.5,  1.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,
    -0.5,  0.5,  0.5,  1.0, 0.0,

     0.5,  0.5,  0.5,  1.0, 0.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
     0.5, -0.5, -0.5,  0.0, 1.0,
     0.5, -0.5, -0.5,  0.0, 1.0,
     0.5, -0.5,  0.5,  0.0, 0.0,
     0.5,  0.5,  0.5,  1.0, 0.0,

    -0.5, -0.5, -0.5,  0.0, 1.0,
     0.5, -0.5, -0.5,  1.0, 1.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,

    -0.5,  0.5, -0.5,  0.0, 1.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
     0.5,  0.5,  0.5,  1.0, 0.0,
     0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5,  0.5,  0.5,  0.0, 0.0,
    -0.5,  0.5, -0.5,  0.0, 1.0

    ]);


    let mut shader: shader::Shader = shader::Shader { 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        texture_2:              0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex_3D.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment_double_texture.frag"),
        texture_src:            Some(String::from("assets/container.jpg")),
        texture_src_2:          Some(String::from("assets/awesomeface.png")),
        vertices:               vertices,
        strides:                5 * std::mem::size_of::<f32>() as i32,
        strides_color:          0 as i32,
        offset_color:           0 as i32,
        strides_texture:        5 * std::mem::size_of::<f32>() as i32,
        offset_texture:         (3 * std::mem::size_of::<f32>()) as i32,
        opt_indices:            None,
        transform:              None,
        draw:                   Some(draw_10_cubes_rotate_),
    };
    unsafe {
        glEnable(gl33::GL_DEPTH_TEST);
    }
    shader.load();
    return shader;
}


pub fn draw_10_cubes_rotate_(shader: &mut shader::Shader, sdl: &sdl::SDL, _camera: &camera::Camera) {
    let cube_positions = [
        nalgebra_glm::vec3( 0.0,  0.0,  0.0), 
        nalgebra_glm::vec3( 2.0,  5.0, -15.0), 
        nalgebra_glm::vec3(-1.5, -2.2, -2.5),  
        nalgebra_glm::vec3(-3.8, -2.0, -12.3),  
        nalgebra_glm::vec3( 2.4, -0.4, -3.5),  
        nalgebra_glm::vec3(-1.7,  3.0, -7.5),  
        nalgebra_glm::vec3( 1.3, -2.0, -2.5),  
        nalgebra_glm::vec3( 1.5,  2.0, -2.5), 
        nalgebra_glm::vec3( 1.5,  0.2, -1.5), 
        nalgebra_glm::vec3(-1.3,  1.0, -1.5)  
    ];

    let identity = nalgebra_glm::Mat4::identity();

    let radius: f32 = 5.0;

    unsafe {
        let cam_x = (sdl::SDL_GetTicks() as f32 / 1000.0) as f32;
        let cam_z = (sdl::SDL_GetTicks() as f32 / 1000.0) as f32;

        


        let eye = nalgebra_glm::vec3(cam_x.sin() * radius, 0.0, cam_z.cos() * radius) ;
        let center = nalgebra_glm::vec3(0.0, 0.0, 0.0);
        let up = nalgebra_glm::vec3(0.0, 1.0, 0.0);

        let view = nalgebra_glm::look_at(&eye, &center, &up);
        let view_loc = glGetUniformLocation(shader.shader_program,
                                            std::ffi::CString::new("view").unwrap().as_ptr() as *const u8,
                                            );
        glUniformMatrix4fv(view_loc, 1, 0, view.as_ptr());
    }

    for i in 0..10 {
        unsafe {

            //XXX get size of the screen

            let model = nalgebra_glm::translate(&identity, &cube_positions[i]);
            let model = nalgebra_glm::rotate(&model, i as f32 * 20.0_f32.to_radians(), &nalgebra_glm::vec3(0.5, 1.0, 0.0));
            let model = nalgebra_glm::rotate(&model, i as f32 * 20.0_f32.to_radians(), &nalgebra_glm::vec3(1.0, 0.3, 0.5));
            let mut w: i32 = 0;
            let mut h: i32 = 0;

            sdl::SDL_GetWindowSize(sdl.window, &mut w as *mut i32, &mut h as *mut i32);

            let projection = nalgebra_glm::perspective(45.0_f32.to_radians(), w as f32 / h as f32, 0.1, 100.0);


            let model_loc = glGetUniformLocation(shader.shader_program,
                                             std::ffi::CString::new("model").unwrap().as_ptr() as *const u8,
                                                 );
            glUniformMatrix4fv(model_loc, 1, 0, model.as_ptr());
            let projection_loc = glGetUniformLocation(shader.shader_program,
                                                      std::ffi::CString::new("projection").unwrap().as_ptr() as *const u8,
                                                      );
            glUniformMatrix4fv(projection_loc, 1, 0, projection.as_ptr());
            glDrawArrays(gl33::GL_TRIANGLES, 0, shader.vertices.len() as i32)
        }
    }
}

pub fn draw_10_cubes_move_camera() -> shader::Shader {

    let vertices = Vec::from([
    -0.5, -0.5, -0.5,  0.0, 0.0,
     0.5, -0.5, -0.5,  1.0, 0.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
    -0.5,  0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 0.0,

    -0.5, -0.5,  0.5,  0.0, 0.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
     0.5,  0.5,  0.5,  1.0, 1.0,
     0.5,  0.5,  0.5,  1.0, 1.0,
    -0.5,  0.5,  0.5,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,

    -0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5,  0.5, -0.5,  1.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,
    -0.5,  0.5,  0.5,  1.0, 0.0,

     0.5,  0.5,  0.5,  1.0, 0.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
     0.5, -0.5, -0.5,  0.0, 1.0,
     0.5, -0.5, -0.5,  0.0, 1.0,
     0.5, -0.5,  0.5,  0.0, 0.0,
     0.5,  0.5,  0.5,  1.0, 0.0,

    -0.5, -0.5, -0.5,  0.0, 1.0,
     0.5, -0.5, -0.5,  1.0, 1.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,

    -0.5,  0.5, -0.5,  0.0, 1.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
     0.5,  0.5,  0.5,  1.0, 0.0,
     0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5,  0.5,  0.5,  0.0, 0.0,
    -0.5,  0.5, -0.5,  0.0, 1.0

    ]);


    let mut shader: shader::Shader = shader::Shader { 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        texture_2:              0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex_3D.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment_double_texture.frag"),
        texture_src:            Some(String::from("assets/container.jpg")),
        texture_src_2:          Some(String::from("assets/awesomeface.png")),
        vertices:               vertices,
        strides:                5 * std::mem::size_of::<f32>() as i32,
        strides_color:          0 as i32,
        offset_color:           0 as i32,
        strides_texture:        5 * std::mem::size_of::<f32>() as i32,
        offset_texture:         (3 * std::mem::size_of::<f32>()) as i32,
        opt_indices:            None,
        transform:              None,
        draw:                   Some(draw_10_cubes_move_camera_),
    };
    unsafe {
        glEnable(gl33::GL_DEPTH_TEST);
    }
    shader.load();
    return shader;
}


pub fn draw_10_cubes_move_camera_(shader: &mut shader::Shader, sdl: &sdl::SDL, camera: &camera::Camera) {
    let cube_positions = [
        nalgebra_glm::vec3( 0.0,  0.0,  0.0), 
        nalgebra_glm::vec3( 2.0,  5.0, -15.0), 
        nalgebra_glm::vec3(-1.5, -2.2, -2.5),  
        nalgebra_glm::vec3(-3.8, -2.0, -12.3),  
        nalgebra_glm::vec3( 2.4, -0.4, -3.5),
        nalgebra_glm::vec3(-1.7,  3.0, -7.5),
        nalgebra_glm::vec3( 1.3, -2.0, -2.5),
        nalgebra_glm::vec3( 1.5,  2.0, -2.5),
        nalgebra_glm::vec3( 1.5,  0.2, -1.5),
        nalgebra_glm::vec3(-1.3,  1.0, -1.5)
    ];

    let identity = nalgebra_glm::Mat4::identity();



    unsafe {
        let center = camera.camera_pos + camera.camera_front;
        let view = nalgebra_glm::look_at(&camera.camera_pos, &center, &camera.camera_up);

        let view_loc = glGetUniformLocation(shader.shader_program,
                                            std::ffi::CString::new("view").unwrap().as_ptr() as *const u8,
                                            );
        glUniformMatrix4fv(view_loc, 1, 0, view.as_ptr());
    }


    for i in 0..10 {
        unsafe {

            //XXX get size of the screen

            let model = nalgebra_glm::translate(&identity, &cube_positions[i]);
            let model = nalgebra_glm::rotate(&model, i as f32 * 20.0_f32.to_radians(), &nalgebra_glm::vec3(0.5, 1.0, 0.0));
            let model = nalgebra_glm::rotate(&model, i as f32 * 20.0_f32.to_radians(), &nalgebra_glm::vec3(1.0, 0.3, 0.5));
            let mut w: i32 = 0;
            let mut h: i32 = 0;

            sdl::SDL_GetWindowSize(sdl.window, &mut w as *mut i32, &mut h as *mut i32);

            let projection = nalgebra_glm::perspective(45.0_f32.to_radians(), w as f32 / h as f32, 0.1, 100.0);


            let model_loc = glGetUniformLocation(shader.shader_program,
                                             std::ffi::CString::new("model").unwrap().as_ptr() as *const u8,
                                                 );
            glUniformMatrix4fv(model_loc, 1, 0, model.as_ptr());
            let projection_loc = glGetUniformLocation(shader.shader_program,
                                                      std::ffi::CString::new("projection").unwrap().as_ptr() as *const u8,
                                                      );
            glUniformMatrix4fv(projection_loc, 1, 0, projection.as_ptr());
            glDrawArrays(gl33::GL_TRIANGLES, 0, shader.vertices.len() as i32)
        }
    }
}
