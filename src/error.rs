#[derive(Debug, Clone)]
pub struct EmptyError {}

impl From<std::io::Error> for EmptyError {
    fn from(err: std::io::Error) -> Self {
        println!("error happens: {}", err);
        EmptyError {}
    }
}
