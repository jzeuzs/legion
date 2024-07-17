use std::io::Result;
use std::path::Path;
use std::{env, fs};

use flate2::write::GzEncoder;
use flate2::Compression;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use tar::Builder;

fn main() -> Result<()> {
    built::write_built_file()?;
    compress_languages()?;

    println!("cargo::rerun-if-changed=languages/");
    println!("cargo::rerun-if-changed=build.rs");

    Ok(())
}

fn compress_languages() -> Result<()> {
    let dir = fs::read_dir("languages")?;

    let mut constants = TokenStream::new();
    let mut matches = TokenStream::new();

    for entry in dir {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_string_lossy();
            let tar = create_tar(&path)?;

            let const_ident = Ident::new(&dir_name.to_uppercase(), Span::call_site());
            let const_decl = quote! {
                pub const #const_ident: &'static [u8] = &[
                    #(#tar),*
                ];
            };

            let dir_ident = Literal::string(&dir_name);
            let match_arm = quote! {
                #dir_ident => Some(#const_ident),
            };

            constants.extend(const_decl);
            matches.extend(match_arm);
        }
    }

    let func_decl = quote! {
        pub fn get_language(name: &str) -> Option<&'static [u8]> {
            match name {
                #matches
                _ => None,
            }
        }
    };

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("languages.rs");

    fs::write(dest_path, format!("{}\n{}", constants, func_decl))?;
    Ok(())
}

fn create_tar(src: &Path) -> Result<Vec<u8>> {
    let mut compressed = Vec::new();
    let encoder = GzEncoder::new(&mut compressed, Compression::best());
    let mut tar = Builder::new(encoder);

    tar.append_dir_all(".", src)?;
    tar.into_inner()?.finish()?;

    Ok(compressed)
}
