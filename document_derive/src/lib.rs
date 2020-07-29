extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(DocumentNode)]
pub fn documentnode_derive(input: TokenStream) -> TokenStream {
	let ast = syn::parse(input).unwrap();
	impl_documentnode_macro(&ast)
}

fn impl_documentnode_macro(ast: &syn::DeriveInput) -> TokenStream {
	let name = &ast.ident;

	let gen = quote! {
		impl document::prelude::Node for #name {
			fn id(&self) -> document::prelude::Uuid {
				self.id
			}
		}

		impl document::prelude::DocumentNode for #name {}
	};

	gen.into()
}

#[proc_macro_derive(Command)]
pub fn command_derive(input: TokenStream) -> TokenStream {
	let ast = syn::parse(input).unwrap();
	impl_command_macro(&ast)
}

fn impl_command_macro(ast: &syn::DeriveInput) -> TokenStream {
	let name = &ast.ident;

	let gen = quote! {
		impl document::prelude::Command for #name {
			fn target(&self) -> document::prelude::Uuid {
				self.target
			}
		}
	};

	gen.into()
}

#[proc_macro_derive(SpriteNode)]
pub fn spritenode_derive(input: TokenStream) -> TokenStream {
	let ast = syn::parse(input).unwrap();
	impl_spritenode_macro(&ast)
}

fn impl_spritenode_macro(ast: &syn::DeriveInput) -> TokenStream {
	let name = &ast.ident;

	let gen = quote! {
		impl document::prelude::Node for #name {
			fn id(&self) -> document::prelude::Uuid {
				self.id
			}
		}

		impl document::prelude::SpriteNode for #name {}
	};

	gen.into()
}
