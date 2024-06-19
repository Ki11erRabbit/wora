use std::io::Read;

use wasmtime::*;
use wasi_common::sync::WasiCtxBuilder;
use zip::ZipArchive;
use utils::parse_war_manifest;

const TEMP_DIR: &str = "/tmp";


pub fn run_wasm_archive(file: &str) -> Result<()> {
    let engine = Engine::default();

    let mut linker = Linker::new(&engine);
    wasi_common::sync::add_to_linker(&mut linker, |s| s)?;

    let file = std::fs::File::open(file)?;
    let mut archive = ZipArchive::new(file)?;
    let manifest_idx = archive.index_for_name("manifest.toml").unwrap();
    let mut manifest = String::new();
    archive.by_index(manifest_idx)?.read_to_string(&mut manifest)?;
    let manifest = parse_war_manifest(&manifest).unwrap();

    if manifest.is_library() {
        Err(anyhow!("Library packages are not supported yet"))
    }

    

}
