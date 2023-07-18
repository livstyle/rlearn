use std::io;
use std::io::ErrorKind;
use std::fmt;

// 自定义错误类型
#[derive(Debug)]
enum CustomError {
    MyError,
    OtherError,
}

// 实现 Display trait 以便可以使用 {} 格式化输出错误信息
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::MyError => write!(f, "This is a custom error: MyError"),
            CustomError::OtherError => write!(f, "This is a custom error: OtherError"),
        }
    }
}

fn custom_function() -> Result<(), io::Error> {
    // 返回一个自定义错误
    Err(io::Error::new(ErrorKind::Other, CustomError::MyError))
}

fn main() {
    match custom_function() {
        Ok(_) => println!("Function completed successfully."),
        Err(e) => println!("Error: {}", e),
    }
}
