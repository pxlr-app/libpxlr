pub struct Document {
	pub root: Group,
}

pub enum Node {
	Group(Group),
	Label(Label)
}

pub struct Group {
	pub name: String,
	pub children: Vec<Node>,
}

pub struct Label {
	pub name: String,
}

pub struct Canvas {
	pub name: String,
	pub layers: Vec<CanvasLayer>,
	pub width: i32,
	pub height: i32,
}

pub enum CanvasLayer {
	Group(CanvasLayerGroup),
	DataRGBA(CanvasLayerDataRGBA),
	DataI(CanvasLayerDataI),
	DataUV(CanvasLayerDataUV)
}

pub struct CanvasLayerGroup {
	pub name: String,
	pub children: Vec<CanvasLayer>,
}

pub struct CanvasLayerDataRGBA {
	pub name: String,
	pub width: i32,
	pub height: i32,
	pub data: Vec<i32>,
}

pub struct CanvasLayerDataI {
	pub name: String,
	pub width: i32,
	pub height: i32,
	pub data: Vec<i8>,
}

pub struct CanvasLayerDataUV {
	pub name: String,
	pub width: i32,
	pub height: i32,
	pub data: Vec<i32>,
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
