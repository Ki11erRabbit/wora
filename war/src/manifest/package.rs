use std::collections::HashMap;

use crate::manifest::{Dependency, Function};

pub struct BuildSettings {
    compiler: String,
    flags: Vec<String>,
}

impl BuildSettings {
    pub fn new(compiler: String, flags: Vec<String>) -> BuildSettings {
        BuildSettings { compiler, flags }
    }

    pub fn parse(table: &toml::Table) -> Option<BuildSettings> {
        let compiler = table.get("compiler")?.as_str()?.to_string();
        let flags = table.get("flags")?.as_array()?.iter().map(|v| v.as_str().unwrap().to_string()).collect();
        Some(BuildSettings::new(compiler, flags))
    }
}

impl std::fmt::Display for BuildSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "compiler = \"{}\", ", self.compiler)?;
        write!(f, "flags = [")?;
        for flag in &self.flags {
            write!(f, "\"{}\", ", flag)?;
        }
        write!(f, "]")
    }
}


pub struct Package {
    name: String,
    version: String,
    library: Option<Library>,
    build: BuildSettings,
    dependencies: HashMap<String, Dependency>,
}

impl Package {
    pub fn new_binary(
        name: String,
        version: String,
        build: BuildSettings,
        dependencies: HashMap<String, Dependency>,
    ) -> Package {
        Package {
            name,
            version,
            library: None,
            build,
            dependencies,
        }
    }

    pub fn new_library(
        name: String,
        version: String,
        provides: Vec<Function>,
        build: BuildSettings,
        dependencies: HashMap<String, Dependency>,
    ) -> Package {
        Package {
            name,
            version,
            library: Some(Library { provides }),
            build,
            dependencies,
        }
    }

    pub fn parse(table: &toml::Table) -> Option<Package> {
        let package = table.get("package")?.as_table()?;
        let name = package.get("name")?.as_str()?.to_string();
        let version = package.get("version")?.as_str()?.to_string();
        let library = package.get("library").map(|v| {
            let provides = v.get("provides")?.as_array()?.iter().map(|v| Function::parse(v.as_table().unwrap()).unwrap()).collect();
            Some(Library { provides })
        });
        let library = library.unwrap_or(None);
        let build = BuildSettings::parse(table.get("build")?.as_table().unwrap()).unwrap();
        let dependencies = table.get("dependencies")?.as_table()?.iter().map(|(k, v)| {
            let dep = Dependency::parse(v.as_table().unwrap()).unwrap();
            (k.to_string(), dep)
        }).collect();
        Some(Package { name, version, library, build, dependencies })
    }
}


impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[package]")?;
        write!(f, "name = \"{}\", ", self.name)?;
        write!(f, "version = \"{}\", ", self.version)?;
        write!(f, "\n")?;
        if let Some(library) = &self.library {
            write!(f, "{}\n", library)?;
        }
        write!(f, "[build]",)?;
        write!(f, "{}", self.build)?;
        write!(f, "\n")?;
        write!(f, "[dependencies]")?;
        for (name, dep) in &self.dependencies {
            write!(f, "{} = {}\n", name, dep)?;
        }
        Ok(())
    }
}


struct Library {
    provides: Vec<Function>,
}


impl std::fmt::Display for Library {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[lib]")?;
        write!(f, "provides = [")?;
        for (i, function) in self.provides.iter().enumerate() {
            write!(f, "   {}", function)?;
            if i < self.provides.len() - 1 {
                write!(f, ",")?;
            }
        }
        write!(f, "]")
    }
}


pub fn parse_package(toml: &str) -> Option<Package> {
    let table = toml.parse::<toml::Table>().ok()?;
    Package::parse(table.get("package")?.as_table()?)
}
