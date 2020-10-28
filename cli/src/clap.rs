use clap::{crate_version, App, Arg, SubCommand};
use std::{ffi, path};

pub fn get_clap_app() -> App<'static, 'static> {
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
	App::new("PXLR CLI")
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
				)
				.arg(
					Arg::with_name("format")
						.long("format")
						.possible_values(&["json", "ron"])
						.default_value("ron")
						.help("Format"),
				)
				.arg(Arg::with_name("pretty").long("pretty").help("Pretty print")),
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
								.validator_os(file_exists),
						),
				),
		)
}

fn file_exists(path: &ffi::OsStr) -> Result<(), ffi::OsString> {
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
