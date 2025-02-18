use std::path::{Path, PathBuf};
use std::{env, fs};

use maplibre_build_tools::mbtiles::extract;
use maplibre_build_tools::wgsl::validate_project_wgsl;

const MUNICH_X: u32 = 17425;
const MUNICH_Y: u32 = 11365;
const MUNICH_Z: u8 = 15;

/// Tiles which can be used by StaticTileFetcher
fn clean_static_tiles() -> PathBuf {
    let out_dir = env::var("OUT_DIR").unwrap();

    let out = Path::new(&out_dir).join("extracted-tiles");

    if out.exists() && out.is_dir() {
        fs::remove_dir_all(&out).unwrap()
    }

    out
}

/*use std::fs::File;
use std::io::BufReader;
use serde_json::Value;*/

fn generate_type_def() -> Option<u32> {
    /*    let f = File::open("style-spec-v8.json").unwrap();
    let mut reader = BufReader::new(f);
    let result = serde_json::from_reader::<_, Value>(&mut reader).unwrap();

    let spec_root = result.as_object()?;
    let version = &spec_root["$version"].as_i64()?;
    let root = &spec_root["$root"].as_object()?;

    for x in spec_root {

    }

    println!("cargo:warning={:?}", version);*/

    Some(5)
}

fn embed_tiles_statically() {
    let out = clean_static_tiles();

    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let source = Path::new(&root_dir).join(format!("test-data/munich-{}.mbtiles", MUNICH_Z));

    if source.exists() {
        println!("cargo:rustc-cfg=static_tiles");
        // Pack tiles around Munich HBF (100 tiles in each direction)
        extract(
            source,
            out,
            MUNICH_Z,
            (MUNICH_X - 2)..(MUNICH_X + 2),
            (MUNICH_Y - 2)..(MUNICH_Y + 2),
        )
        .unwrap();
    } else {
        // Do not statically embed tiles
    }
}

fn main() {
    validate_project_wgsl();

    embed_tiles_statically();
}
