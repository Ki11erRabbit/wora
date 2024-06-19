

mod package;
mod war;

pub use package::*;
pub use war::*;

pub enum Dependency {
    Git(GitDependency),
}

impl Dependency {
    pub fn parse(table: &toml::Table) -> Option<Dependency> {
        if let Some(git) = table.get("git") {
            Some(Dependency::Git(GitDependency::parse(git.as_table().unwrap()).unwrap()))
        } else {
            None
        }
    }
}

impl std::fmt::Display for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Dependency::Git(git) => write!(f, "{}", git),
        }
    }
}


pub struct GitDependency {
    url: String,
    branch: Option<String>,
    tag: Option<String>,
    commit: Option<String>,
}

impl GitDependency {
    pub fn parse(table: &toml::Table) -> Option<GitDependency> {
        let url = table.get("url")?.as_str()?.to_string();
        let branch = table.get("branch").map(|b| b.as_str().unwrap().to_string());
        let tag = table.get("tag").map(|t| t.as_str().unwrap().to_string());
        let commit = table.get("commit").map(|c| c.as_str().unwrap().to_string());
        Some(GitDependency { url, branch, tag, commit })
    }
}

impl std::fmt::Display for GitDependency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{ ")?;
        write!(f, "url = \"{}\", ", self.url)?;
        if let Some(branch) = &self.branch {
            write!(f, "branch = \"{}\", ", branch)?;
        }
        if let Some(tag) = &self.tag {
            write!(f, "tag = \"{}\", ", tag)?;
        }
        if let Some(commit) = &self.commit {
            write!(f, "commit = \"{}\", ", commit)?;
        }
        write!(f, "}}")
    }
}

pub struct Module {
    path: String,
    functions: Vec<Function>,
}

impl Module {
    pub fn parse(table: &toml::Table) -> Option<Module> {
        let path = table.get("path")?.as_str()?.to_string();
        let functions = table.get("functions")?.as_array()?.iter().map(|f| Function::parse(f.as_table().unwrap()).unwrap()).collect();
        Some(Module { path, functions })
    }
}

impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{ ")?;
        write!(f, "path = \"{}\", ", self.path)?;
        write!(f, "functions = [\n")?;
        for (i, function) in self.functions.iter().enumerate() {
            write!(f, "        {}, ", function)?;
            if i < self.functions.len() - 1 {
                write!(f, ",\n")?;
            }
        }
        write!(f, "    ]")?;
        write!(f, "}}")
    }
}

pub struct Function {
    name: String,
    args: Vec<String>,
    returns: String,
}

impl Function {
    pub fn parse(table: &toml::Table) -> Option<Function> {
        let name = table.get("name")?.as_str()?.to_string();
        let args = table.get("args")?.as_array()?.iter().map(|a| a.as_str().unwrap().to_string()).collect();
        let returns = table.get("returns")?.as_str()?.to_string();
        Some(Function { name, args, returns })
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{ ")?;
        write!(f, "name = \"{}\", ", self.name)?;
        write!(f, "args = [, ")?;
        for arg in &self.args {
            write!(f, "\"{}\", ", arg)?;
        }
        write!(f, "], ")?;
        write!(f, "returns = \"{}\" ", self.returns)?;
        write!(f, "}}")
    }
}
  
