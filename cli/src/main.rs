use clap::{crate_version, App, Arg, SubCommand};
use document::prelude::*;
use image::DynamicImage;
use std::{ffi, fs, io, path};

fn main() {
	let doc_arg = Arg::with_name("document")
		.long("document")
		.short("d")
		.value_name("DOCUMENT")
		.help("Path to a document");
	let rev_arg = Arg::with_name("revision")
		.long("revision")
		.short("r")
		.value_name("UUID")
		.help("Specific revision UUID");
	let long_id_arg = Arg::with_name("show-long-id")
		.long("show-long-id")
		.help("Show long ID");
	let out_arg = Arg::with_name("out")
		.long("out")
		.short("out")
		.value_name("DOCUMENT")
		.help("Path to a document");
	let x_arg = Arg::with_name("x")
		.short("x")
		.value_name("X")
		.validator(|v| {
			v.parse::<u32>()
				.map_or_else(|_| Err("expected an integer".into()), |_| Ok(()))
		})
		.help("X coordinate");
	let y_arg = Arg::with_name("y")
		.short("y")
		.value_name("Y")
		.validator(|v| {
			v.parse::<u32>()
				.map_or_else(|_| Err("expected an integer".into()), |_| Ok(()))
		})
		.help("Y coordinate");
	let width_arg = Arg::with_name("width")
		.short("w")
		.long("width")
		.value_name("WIDTH")
		.validator(|v| {
			v.parse::<u32>()
				.map_or_else(|_| Err("expected an integer".into()), |_| Ok(()))
		})
		.help("Width");
	let height_arg = Arg::with_name("height")
		.short("h")
		.long("height")
		.value_name("HEIGHT")
		.requires("width")
		.validator(|v| {
			v.parse::<u32>()
				.map_or_else(|_| Err("expected an integer".into()), |_| Ok(()))
		})
		.help("Height");
	let matches = App::new("PXLR CLI")
		.version(crate_version!())
		.about("Create and manipulate PXLR documents")
		.arg(doc_arg.clone().required(true).help("Document to inspect"))
		.subcommand(
			SubCommand::with_name("revisions")
				.about("Show previous revisions")
				.arg(long_id_arg.clone()),
		)
		.subcommand(
			SubCommand::with_name("squash")
				.about("Squash all previous revisions of the document")
				.arg(out_arg.clone().required(true)),
		)
		.subcommand(
			SubCommand::with_name("revert")
				.about("Revert to a previous revision of the document")
				.arg(rev_arg.clone().required(true).help("Revision to revert to")),
		)
		.subcommand(
			SubCommand::with_name("fork")
				.about("Fork the document")
				.arg(out_arg.clone().required(true)),
		)
		.subcommand(
			SubCommand::with_name("list")
				.about("List nodes")
				.arg(rev_arg.clone())
				.arg(
					Arg::with_name("max-depth")
						.long("max-depth")
						.value_name("N")
						.help("Only print node that are N or fewer levels deep"),
				)
				.arg(
					Arg::with_name("show-long-id")
						.long("show-long-id")
						.help("Show long ID"),
				),
		)
		.subcommand(
			SubCommand::with_name("search")
				.about("Search nodes")
				.arg(rev_arg.clone())
				.arg(
					Arg::with_name("all-revisions")
						.long("all-revisions")
						.conflicts_with("revision")
						.help("Search within all revisions"),
				)
				.arg(
					Arg::with_name("max-depth")
						.long("max-depth")
						.value_name("N")
						.help("Only print node that are N or fewer levels deep"),
				)
				.arg(
					Arg::with_name("filter")
						.long("filter")
						.value_name("QUERY")
						.required(true)
						.help("Query string used to filter nodes"),
				)
				.arg(
					Arg::with_name("root")
						.long("root")
						.value_name("UUID")
						.help("Search only within specific node"),
				),
		)
		.subcommand(
			SubCommand::with_name("describe")
				.about("Describe a node")
				.arg(rev_arg.clone())
				.arg(
					Arg::with_name("id")
						.long("id")
						.value_name("UUID")
						.required(true)
						.help("UUID of the node"),
				),
		)
		.subcommand(
			SubCommand::with_name("export")
				.about("Export nodes")
				.arg(rev_arg)
				.arg(
					Arg::with_name("id")
						.long("id")
						.value_name("UUID")
						.multiple(true)
						.required(true)
						.help("UUID of the node(s) to export"),
				),
		)
		.subcommand(
			SubCommand::with_name("add")
				.about("Add node to the document")
				.arg(
					Arg::with_name("parent")
						.long("parent")
						.value_name("UUID")
						.help("UUID of the parent node"),
				)
				.arg(x_arg.clone())
				.arg(y_arg.clone())
				.subcommand(
					SubCommand::with_name("note").about("Add note node").arg(
						Arg::with_name("content")
							.long("content")
							.value_name("CONTENT")
							.required(true)
							.help("Content of the note"),
					),
				)
				.subcommand(
					SubCommand::with_name("group").about("Add group node").arg(
						Arg::with_name("name")
							.long("name")
							.value_name("NAME")
							.required(true)
							.help("Name of the group"),
					),
				)
				.subcommand(
					SubCommand::with_name("canvas")
						.about("Add canvas node")
						.arg(
							Arg::with_name("name")
								.long("name")
								.value_name("NAME")
								.help("Name of the canvas"),
						)
						.arg(width_arg.clone().required(true))
						.arg(height_arg.clone())
						.arg(
							Arg::with_name("format")
								.long("format")
								.value_name("FORMAT")
								.possible_values(&[
									"I", "IXYZ", "RGB", "RGBXYZ", "RGBA", "RGBAXYZ", "UV",
								])
								.required(true)
								.help("Format of the canvas"),
						),
				)
				.subcommand(
					SubCommand::with_name("layer")
						.about("Add layer node")
						.arg(
							Arg::with_name("name")
								.long("name")
								.value_name("NAME")
								.help("Name of the layer"),
						)
						.arg(
							Arg::with_name("file")
								.value_name("FILE")
								.required(true)
								.help("Content of the layer")
								.validator_os(validate_file),
						),
				),
		)
		.get_matches();

	let doc_path = matches
		.value_of_os("document")
		.expect("--document argument missing");

	let new_document = !path::Path::new(doc_path).is_file();
	let mut handle = fs::OpenOptions::new()
		.read(true)
		.write(true)
		.create(true)
		.open(doc_path)
		.expect("Could not access document.");

	if let Some(matches) = matches.subcommand_matches("revisions") {
		let long_id = matches.is_present("show-long-id");
		let mut file = File::read(&mut handle).expect("Could not parse document.");
		loop {
			println!(
				"{} {}",
				if long_id {
					file.index.hash.to_string()
				} else {
					file.index.hash.to_string()[..8].to_owned()
				},
				file.rows.len()
			);

			if let Ok(prev) = file.read_previous(&mut handle) {
				file = prev;
			} else {
				break;
			}
		}
	} else if let Some(matches) = matches.subcommand_matches("squash") {
		let mut file = File::read(&mut handle).expect("Could not parse document.");
		let out_path = matches.value_of_os("out").unwrap();
		let mut out_handle = fs::OpenOptions::new()
			.write(true)
			.create(true)
			.truncate(true)
			.open(out_path)
			.expect("Could not access document.");
		file.trim(&mut handle, &mut out_handle)
			.expect("Could not trim document.");
	} else if let Some(matches) = matches.subcommand_matches("revert") {
		let mut file = load_file_at(&mut handle, matches.value_of("revision"))
			.expect("Could not parse document.");
		file.update_index_only(&mut handle)
			.expect("Could not revert document.");
	} else if let Some(matches) = matches.subcommand_matches("fork") {
		let out_path = matches.value_of_os("out").unwrap();
		fs::copy(doc_path, out_path).expect("Could not form document");
	} else if let Some(matches) = matches.subcommand_matches("list") {
		let file = load_file_at(&mut handle, matches.value_of("revision"))
			.expect("Could not parse document.");
		let long_id = matches.is_present("show-long-id");
		for row in &file.rows {
			println!(
				"{} {} {} {} {}",
				if long_id {
					row.id.to_string()
				} else {
					row.id.to_string()[..8].to_owned()
				},
				row.chunk_type,
				row.chunk_offset,
				row.chunk_size,
				row.name
			);
		}
	} else if let Some(matches) = matches.subcommand_matches("add") {
		let mut file = if new_document {
			File::new()
		} else {
			File::read(&mut handle).expect("Could not parse document.")
		};
		let parent = if let Some(uuid) = matches.value_of("parent") {
			let uuid = find_id_lazy(&file, uuid).expect("Could not find UUID in document.");
			file.get(&mut handle, uuid)
				.expect("Could not find parent in document.")
		} else {
			Arc::new(NodeType::Group(GroupNode {
				id: Uuid::new_v4(),
				name: Arc::new("Root".into()),
				..Default::default()
			}))
		};
		let x = matches
			.value_of("x")
			.map_or_else(|| 0, |v| v.parse::<u32>().unwrap());
		let y = matches
			.value_of("y")
			.map_or_else(|| 0, |v| v.parse::<u32>().unwrap());
		let position = Vec2::new(x, y);

		let id = Uuid::new_v4();
		let node = if let Some(matches) = matches.subcommand_matches("note") {
			let content = matches.value_of("content").unwrap();
			NodeType::Note(NoteNode {
				id,
				name: Arc::new(content.to_owned()),
				position: Arc::new(position),
				..Default::default()
			})
		} else if let Some(matches) = matches.subcommand_matches("group") {
			let name = matches.value_of("name").unwrap();
			NodeType::Group(GroupNode {
				id,
				name: Arc::new(name.to_owned()),
				position: Arc::new(position),
				..Default::default()
			})
		} else if let Some(matches) = matches.subcommand_matches("canvas") {
			let name = matches.value_of("name").unwrap();
			let channels = map_format_to_channel(&matches.value_of("format").unwrap()).unwrap();
			let width = matches.value_of("width").unwrap().parse::<u32>().unwrap();
			let height = matches
				.value_of("height")
				.map_or_else(|| width, |v| v.parse::<u32>().unwrap());
			let size = Extent2::new(width, height);
			NodeType::CanvasGroup(CanvasGroupNode {
				id,
				name: Arc::new(name.to_owned()),
				position: Arc::new(position),
				size: Arc::new(size),
				opacity: 1.,
				channels: channels,
				..Default::default()
			})
		} else if let Some(matches) = matches.subcommand_matches("layer") {
			let name = matches.value_of("name").unwrap_or("Layer");
			let path = matches.value_of_os("file").unwrap();
			let canvas = load_image_at(path).expect("Could not load image.");
			NodeType::Canvas(CanvasNode {
				id,
				name: Arc::new(name.to_owned()),
				size: Arc::new(canvas.size),
				channels: canvas.channels,
				canvas: Arc::new(canvas),
				..Default::default()
			})
		} else {
			unimplemented!()
		};

		println!("{}", node.as_node().id());

		let parent = if let NodeType::Group(_) = node {
			node
		} else {
			match *parent {
				NodeType::Group(ref parent) => {
					if node.as_documentnode().is_none() {
						panic!("can not add a non-DocumentNode to a GroupNode");
					}
					let noderef = Arc::new(node);
					let (redo, _) = parent
						.add_child(noderef.clone())
						.expect("Could not add child");
					parent.execute(&redo).expect("Could not add child")
				}
				NodeType::CanvasGroup(ref parent) => {
					match node.as_spritenode() {
						Some(sprite) if sprite.channels() != parent.channels() => {
							panic!("layer needs to match the canvas format")
						}
						None => panic!("can not add a non-DocumentNode to a GroupNode"),
						_ => {}
					}
					let noderef = Arc::new(node);
					let (redo, _) = parent
						.add_child(noderef.clone())
						.expect("Could not add child");
					parent.execute(&redo).expect("Could not add child")
				}
				_ => panic!("Could not add a node to parent"),
			}
		};

		if new_document {
			file.write(&mut handle, &parent)
				.expect("Could not create document");
		} else {
			file.update(&mut handle, &parent)
				.expect("Could not update document");
		}
	} else {
		unimplemented!()
	}
}

fn map_format_to_channel(format: &str) -> Result<Channel, &'static str> {
	match format {
		"I" => Ok(Channel::I),
		"IXYZ" => Ok(Channel::I | Channel::XYZ),
		"RGB" => Ok(Channel::RGB),
		"RGBXYZ" => Ok(Channel::RGB | Channel::XYZ),
		"RGBA" => Ok(Channel::RGB | Channel::A),
		"RGBAXYZ" => Ok(Channel::RGB | Channel::A | Channel::XYZ),
		"UV" => Ok(Channel::UV),
		_ => Err("unknown format"),
	}
}

fn load_file_at<R: io::Read + io::Seek>(
	handle: &mut R,
	begins_with: Option<&str>,
) -> Result<File, &'static str> {
	let mut file = File::read(handle).expect("Could not parse document.");
	if let Some(begins_with) = begins_with {
		let len = begins_with.len();
		loop {
			if &file.index.hash.to_string()[..len] == begins_with {
				return Ok(file);
			}
			match file.read_previous(handle) {
				Ok(prev) => file = prev,
				Err(_) => break,
			};
		}
		Err("Could not find revision.")
	} else {
		Ok(file)
	}
}

fn find_id_lazy(file: &File, begins_with: &str) -> Result<Uuid, &'static str> {
	let len = begins_with.len();
	for row in &file.rows {
		if &row.id.to_string()[..len] == begins_with {
			return Ok(row.id);
		}
	}
	Err("Could not find node.")
}

fn validate_file(path: &ffi::OsStr) -> Result<(), ffi::OsString> {
	if path::Path::new(path).is_file() {
		Ok(())
	} else {
		Err(ffi::OsString::from(format!(
			"file {} does not exists",
			path.to_string_lossy()
		)))
	}
}

// fn validate_uuid(val: String) -> Result<(), String> {
// 	Uuid::parse_str(&val).map_or_else(|_| Err("not a valid UUID".to_owned()), |_| Ok(()))
// }

#[cfg(feature = "imagerust")]
fn load_image_at(path: &ffi::OsStr) -> Result<Canvas, String> {
	match image::open(path).map_err(|_| "could not load image")? {
		DynamicImage::ImageRgb8(img) => {
			let (w, h) = img.dimensions();
			let channels = Channel::RGB;
			let len = channels.size();
			let mut pixels = vec![0u8; w as usize * h as usize * len];

			for (x, y, pixel) in img.enumerate_pixels() {
				let index = (x as u32 + w * y as u32) as usize;
				let buf = &mut pixels[(index * len)..((index + 1) * len)];
				unsafe {
					*channels.unsafe_rgb_mut(buf) = RGB::new(pixel[0], pixel[1], pixel[2]);
				}
			}

			Ok(Canvas::from_buffer(Extent2::new(w, h), channels, pixels))
		}
		DynamicImage::ImageRgba8(img) => {
			let (w, h) = img.dimensions();
			let channels = Channel::RGB | Channel::A;
			let len = channels.size();
			let mut pixels = vec![0u8; w as usize * h as usize * len];

			for (x, y, pixel) in img.enumerate_pixels() {
				let index = (x as u32 + w * y as u32) as usize;
				let buf = &mut pixels[(index * len)..((index + 1) * len)];
				unsafe {
					*channels.unsafe_rgb_mut(buf) = RGB::new(pixel[0], pixel[1], pixel[2]);
					*channels.unsafe_a_mut(buf) = A::new(pixel[3]);
				}
			}

			Ok(Canvas::from_buffer(Extent2::new(w, h), channels, pixels))
		}
		_ => Err("unknown color format".into()),
	}
}
