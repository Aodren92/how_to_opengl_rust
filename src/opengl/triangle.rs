use gl33::*;
use gl33::global_loader::*;

const VERTEX_SHADER_SOURCE : &str = r#"#version 330 core
    layout(location = 0) in vec3 aPos;

    void main() 
    {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const VERTEX_SHADER_FRAGMENT: &str = r#"#version 330 core
    out vec4 FragColor;

    void main()
    {
        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

type VERTEX = [f32; 3];
const VERTICES: [VERTEX; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];



pub fn draw_triangle() {

    unsafe {
        // VBO Vertex buffer object
        let mut vbo = 0;
        // generate a buffer ID
        glGenBuffers(1, &mut vbo); 
        //
        // VERTEX ARRAY OBJECT
        let mut vao = 0;
        glGenVertexArrays(1, &mut vao);
        glBindVertexArray(vao);

        // Opengl has many types of buffer object, and the buffer type of vertex buffer is
        // GL_ARRAY_BUFFER
        // We bind the new id to the type of buffer
        glBindBuffer(GL_ARRAY_BUFFER, vbo);

        // copy the previously vertex data into buffer's memory
        glBufferData(
            GL_ARRAY_BUFFER,
            core::mem::size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(), GL_STATIC_DRAW
            );

        // Then set our vertex attributes pointers
        glVertexAttribPointer(0, 3, GL_FLOAT, 0 /* GL_False */, core::mem::size_of::<VERTEX>().try_into().unwrap(), 0 as *const _);
        glEnableVertexAttribArray(0);



        // Compile shader at runtime
        let vertex_shader;
        vertex_shader = glCreateShader(GL_VERTEX_SHADER);
        if vertex_shader == 0 {
            std::process::exit(1);

        }
        assert_ne!(vertex_shader, 0);
        glShaderSource(vertex_shader, 1, &(VERTEX_SHADER_SOURCE.as_bytes().as_ptr().cast()), &(VERTEX_SHADER_SOURCE.len().try_into().unwrap()));
        glCompileShader(vertex_shader);

        // check if shader as succefuly compilated
        let mut success = 0;
        glGetShaderiv(vertex_shader, GL_COMPILE_STATUS, &mut success); 
        if success == 0 {
            // XXX
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len: i32 = 0;
            glGetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());

            panic!("Error compile shader: {}", String::from_utf8_lossy(&v));
        }

        // FRAGMENT SHADER
        let fragment_shader;
        fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0);
        glShaderSource(fragment_shader, 1, &(VERTEX_SHADER_FRAGMENT.as_bytes().as_ptr().cast()), &(VERTEX_SHADER_FRAGMENT.len().try_into().unwrap()));
        glCompileShader(fragment_shader);
        // check if shader as succefuly compilated
        let mut success = 0;
        glGetShaderiv(fragment_shader, GL_COMPILE_STATUS, &mut success); 
        if success == 0 {
            // XXX
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len: i32 = 0;
            glGetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());

            panic!("Error compile shader: {}", String::from_utf8_lossy(&v));
        }

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
