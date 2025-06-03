use gl33::*;
use gl33::global_loader::*;

use crate::sdl::*;

use crate::opengl;

fn load_texture(triangle: &mut opengl::GlTriangle) {
    if let Some(texture_src) = &triangle.texture_src {
        unsafe {

            let surface = IMG_Load(std::ffi::CString::new(texture_src.clone()).unwrap().as_ptr());
            if surface.is_null() {
                println!("Error loading texture{texture_src}");
                std::process::exit(1);
            }
            let mut texture = 0;
            glGenTextures(1, &mut texture);
            triangle.texture = texture;
            glBindTexture(GL_TEXTURE_2D, texture);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT.0 as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT.0 as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR.0 as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR.0 as i32);

            glTexImage2D(GL_TEXTURE_2D,
                         0,
                         GL_RGBA.0 as i32,
                         (*surface).w,
                         (*surface).h,
                         0,
                         GL_RGBA,
                         GL_UNSIGNED_BYTE,
                         (*surface).pixels
            );
            glGenerateMipmap(GL_TEXTURE_2D);
            SDL_DestroySurface(surface);
            glVertexAttribPointer(2,
                                  2,
                                  GL_FLOAT,
                                  0 /* GL_False */,
                                  triangle.strides_texture, //3 * std::mem::size_of::<f32>() as i32,
                                  triangle.offset_texture as *const std::ffi::c_void,
                                  );
            glEnableVertexAttribArray(2);
        }
    }
}

//pub fn draw_triangle<const N: usize>(vertices: [opengl::VERTEX; N], opt_indices:  Option<&[u32]>) {
pub fn draw_triangle(gl_triangle: &mut opengl::GlTriangle) {

    unsafe {
        // VBO Vertex buffer object
        let mut vbo = 0;
        // generate a buffer ID
        glGenBuffers(1, &mut vbo); 
        assert!(vbo != 0);
        // VERTEX ARRAY OBJECT
        let mut vao = 0;
        glGenVertexArrays(1, &mut vao);
        glBindVertexArray(vao);
        assert!(vao != 0);

        gl_triangle.vao = vao;
        gl_triangle.vbo = vbo;

        // Opengl has many types of buffer object, and the buffer type of vertex buffer is
        // GL_ARRAY_BUFFER
        // We bind the new id to the type of buffer
        glBindBuffer(GL_ARRAY_BUFFER, vbo);

        // copy the previously vertex data into buffer's memory

        glBufferData(
            GL_ARRAY_BUFFER,
            (gl_triangle.vertices.len() * std::mem::size_of::<f32>()) as isize, 
            gl_triangle.vertices.as_ptr().cast(),
            GL_STATIC_DRAW
            );

        match &gl_triangle.opt_indices {
            Some(indices) => {
                let mut ebo = 0;
                glGenBuffers(1, &mut ebo);
                glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);

                glBufferData(
                    GL_ELEMENT_ARRAY_BUFFER,
                    (indices.len() * std::mem::size_of::<u32>()) as isize, 
                    indices.as_ptr().cast(),
                    GL_STATIC_DRAW
                    );
            },
            None => {}
        } 

        // Then set our vertex attributes pointers
        glVertexAttribPointer(0,
                              3,
                              GL_FLOAT,
                              0 /* GL_False */,
                              gl_triangle.strides, //3 * std::mem::size_of::<f32>() as i32,
                              0 as *const _);
        glEnableVertexAttribArray(0);

        if gl_triangle.strides_color != 0 {
            glVertexAttribPointer(1,
                                  3,
                                  GL_FLOAT,
                                  0 /* GL_False */,
                                  gl_triangle.strides_color, //3 * std::mem::size_of::<f32>() as i32,
                                  gl_triangle.offset_color as *const std::ffi::c_void,
                                  );
            glEnableVertexAttribArray(1);
        }



        let vertex_shader_src = opengl::load_shader(gl_triangle.vertex_shader_src.as_str());
        let vertex_shader = opengl::compile_shader(vertex_shader_src, GL_VERTEX_SHADER);

        let fragement_shader_src = opengl::load_shader(gl_triangle.fragment_shader_src.as_str());
        let fragment_shader = opengl::compile_shader(fragement_shader_src, GL_FRAGMENT_SHADER);

        // shader program
        let shader_program;
        shader_program = glCreateProgram();
        assert_ne!(shader_program, 0);
        glAttachShader(shader_program, vertex_shader);
        glAttachShader(shader_program, fragment_shader);
        glLinkProgram(shader_program);

        let mut success = 0;
        glGetProgramiv(shader_program, GL_LINK_STATUS, &mut success); 
        if success == 0 {
            // XXX
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len: i32 = 0;
            glGetShaderInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());

            panic!("Error link Program: {}", String::from_utf8_lossy(&v));
        }


        load_texture(gl_triangle);

        glDeleteShader(vertex_shader);
        glDeleteShader(fragment_shader);
      //  glUseProgram(shader_program);
        gl_triangle.shader_program = shader_program;
    }
}
