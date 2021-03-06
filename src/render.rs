use gl::types::*;

use std::ops::Drop;
use std::ffi::CString;
use std::ptr;
use std::mem;
use math;
use gl;

static VS_SOURCE: &'static str = "
#version 330 
layout (location = 0) in vec2 pos;
uniform vec2 scale;
uniform vec2 offset;
uniform float depth = 0.5;

void main(){
    gl_Position = vec4(pos * scale + offset,depth,1.0) - vec4(1.0,1.0,0.0,0.0);
}
";

static FS_SOURCE: &'static str = "
#version 330 
uniform vec3 color = vec3(1.0,1.0,1.0);
out vec4 color_out;

void main(){
    color_out = vec4(color,1.0);
}
";
pub struct RenderObject{
    pub color: math::Color,
    pub scale: math::Vector2,
    pub offset: math::Vector2,
}



struct Mesh{
    vbo: GLuint,
    ibo: GLuint,
    size: i32,
}

struct Shader{
    program: GLuint,
    scale_uni: GLint,
    offset_uni: GLint,
    color_uni: GLint,
    depth_uni: GLint,
}

pub struct Engine{
    shader: Shader,
    mesh: Mesh,
    render_list: Vec<RenderObject>,
}

impl Mesh{
    pub fn new() -> Mesh{
        unsafe{
            let mut vbo: GLuint = 0;
            gl::GenBuffers(1,&mut vbo);
            let mut ibo: GLuint = 0;
            gl::GenBuffers(1,&mut ibo);

            let vertex_data: [f32; 8] = [0.0,0.0,
            1.0,0.0,
            1.0,1.0,
            0.0,1.0,];
            let index_data: [u32; 6] = [0,1,2,
            0,2,3];
            gl::BindBuffer(gl::ARRAY_BUFFER,vbo);
            gl::BufferData(gl::ARRAY_BUFFER,8*4,mem::transmute(&vertex_data[0]),gl::STATIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER,ibo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,6*4,mem::transmute(&index_data[0]),gl::STATIC_DRAW);
            Mesh{
                vbo: vbo,
                ibo: ibo,
                size: 6,
            }
        }
    }

    pub fn draw(&self){
        unsafe{
            gl::BindBuffer(gl::ARRAY_BUFFER,self.vbo);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0,2,gl::FLOAT,0,0,ptr::null());
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER,self.ibo);
            gl::DrawElements(gl::TRIANGLES,self.size,gl::UNSIGNED_INT,mem::transmute(0i64));
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER,0);
            gl::BindBuffer(gl::ARRAY_BUFFER,0);
        } 
    }
}

impl Drop for Mesh{
    fn drop(&mut self){
        unsafe{
            gl::DeleteBuffers(1,&self.vbo);
            gl::DeleteBuffers(1,&self.ibo);
        }
    }
}



impl Shader{
    pub fn new()-> Shader{
        unsafe{
            let vs = gl::CreateShader(gl::VERTEX_SHADER);
            let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
            let vs_src = CString::new(VS_SOURCE.as_bytes()).unwrap();
            let fs_src = CString::new(FS_SOURCE.as_bytes()).unwrap();

            gl::ShaderSource(vs, 1, &vs_src.as_ptr(), ptr::null());
            gl::ShaderSource(fs, 1, &fs_src.as_ptr(), ptr::null());
            gl::CompileShader(vs);
            Shader::check_shader_error(vs);
            gl::CompileShader(fs);
            Shader::check_shader_error(fs);

            let program : GLuint = gl::CreateProgram();
            gl::AttachShader(program,fs);
            gl::AttachShader(program,vs);
            gl::LinkProgram(program);

            let mut compiled: GLint = -1;
            gl::GetProgramiv(program,gl::COMPILE_STATUS, &mut compiled);
            if compiled == gl::FALSE as GLint{
                let mut lenght: GLint = 0;
                gl::GetProgramiv(program,gl::INFO_LOG_LENGTH, &mut lenght);
                let c_str = CString::new(vec![' ' as u8; lenght as usize]).unwrap();
                gl::GetProgramInfoLog(program, lenght, &mut 0,c_str.as_ptr() as *mut i8);
                let bytes = c_str.as_bytes();
                let bytess = bytes.to_vec();
                panic!("Program linking failed {};", String::from_utf8(bytess).unwrap());
            }

            gl::DeleteShader(fs);
            gl::DeleteShader(vs);


            Shader{
                program: program,
                color_uni: Shader::get_attrib_location(program,"color"),
                scale_uni: Shader::get_attrib_location(program,"scale"),
                offset_uni: Shader::get_attrib_location(program,"offset"),
                depth_uni: Shader::get_attrib_location(program,"depth"),
            }
        }
    }

    fn get_attrib_location(program: GLuint,name: &str) -> GLint{
        let c_str = CString::new(name.as_bytes()).unwrap();
        unsafe{
            gl::GetUniformLocation(program,c_str.as_ptr())
        }
    }

    fn check_shader_error(shader: GLuint){
        unsafe{
            let mut compiled: GLint = -1;
            gl::GetShaderiv(shader,gl::COMPILE_STATUS, &mut compiled);
            if compiled == gl::FALSE as GLint{
                let mut lenght: GLint = 0;
                gl::GetShaderiv(shader,gl::INFO_LOG_LENGTH, &mut lenght);
                let c_str = CString::new(vec![' ' as u8; lenght as usize]).unwrap();
                gl::GetShaderInfoLog(shader, lenght, &mut 0,c_str.as_ptr() as *mut i8);
                let bytes = c_str.as_bytes();
                let bytess = bytes.to_vec();
                panic!("Shader compilation failed {};", String::from_utf8(bytess).unwrap());
            }
        }
    }
}

impl Drop for Shader{
    fn drop(&mut self){
        unsafe{
            gl::DeleteProgram(self.program);
        }
    }
}

impl Engine{
    pub fn new() -> Engine{
        Engine{
            shader: Shader::new(),            
            mesh: Mesh::new(),
            render_list: vec![RenderObject{
                color: math::Color{r:1.0,g:0.0,b:0.0},
                scale: math::Vector2{x:10.0,y:10.0},
                offset: math::Vector2{x:10.0,y:10.0},
            }],
        }
    }

    pub fn render(&mut self){
        for i in 0..self.render_list.len() {
            let robj = &self.render_list[i];
            unsafe{
                gl::UseProgram(self.shader.program);
                gl::Uniform2f(self.shader.scale_uni,robj.scale.x/400.0,robj.scale.y/300.0);
                gl::Uniform2f(self.shader.offset_uni,robj.offset.x/400.0,robj.offset.y/300.0);
                gl::Uniform3f(self.shader.color_uni,robj.color.r,robj.color.g,robj.color.b);
            }
            self.mesh.draw();
            unsafe{
                gl::UseProgram(0);
            }
        }
        self.render_list.clear();
    }

    pub fn add_render_obj(&mut self,robj: RenderObject){
        self.render_list.push(robj);                
    }
}
