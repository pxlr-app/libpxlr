use crate::parser::Location;
use async_std::{fs, io};
use async_trait::async_trait;
use std::{marker, path::PathBuf};
use uuid::Uuid;

fn uuid_to_pathbuf(uuid: &Uuid) -> PathBuf {
	#[cfg(target_os = "windows")]
	let path = Into::<std::ffi::OsString>::into(uuid.to_string());
	#[cfg(not(target_os = "windows"))]
	let path = <std::ffi::OsStr as std::os::unix::ffi::OsStrExt>::from_bytes(uuid.as_bytes());
	let mut pathbuf = PathBuf::new();
	pathbuf.push(path);
	pathbuf
}

impl From<&Location<Uuid>> for PathBuf {
	fn from(location: &Location<Uuid>) -> Self {
		let mut path = PathBuf::new();

		match location {
			Location::Public(key) => {
				path.push(uuid_to_pathbuf(key));
			}
			Location::Private { owner, key } => {
				path.push(uuid_to_pathbuf(owner));
				path.push(uuid_to_pathbuf(key));
			}
			_ => {}
		};

		path
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
