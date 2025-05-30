pub mod triangle;

pub type VERTEX = [f32; 3];

use gl33::*;
use gl33::global_loader::*;

pub struct GlTriangle {
    pub vao:                    u32,
    pub vbo:                    u32,
    pub vertex_shader_src:      String,
    pub fragment_shader_src:    String,
    pub vertices:               Vec<f32>,
    pub opt_indices:            Option<Vec<u32>>,
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
        vertex_shader_src:      String::from("shader/simple_vertex.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment.frag"),
        vertices:               vertices,
        opt_indices:            None
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
        vertex_shader_src:      String::from("shader/simple_vertex_color.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment_color.frag"),
        vertices:               vertices,
        opt_indices:            None
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
        vertex_shader_src:      String::from("shader/simple_vertex.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment.frag"),
        vertices:               vertices,
        opt_indices:            None
    };
    triangle::draw_triangle(&mut gl_triangle);
    return gl_triangle;
}

pub fn draw_simple_rectangle_with_indices() -> GlTriangle {

    let mut vertices = Vec::new();

    vertices.push(0.5);
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


    let mut indices = Vec::new();

    indices.push(0);
    indices.push(1);
    indices.push(3);
    indices.push(1);
    indices.push(2);
    indices.push(3);


    let mut gl_triangle: GlTriangle = GlTriangle{ 
        vao:                    0,
        vbo:                    0,
        vertex_shader_src:      String::from("shader/simple_vertex.vert"),
        fragment_shader_src:    String::from("shader/simple_fragment.frag"),
        vertices:               vertices,
        opt_indices:            Some(indices)
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
            // XXX
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len: i32 = 0;
            glGetShaderInfoLog(shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Error compile shader: {}", String::from_utf8_lossy(&v));
        }
    }
    shader
}
