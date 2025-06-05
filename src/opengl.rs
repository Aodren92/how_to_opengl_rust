pub mod shader;
use nalgebra_glm;
use gl33::global_loader::*;
use crate::sdl;

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
};
    shader.load();
    return shader;
}

pub fn simple_transform_and_rotate(shader: &shader::Shader) {

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

pub fn simple_color_change(shader: &shader::Shader) {

    unsafe {
        let time_value = (sdl::SDL_GetTicks() as f32 / 1000.0) as f32;
        let green_value = time_value.sin() / 2.0 + 0.5;
        let vertex_color_location = glGetUniformLocation(shader.shader_program, std::ffi::CString::new("ourColor").unwrap().as_ptr() as *const u8);
        glUniform4f(vertex_color_location, 0.0, green_value as f32, 0.0, 1.0);
    }
}
