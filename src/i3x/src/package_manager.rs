use std::collections::HashMap;

#[derive(Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub dependencies: HashMap<String, String>, // Module name -> Version
}

pub struct PackageManager {
    installed: HashMap<String, Package>, // Module name -> Package
}

impl PackageManager {
    pub fn new() -> Self {
        PackageManager {
            installed: HashMap::new(),
        }
    }

    pub fn install(&mut self, package: Package) {
        println!("Installing package {} version {}", package.name, package.version);
        self.installed.insert(package.name.clone(), package);
    }

    pub fn resolve_dependencies(&self, package: &Package) {
        for (dep_name, dep_version) in &package.dependencies {
            println!("Resolving dependency {} version {}", dep_name, dep_version);
            // In real-world usage, query the decentralized registry to resolve dependencies
        }
    }

    pub fn update(&mut self, package: Package) {
        println!("Updating package {} to version {}", package.name, package.version);
        self.installed.insert(package.name.clone(), package);
    }
}
