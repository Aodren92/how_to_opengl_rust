pub mod triangle;

pub type VERTEX = [f32; 3];

use gl33::*;
use gl33::global_loader::*;

pub fn draw_simple_triangle() -> usize {

    let vertices: [VERTEX; 3] = [
        [-0.5, -0.5, 0.0],
        [0.5, -0.5, 0.0],
        [0.0, 0.5, 0.0],
    ];
    triangle::draw_triangle(vertices, None);
    return vertices.len();
}

pub fn draw_simple_rectangle() -> usize {

   let vertices: [VERTEX; 6] = [
           // first triangle
       [ 0.5,  0.5,  0.0],  // top right
       [ 0.5, -0.5,  0.0],  // bottom right
       [-0.5,  0.5,  0.0],  // top left 
       // second triangle
       [ 0.5, -0.5, 0.0],  // bottom right
       [-0.5, -0.5, 0.0],  // bottom left
       [-0.5,  0.5, 0.0]   // top left
   ];
   triangle::draw_triangle(vertices, None);
   return vertices.len();
}

pub fn draw_simple_rectangle_with_indices() -> usize {

   let vertices: [VERTEX; 4] = [
       [ 0.5,  0.5,  0.0],  // top right
       [ 0.5, -0.5,  0.0],  // bottom right
       [-0.5, -0.5,  0.0],  // bottom left
       [-0.5,  0.5,  0.0]   // top left
   ];

   let indices: [u32; 6] = [
       0, 1, 3,
       1, 2, 3
   ];
   triangle::draw_triangle(vertices, Some(&indices));
   return indices.len();
}


fn load_shader(src: &str) -> String {
    std::fs::read_to_string(src).expect("Failed to read shader file")
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
