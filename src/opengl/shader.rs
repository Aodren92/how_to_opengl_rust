use gl33::GL_TRIANGLES;
use gl33::*;
use gl33::global_loader::*;
use crate::sdl;
use crate::sdl::*;

pub enum TriangleType {
    NORMAL,
    UNIFORM,
}

pub struct Shader {
    pub vao:                    u32,
    pub vbo:                    u32,
    pub texture:                u32,
    pub texture_2:              u32,
    pub shader_program:         u32,
    pub vertex_shader_src:      String,
    pub fragment_shader_src:    String,
    pub texture_src:            Option<String>,
    pub texture_src_2:          Option<String>,
    pub vertices:               Vec<f32>,
    pub strides:                i32,
    pub strides_color:          i32,
    pub offset_color:           i32,
    pub strides_texture:        i32,
    pub offset_texture:         i32,
    pub opt_indices:            Option<Vec<u32>>,
    pub r#type:                 TriangleType,
}

impl Shader {

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

    fn load_texture(texture_src: String, internal_format: i32, format: PixelFormat) -> u32 {
        let mut texture = 0;
        unsafe {
            let surface = IMG_Load(std::ffi::CString::new(texture_src.clone()).unwrap().as_ptr());
            if surface.is_null() {
                println!("Error loading texture: {texture_src}");
                std::process::exit(1);
            }
            glGenTextures(1, &mut texture);
            //shader.texture = texture;
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
                         format, //GL_RGB,
                         GL_UNSIGNED_BYTE,
                         (*surface).pixels
                        );
            glGenerateMipmap(GL_TEXTURE_2D);
            SDL_DestroySurface(surface);
        }
        return texture;

    }

    fn load_textures(shader: &mut Shader) {
        if let Some(texture_src) = &shader.texture_src {
            unsafe {
                glEnable(GL_TEXTURE_2D);
                glEnable(GL_BLEND);
                glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
                glVertexAttribPointer(2,
                                      2,
                                      GL_FLOAT,
                                      0 /* GL_False */,
                                      shader.strides_texture, //3 * std::mem::size_of::<f32>() as i32,
                                      shader.offset_texture as *const std::ffi::c_void,
                                      );
                glEnableVertexAttribArray(2);
                // XXX if more than 2 texture in the turotial use a vec
                shader.texture = Self::load_texture(texture_src.to_string(), GL_BGR.0 as i32, GL_BGRA) ;
                if let Some(texture_src_2) = &shader.texture_src_2 {
                    shader.texture_2 = Self::load_texture(texture_src_2.to_string(), GL_BGRA.0 as i32, GL_BGRA) ;
                }
            }
        }
    }

    pub fn load(&mut self) {

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

            self.vao = vao;
            self.vbo = vbo;

            // Opengl has many types of buffer object, and the buffer type of vertex buffer is
            // GL_ARRAY_BUFFER
            // We bind the new id to the type of buffer
            glBindBuffer(GL_ARRAY_BUFFER, vbo);

            // copy the previously vertex data into buffer's memory

            glBufferData(
                GL_ARRAY_BUFFER,
                (self.vertices.len() * std::mem::size_of::<f32>()) as isize, 
                self.vertices.as_ptr().cast(),
                GL_STATIC_DRAW
                );

            match &self.opt_indices {
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
                                  self.strides, //3 * std::mem::size_of::<f32>() as i32,
                                  0 as *const _);
            glEnableVertexAttribArray(0);

            if self.strides_color != 0 {
                glVertexAttribPointer(1,
                                      3,
                                      GL_FLOAT,
                                      0 /* GL_False */,
                                      self.strides_color, //3 * std::mem::size_of::<f32>() as i32,
                                      self.offset_color as *const std::ffi::c_void,
                                      );
                glEnableVertexAttribArray(1);
            }


            let vertex_shader_src = Self::load_shader(self.vertex_shader_src.as_str());
            let vertex_shader = Self::compile_shader(vertex_shader_src, GL_VERTEX_SHADER);

            let fragement_shader_src = Self::load_shader(self.fragment_shader_src.as_str());
            let fragment_shader = Self::compile_shader(fragement_shader_src, GL_FRAGMENT_SHADER);

            // shader program
            let shader_program;
            shader_program = glCreateProgram();
            assert_ne!(shader_program, 0);
            glAttachShader(shader_program, vertex_shader);
            glAttachShader(shader_program, fragment_shader);
            glLinkProgram(shader_program);

            Self::load_textures(self);

            let mut success = 0;
            glGetProgramiv(shader_program, GL_LINK_STATUS, &mut success); 
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len: i32 = 0;
                glGetShaderInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());

                panic!("Error link Program: {}", String::from_utf8_lossy(&v));
            }
            glDeleteShader(vertex_shader);
            glDeleteShader(fragment_shader);
            self.shader_program = shader_program;
        }
    }

    pub fn draw(&self) {
        unsafe {
            glClearColor(0.9, 0.3, 0.5, 0.5);
            glClear(gl33::GL_COLOR_BUFFER_BIT);
        }

       if self.texture != 0 {
           unsafe {
               glActiveTexture(GL_TEXTURE0);
               glBindTexture(GL_TEXTURE_2D, self.texture);
            glUniform1i(
                glGetUniformLocation(
                    self.shader_program, 
                    std::ffi::CString::new("texture1").unwrap().as_ptr() as *const u8,
                    ),
                    0
            );
           }
       }
       if self.texture_2 != 0 {
           unsafe {
               glActiveTexture(GL_TEXTURE1);
               glBindTexture(GL_TEXTURE_2D, self.texture_2);
            glUniform1i(
                glGetUniformLocation(
                    self.shader_program, 
                    std::ffi::CString::new("texture2").unwrap().as_ptr() as *const u8,
                    ),
                    1
            );
           }
       }
        glUseProgram(self.shader_program);
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
