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

#[proc_macro_derive(Patch)]
pub fn patch_derive(input: TokenStream) -> TokenStream {
	let ast = syn::parse(input).unwrap();
	impl_patch_macro(&ast)
}

fn impl_patch_macro(ast: &syn::DeriveInput) -> TokenStream {
	let name = &ast.ident;

	let gen = quote! {
		impl document::prelude::patch::Patch for #name {
			fn target(&self) -> document::prelude::Uuid {
				self.target
			}
		}
	};

	gen.into()
}

#[proc_macro_derive(Color)]
pub fn color_derive(input: TokenStream) -> TokenStream {
	let ast = syn::parse(input).unwrap();
	impl_color_macro(&ast)
}

fn impl_color_macro(ast: &syn::DeriveInput) -> TokenStream {
	let name = &ast.ident;

	let gen = quote! {
		impl document::prelude::Color for #name {}
	};

	gen.into()
}
