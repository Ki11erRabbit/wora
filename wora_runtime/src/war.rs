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

fn link_main(file_name: &str, linker: &mut Linker, engine: &Engine, store: &mut Store, manifest: &WarPackage, archive: &ZipArchive<std::fs::File>) -> Result<Instance> {

    if let Some(main) = manifest.get_path() {
        let main_idx = archive.index_for_name(main).unwrap();
        let mut main = archive.by_index(main_idx)?;
        let mut code = Vec::new();
        main.read_to_end(&mut code)?;
        let main_module = Module::new(engine, &code)?;

        let path = format!("{}/{}", TEMP_DIR, file_name);
        archive.extract(path.as_str())?;
        let mut paths = std::fs::read_dir(path.as_str())?.filter(|entry| {
            entry.path().as_path() == &std::path::Path::new(format!("{}/deps",path).as_str())
        });


        let Some(deps) = paths.next() else {
            return Err(anyhow!("No deps directory found in archive"))
        }

        let mut other_modules = Vec::new();
        for entry in std::fs::read_dir(deps.path())? {
            let entry = entry?;
            let path = entry.path();
            let code = std::fs::read(path)?;
            let module = Module::new(engine, &code)?;

            let module = link_subdir(path.clone(), module, linker, store)?;
            let name = path.file_name().unwrap().to_str().unwrap();
            other_modules.push((name, module));
        }

        for (name, module) in other_modules {
            linker.instance(store, name, &module)?;
        } 

        let instance = linker.instantiate(store, &main_module)?;
        Ok(instance)
    }
}

fn link_subdir(path: std::fs::Path, module: Module, linker: &mut Linker, store: &mut Store) -> Result<Module> {
}
