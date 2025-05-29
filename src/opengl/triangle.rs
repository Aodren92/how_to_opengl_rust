use gl33::*;
use gl33::global_loader::*;

use crate::opengl;


pub fn draw_triangle<const N: usize>(vertices: [opengl::VERTEX; N], opt_indices:  Option<&[u32]>) {

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

        // Opengl has many types of buffer object, and the buffer type of vertex buffer is
        // GL_ARRAY_BUFFER
        // We bind the new id to the type of buffer
        glBindBuffer(GL_ARRAY_BUFFER, vbo);

        // copy the previously vertex data into buffer's memory
        glBufferData(
            GL_ARRAY_BUFFER,
            core::mem::size_of_val(&vertices) as isize,
            vertices.as_ptr().cast(),
            GL_STATIC_DRAW
            );

        match opt_indices {
            Some(indices) => {
                let mut ebo =0;
                glGenBuffers(1, &mut ebo);
                glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);

                glBufferData(
                    GL_ELEMENT_ARRAY_BUFFER,
                    core::mem::size_of_val(indices) as isize,
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
                              core::mem::size_of::<opengl::VERTEX>().try_into().unwrap(),
                              0 as *const _);
        glEnableVertexAttribArray(0);


        let vertex_shader_src = opengl::load_shader("shader/simple_vertex.vert");
        let vertex_shader = opengl::compile_shader(vertex_shader_src, GL_VERTEX_SHADER);

        let fragement_shader_src = opengl::load_shader("shader/simple_fragment.frag");
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

        glDeleteShader(vertex_shader);
        glDeleteShader(fragment_shader);
        glUseProgram(shader_program);
        glBindVertexArray(vao);
    }
}
