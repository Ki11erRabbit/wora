use std::io::Read;

use anyhow::{Result, anyhow};
use wasi_common::WasiCtx;
use wasmtime::*;
use wasi_common::sync::WasiCtxBuilder;
use zip::ZipArchive;
use utils::parse_war_manifest;
use utils::WarPackage;

const TEMP_DIR: &str = "/tmp";


pub fn run_wasm_archive(file: &str) -> Result<()> {
    let engine = Engine::default();

    let mut linker = Linker::new(&engine);
    wasi_common::sync::add_to_linker(&mut linker, |s| s)?;
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, wasi);

    let file = std::fs::File::open(file)?;
    let mut archive = ZipArchive::new(file)?;
    let manifest_idx = archive.index_for_name("manifest.toml").unwrap();
    let mut manifest = String::new();
    archive.by_index(manifest_idx)?.read_to_string(&mut manifest)?;
    println!("{}", manifest);
    let manifest = parse_war_manifest(&manifest).unwrap();

    if manifest.is_library() {
        return Err(anyhow!("Library packages are not supported yet"))
    }

    let instance = link_main(&mut linker, &engine, &mut store, &manifest, &mut archive)?;

    let run = instance.get_typed_func::<(), ()>(&mut store, manifest.get_main_function().unwrap())?;
    run.call(&mut store, ())?;
    Ok(())

}

fn link_main(
    linker: &mut Linker<WasiCtx>,
    engine: &Engine,
    store: &mut Store<WasiCtx>,
    manifest: &WarPackage,
    archive: &mut ZipArchive<std::fs::File>
) -> Result<Instance> {

    if let Some(main) = manifest.get_path() {
        let main_idx = archive.index_for_name(main).unwrap();
        let mut main = archive.by_index(main_idx)?;
        let mut code = Vec::new();
        main.read_to_end(&mut code)?;
        let main_module = Module::new(engine, &code)?;
        drop(main);
        
        let mut other_modules = Vec::new();
        let mut indices = Vec::new();
        let mut names = Vec::new();
        for entry in archive.file_names() {
            let entry = entry;
            if entry.ends_with(".wasm") {
                let idx = archive.index_for_name(&entry).unwrap();
                indices.push(idx);
                let name = entry.split('.').next().unwrap();
                names.push(name.to_string());
            }
        }
        for idx in indices {
            let mut module = archive.by_index(idx)?;
            let mut code = Vec::new();
            module.read_to_end(&mut code)?;
            let module = Module::new(engine, &code)?;
            other_modules.push(module);
        }

        for (name, module) in names.iter().zip(other_modules.iter()) {
            let wasi = WasiCtxBuilder::new().build();
            let mut store = Store::new(&engine, wasi);
            let instance = linker.instantiate(&mut store, module)?;
            linker.instance(store, name, instance)?;
        }

        let instance = linker.instantiate(store, &main_module)?;
        Ok(instance)
    }
    else {
        Err(anyhow!("No main function found"))
    }
}

