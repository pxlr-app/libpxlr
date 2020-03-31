use crate::node::INode;
use crate::sprite::Group;
use crate::sprite::ImageRGBA;
// use crate::sprite::ImageI;
// use crate::sprite::ImageUV;

pub enum Layer {
	Group(Group),
	ImageRGBA(ImageRGBA),
	// ImageI(ImageI),
	// ImageUV(ImageUV),
}

pub trait ILayer: INode {}