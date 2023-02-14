use glm::{Vec3, Mat4};
use gl::{types::*, VERTEX_SHADER, FRAGMENT_SHADER};
use std::{ffi::{CStr, CString}, fs::read_to_string};

pub struct Shader {
    pub id: u32
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str ) -> Self {
        let mut shader = Self {id: 0};

        let vertex_string = read_to_string(vertex_path).unwrap_or_else(|_| panic!("Failed to read vertex shader file: {}", vertex_path));
        let fragment_string = read_to_string(fragment_path).unwrap_or_else(|_| panic!("Failed to read fragment shader file: {}", fragment_path));
        
        let v_shader_code = CString::new(vertex_string.as_bytes()).unwrap();
        let f_shader_code = CString::new(fragment_string.as_bytes()).unwrap();
        
        let vertex_shader = vert_shader_from_source(&v_shader_code).unwrap();
        let fragment_shader = frag_shader_from_source(&f_shader_code).unwrap();

        unsafe {
            let id = gl::CreateProgram();

            gl::AttachShader(id, vertex_shader);
            gl::AttachShader(id, fragment_shader);

            gl::LinkProgram(id);

            shader.check_compile_errors(id, "PROGRAM");

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            shader.id = id;

        };
        shader
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id)
    }

    #[allow(dead_code)]
    pub unsafe fn set_bool(&self, name: &CStr, value: bool) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value as i32)
    }

    #[allow(dead_code)]
    pub unsafe fn set_int(&self, name: &CStr, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value)
    }

    #[allow(dead_code)]
    pub unsafe fn set_float(&self, name: &CStr, value: f32) {
        gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), value)
    }

    #[allow(dead_code)]
    pub unsafe fn set_vector3(&self, name: &CStr, value: &Vec3) {
        gl::Uniform3fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, value.as_ptr())
    }

    #[allow(dead_code)]
    pub unsafe fn set_vec3(&self, name: &CStr, x: f32, y: f32, z: f32) {
        gl::Uniform3f(gl::GetUniformLocation(self.id, name.as_ptr()), x, y, z)
    }

    pub unsafe fn set_vec4(&self, name: &str, x: f32, y: f32, z: f32, w: f32) {
        let c_name = CString::new(name).unwrap();
        gl::Uniform4f(gl::GetUniformLocation(self.id, c_name.as_ptr()), x, y, z, w)
    }

    pub unsafe fn set_mat4(&self, name: &str, mat: &Mat4) {
        let c_name = CString::new(name).unwrap();
        gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, c_name.as_ptr()), 1, gl::FALSE, mat.as_ptr())
    }

    unsafe fn check_compile_errors(&self, shader: u32, type_: &str) {
        let mut success = gl::FALSE as GLint;
        let mut info_log: Vec<u8> = vec![0; 1024];
        info_log.set_len(1024 - 1); // subtract 1 to skip the trailing null character
        if type_ != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(shader, 1024, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         std::str::from_utf8(&info_log).unwrap());
            }

        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(shader, 1024, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         std::str::from_utf8(&info_log).unwrap());
            }
        }

    }
}

pub fn vert_shader_from_source(source: &CStr) -> Result<GLuint, String> {
    shader_from_source(source, VERTEX_SHADER)
}

pub fn frag_shader_from_source(source: &CStr) -> Result<GLuint, String> {
    shader_from_source(source, FRAGMENT_SHADER)
}


fn shader_from_source(source: &CStr, type_: GLenum) -> Result<GLuint, String> {
    let id = unsafe { gl::CreateShader(type_) };

    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id)
    };

    let mut success: gl::types::GLint = 1;

    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;

        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }
        
        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
        
        buffer.extend([b' '].iter().cycle().take(len as usize));
        
        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );
        }
        return Err(error.to_string_lossy().into_owned());
    }
    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}