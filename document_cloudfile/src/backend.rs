use crate::parser::Location;
use async_std::{fs, io};
use async_trait::async_trait;
use std::{marker, path::PathBuf};
use uuid::Uuid;

impl From<&Location<Uuid>> for PathBuf {
	fn from(location: &Location<Uuid>) -> Self {
		let mut path = location.0.to_string();
		path.push('_');
		path.push_str(&location.1.to_string());
		path.into()
	}
}

#[async_trait(?Send)]
pub trait Backend {
	type Reader: io::Read + io::Seek + marker::Unpin;
	type Writer: io::Write + marker::Unpin;

	async fn get_reader(&mut self, location: &Location<Uuid>) -> io::Result<Self::Reader>;
	async fn get_writer(&mut self, location: &Location<Uuid>) -> io::Result<Self::Writer>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalVaultBackend {
	root: PathBuf,
}

impl LocalVaultBackend {
	pub fn new(root: impl Into<PathBuf>) -> Self {
		Self { root: root.into() }
	}
}

#[async_trait(?Send)]
impl Backend for LocalVaultBackend {
	type Reader = fs::File;
	type Writer = fs::File;

	async fn get_reader(&mut self, location: &Location<Uuid>) -> io::Result<Self::Reader> {
		let path: PathBuf = location.into();
		let file = fs::OpenOptions::new()
			.read(true)
			.open(self.root.join(path))
			.await?;
		Ok(file)
	}

	async fn get_writer(&mut self, location: &Location<Uuid>) -> io::Result<Self::Writer> {
		let path: PathBuf = location.into();
		let file = fs::OpenOptions::new()
			.create(true)
			.write(true)
			.open(self.root.join(path))
			.await?;
		Ok(file)
	}
}
