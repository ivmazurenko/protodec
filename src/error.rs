#[derive(Debug, Clone)]
pub struct ProtodecError {
	pub message: String,
}

impl From<std::io::Error> for ProtodecError {
	fn from(err: std::io::Error) -> Self {
		println!("error happens: {}", err);
		ProtodecError {
			message: err.to_string(),
		}
	}
}
