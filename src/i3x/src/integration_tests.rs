#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_project() {
        assert!(i3m_project::create("TestGame".to_string()).is_ok());
    }

    #[test]
    fn test_install_module() {
        let mut pm = PackageManager::new();
        let package = Package {
            name: "i3m-core".to_string(),
            version: "1.0.0".to_string(),
            dependencies: HashMap::new(),
        };
        pm.install(package.clone());
        assert_eq!(pm.installed.get("i3m-core").unwrap().version, "1.0.0");
    }

    #[test]
    fn test_update_module() {
        let mut pm = PackageManager::new();
        let package = Package {
            name: "i3m-core".to_string(),
            version: "1.0.0".to_string(),
            dependencies: HashMap::new(),
        };
        pm.install(package.clone());
        pm.update(Package {
            name: "i3m-core".to_string(),
            version: "1.1.0".to_string(),
            dependencies: HashMap::new(),
        });
        assert_eq!(pm.installed.get("i3m-core").unwrap().version, "1.1.0");
    }
}
