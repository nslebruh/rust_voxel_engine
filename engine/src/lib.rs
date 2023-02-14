pub mod texture;
pub mod window;
pub mod camera;
pub mod renderer;
pub mod keybinds;
pub mod input_functions;
pub mod buffer;
pub mod shader;

pub extern crate nalgebra_glm as glm;
pub extern crate glfw;
pub extern crate gl;
pub extern crate nalgebra as na;
pub extern crate image;


pub struct Config {

}

impl Config {

}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn it_works() {
//        let result = add(2, 2);
//        assert_eq!(result, 4);
//    }
//}

#[macro_use]
pub mod macros {
    #[macro_export]
    macro_rules! glchk {
        ($($s:stmt;)*) => {
            $(
                $s;
                if cfg!(debug_assertions) {
                    let err = gl::GetError();
                    if err != gl::NO_ERROR {
                        let err_str = match err {
                            gl::INVALID_ENUM => "GL_INVALID_ENUM",
                            gl::INVALID_VALUE => "GL_INVALID_VALUE",
                            gl::INVALID_OPERATION => "GL_INVALID_OPERATION",
                            gl::INVALID_FRAMEBUFFER_OPERATION => "GL_INVALID_FRAMEBUFFER_OPERATION",
                            gl::OUT_OF_MEMORY => "GL_OUT_OF_MEMORY",
                            gl::STACK_UNDERFLOW => "GL_STACK_UNDERFLOW",
                            gl::STACK_OVERFLOW => "GL_STACK_OVERFLOW",
                            _ => "unknown error"
                        };
                        println!("{}:{} - {} caused {}",
                                 file!(),
                                 line!(),
                                 stringify!($s),
                                 err_str);
                    } else {
                        println!("No errors")
                    }
                }
            )*
        }
    }
}
