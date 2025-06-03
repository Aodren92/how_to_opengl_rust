pub mod triangle;
use crate::sdl;

use gl33::*;
use gl33::global_loader::*;
use gl33::GL_TRIANGLES;


enum TriangleType {
    NORMAL,
    UNIFORM,
}

pub struct GlTriangle {
    pub vao:                    u32,
    pub vbo:                    u32,
    pub texture:                u32,
    pub shader_program:         u32,
    pub vertex_shader_src:      String,
    pub fragment_shader_src:    String,
    pub texture_src:            Option<String>,
    pub vertices:               Vec<f32>,
    pub strides:                i32,
    pub strides_color:          i32,
    pub offset_color:           i32,
    pub strides_texture:        i32,
    pub offset_texture:         i32,
    pub opt_indices:            Option<Vec<u32>>,
    r#type:                     TriangleType,
}

impl GlTriangle {
    pub fn draw(&self) {
        match self.r#type {
            TriangleType::UNIFORM   => {
                unsafe {
                    let time_value = (sdl::SDL_GetTicks() as f32 / 1000.0) as f32;
                    let green_value = time_value.sin() / 2.0 + 0.5;
                    let vertex_color_location = glGetUniformLocation(self.shader_program, std::ffi::CString::new("ourColor").unwrap().as_ptr() as *const u8);
                    glUniform4f(vertex_color_location, 0.0, green_value as f32, 0.0, 1.0);
                }
            },
            TriangleType::NORMAL    => {

            },
        }
        if self.texture != 0 {
            unsafe {
                glBindTexture(GL_TEXTURE_2D, self.texture);    
            }
        }
        glBindVertexArray(self.vao);
        match &self.opt_indices {
            Some(indices) => {
                unsafe {
                    glDrawElements(GL_TRIANGLES, indices.len() as i32, gl33::GL_UNSIGNED_INT, std::ptr::null());
                }
            },
            None => {
                unsafe {
                    glDrawArrays(GL_TRIANGLES, 0, self.vertices.len() as i32)
                }
            }
        }
    }
}

pub fn draw_simple_triangle() -> GlTriangle {

    let mut vertices = Vec::new();
    
    vertices.push(-0.5);
    vertices.push(-0.5);
    vertices.push(0.0);
    vertices.push(0.5);
    vertices.push(-0.5);
    vertices.push(0.0);
    vertices.push(0.0);
    vertices.push(0.5);
    vertices.push(0.0);


    let mut gl_triangle: GlTriangle = GlTriangle{ 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment.frag"),
        texture_src:            None,
        vertices:               vertices,
        strides:                3 * std::mem::size_of::<f32>() as i32,
        strides_color:          0,
        offset_color:           0,
        strides_texture:        0,
        offset_texture:         0,
        opt_indices:            None,
        r#type:                 TriangleType::NORMAL,
    };
    triangle::draw_triangle(&mut gl_triangle);
    return gl_triangle;
}

pub fn draw_simple_triangle_color() -> GlTriangle {

    let mut vertices = Vec::new();
    
    vertices.push(-0.5);
    vertices.push(-0.5);
    vertices.push(0.0);
    vertices.push(0.5);
    vertices.push(-0.5);
    vertices.push(0.0);
    vertices.push(0.0);
    vertices.push(0.5);
    vertices.push(0.0);

    let mut gl_triangle: GlTriangle = GlTriangle{ 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex_color.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment_color.frag"),
        texture_src:            None,
        vertices:               vertices,
        strides:                3 * std::mem::size_of::<f32>() as i32,
        strides_color:          0,
        offset_color:           0,
        strides_texture:        0,
        offset_texture:         0,
        opt_indices:            None,
        r#type:                 TriangleType::NORMAL,
    };
    triangle::draw_triangle(&mut gl_triangle);
    return gl_triangle;
}


pub fn draw_simple_rectangle() -> GlTriangle {

    let mut vertices = Vec::new();

    vertices.push(0.5);
    vertices.push(0.5);
    vertices.push(0.0);
    vertices.push(0.5);
    vertices.push(-0.5);
    vertices.push(0.0);
    vertices.push(-0.5);
    vertices.push(0.5);
    vertices.push(0.0);

    vertices.push(0.5);
    vertices.push(-0.5);
    vertices.push(0.0);
    vertices.push(-0.5);
    vertices.push(-0.5);
    vertices.push(0.0);
    vertices.push(-0.5);
    vertices.push(0.5);
    vertices.push(0.0);


    let mut gl_triangle: GlTriangle = GlTriangle{ 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment.frag"),
        texture_src:            None,
        vertices:               vertices,
        strides:                3 * std::mem::size_of::<f32>() as i32,
        strides_color:          0,
        offset_color:           0,
        strides_texture:        0,
        offset_texture:         0,
        opt_indices:            None,
        r#type:                 TriangleType::NORMAL,
    };
    triangle::draw_triangle(&mut gl_triangle);
    return gl_triangle;
}

pub fn draw_simple_rectangle_with_indices() -> GlTriangle {


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

    let mut gl_triangle: GlTriangle = GlTriangle{ 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment.frag"),
        texture_src:            None,
        vertices:               vertices,
        strides:                3 * std::mem::size_of::<f32>() as i32,
        strides_color:          0,
        offset_color:           0,
        strides_texture:        0,
        offset_texture:         0,
        opt_indices:            Some(indices),
        r#type:                 TriangleType::NORMAL,
    };
    triangle::draw_triangle(&mut gl_triangle);
    return gl_triangle;
}

pub fn draw_simple_triangle_uniform() -> GlTriangle {

    let mut vertices = Vec::new();
    
    vertices.push(-0.5);
    vertices.push(-0.5);
    vertices.push(0.0);
    vertices.push(0.5);
    vertices.push(-0.5);
    vertices.push(0.0);
    vertices.push(0.0);
    vertices.push(0.5);
    vertices.push(0.0);


    let mut gl_triangle: GlTriangle = GlTriangle{ 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment_uniform.frag"),
        texture_src:            None,
        vertices:               vertices,
        strides:                3 * std::mem::size_of::<f32>() as i32,
        strides_color:          0,
        offset_color:           0,
        strides_texture:        0,
        offset_texture:         0,
        opt_indices:            None,
        r#type:                 TriangleType::UNIFORM,
    };
    triangle::draw_triangle(&mut gl_triangle);
    return gl_triangle;
}

pub fn draw_simple_triangle_fragment_interpollation() -> GlTriangle {

    let vertices = Vec::from([
            //positions     //colors
             0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 
            -0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
             0.0,  0.5, 0.0, 0.0, 0.0, 1.0,
    ]);


    let mut gl_triangle: GlTriangle = GlTriangle{ 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex_interpollation.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment_interpollation.frag"),
        texture_src:            None,
        vertices:               vertices,
        strides:                6 * std::mem::size_of::<f32>() as i32,
        strides_color:          6 * std::mem::size_of::<f32>() as i32,
        offset_color:           (3 * std::mem::size_of::<f32>()) as i32,
        strides_texture:        0,
        offset_texture:         0,
        opt_indices:            None,
        r#type:                 TriangleType::UNIFORM,
    };
    triangle::draw_triangle(&mut gl_triangle);
    return gl_triangle;
}

pub fn draw_simple_triangle_texture() -> GlTriangle {

    let vertices = Vec::from([
            //positions         //colors            // textTures color
             0.5, -0.5, 0.0,    1.0, 0.0, 0.0,      1.0, 1.0,
            -0.5, -0.5, 0.0,    0.0, 1.0, 0.0,      1.0, 0.0,
             0.0,  0.5, 0.0,    0.0, 0.0, 1.0,      0.0, 1.0
    ]);

    let mut gl_triangle: GlTriangle = GlTriangle{ 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex_texture.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment_texture.frag"),
        texture_src:            Some(String::from("assets/wall.jpg")),
        vertices:               vertices,
        strides:                8 * std::mem::size_of::<f32>() as i32,
        strides_color:          8 * std::mem::size_of::<f32>() as i32,
        offset_color:           (3 * std::mem::size_of::<f32>()) as i32,
        strides_texture:        8 * std::mem::size_of::<f32>() as i32,
        offset_texture:         (6 * std::mem::size_of::<f32>()) as i32,
        opt_indices:            None,
        r#type:                 TriangleType::NORMAL,
    };
    triangle::draw_triangle(&mut gl_triangle);
    return gl_triangle;
}

pub fn draw_simple_rectangle_texture() -> GlTriangle {

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

    let mut gl_triangle: GlTriangle = GlTriangle{ 
        vao:                    0,
        vbo:                    0,
        texture:                0,
        shader_program:         0,
        vertex_shader_src:      String::from("shader/simple_vertex_texture.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment_texture.frag"),
        texture_src:            Some(String::from("assets/wall.jpg")),
        vertices:               vertices,
        strides:                8 * std::mem::size_of::<f32>() as i32,
        strides_color:          8 * std::mem::size_of::<f32>() as i32,
        offset_color:           (3 * std::mem::size_of::<f32>()) as i32,
        strides_texture:        8 * std::mem::size_of::<f32>() as i32,
        offset_texture:         (6 * std::mem::size_of::<f32>()) as i32,
        opt_indices:            Some(indices),
        r#type:                 TriangleType::NORMAL,
    };
    triangle::draw_triangle(&mut gl_triangle);
    return gl_triangle;
}

fn load_shader(src: &str) -> String {

    std::fs::read_to_string(src).expect(format!("Failed to read shader file {src}").as_str())
}


fn compile_shader(str: String, r#type: ShaderType) -> u32 {
        
    let shader;
    shader = glCreateShader(r#type);


    if shader == 0 {
        std::process::exit(1);
    }
    let shader_ = std::ffi::CString::new(str).unwrap();

    //assert_ne!(shader, 0);
    unsafe { 
        glShaderSource(shader, 1, &(shader_.as_bytes().as_ptr().cast()), &(shader_.count_bytes().try_into().unwrap()));
        glCompileShader(shader);

        // check if shader as succefuly compilated
        let mut success = 0;
        glGetShaderiv(shader, GL_COMPILE_STATUS, &mut success); 
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len: i32 = 0;
            glGetShaderInfoLog(shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Error compile shader: {}", String::from_utf8_lossy(&v));
        }
    }
    shader
}
