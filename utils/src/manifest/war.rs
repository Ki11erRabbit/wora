
use crate::Module;
use crate::Function;

pub struct WarPackage {
    name: String,
    version: String,
    package: PackageType,
}

impl WarPackage {
    pub fn new_binary(name: String, version: String, main_function: String, path: String) -> WarPackage {
        WarPackage {
            name,
            version,
            package: PackageType::Binary(BinaryPackage {
                main_function,
                path,
            }),
        }
    }

    pub fn new_library(name: String, version: String, provides: Vec<Function>) -> WarPackage {
        WarPackage {
            name,
            version,
            package: PackageType::Library(LibraryPackage {
                provides,
            }),
        }
    }

    pub fn parse(table: &toml::Table) -> Option<WarPackage> {
        let package = table.get("package")?.as_table()?;
        let name = package.get("name")?.as_str()?.to_string();
        let version = package.get("version")?.as_str()?.to_string();
        let package = if let Some(bin) = table.get("bin") {
            println!("bin");
            let main_function = bin.get("main")?.as_str()?.to_string();
            let path = bin.get("path")?.as_str()?.to_string();
            PackageType::Binary(BinaryPackage { main_function, path })
        } else if let Some(lib) = table.get("lib") {
            let provides = lib.get("provides")?.as_array()?.iter().map(|v| Function::parse(v.as_table().unwrap()).unwrap()).collect();
            PackageType::Library(LibraryPackage { provides })
        } else {
            return None;
        };
        Some(WarPackage { name, version, package })
    }

    pub fn is_binary(&self) -> bool {
        match &self.package {
            PackageType::Binary(_) => true,
            _ => false,
        }
    }

    pub fn is_library(&self) -> bool {
        match &self.package {
            PackageType::Library(_) => true,
            _ => false,
        }
    }

    pub fn get_main_function(&self) -> Option<&str> {
        match &self.package {
            PackageType::Binary(bin) => Some(&bin.main_function),
            _ => None,
        }
    }
    pub fn get_path(&self) -> Option<&str> {
        match &self.package {
            PackageType::Binary(bin) => Some(&bin.path),
            _ => None,
        }
    }
}

impl std::fmt::Display for WarPackage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[package]")?;
        write!(f, "name = \"{}\"", self.name)?;
        write!(f, "version = \"{}\"", self.version)?;
        write!(f, "\n",)?;
        write!(f, "{}", self.package)
     
    }
}

    

enum PackageType {
    Binary(BinaryPackage),
    Library(LibraryPackage),
}

impl std::fmt::Display for PackageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PackageType::Binary(binary) => {
                write!(f, "[bin]")?;
                write!(f, "{}", binary)
            },
            PackageType::Library(library) => {
                write!(f, "[lib]")?;
                write!(f, "{}", library)
            },
        }
    }
}

struct BinaryPackage {
    main_function: String,
    path: String,
}

impl std::fmt::Display for BinaryPackage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "name = \"{}\"", self.main_function)?;
        write!(f, "path = \"{}\"", self.path)
    }
}

struct LibraryPackage {
    provides: Vec<Function>,
}

impl std::fmt::Display for LibraryPackage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "provides = [")?;
        for (i, function) in self.provides.iter().enumerate() {
            write!(f, "    {}", function)?;
            if i < self.provides.len() - 1 {
                write!(f, ",")?;
            }
        }
        write!(f, "]")
    }
}


pub fn parse_war_manifest(toml: &str) -> Option<WarPackage> {
    let table = toml.parse::<toml::Table>().ok()?;
    WarPackage::parse(&table)
}
