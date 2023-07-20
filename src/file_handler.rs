use std::path::Path;
use regex::Regex;

pub struct MvObj {
    pub source: String,
    pub target: String,
    pub pattern: String,
    pub exclude: bool,
}

pub trait Mover {
    fn create(source: String, target: String, pattern: String, exclude: bool) -> MvObj;
    fn create_ref_str(source: &str, target: &str, pattern: &str, exclude: bool) -> MvObj;
    fn print_src_files(&self);
    fn get_file_paths(&self) -> Vec<String>;
    fn is_part_of_pattern(&self, path: &String) -> bool;
}

impl Mover for MvObj {
    fn create(source: String, target: String, pattern: String, exclude: bool) -> MvObj {
        MvObj {
            source,
            target,
            pattern,
            exclude,
        }
    }
    fn create_ref_str(source: &str, target: &str, pattern: &str, exclude: bool) -> MvObj {
        Self::create(source.to_string(), target.to_string(), pattern.to_string(), exclude)
    }
    fn print_src_files(&self) {
        let paths: Vec<String> = self.get_file_paths();
        for path in paths {
            println!("Path: {}", path);
        }
    }
    fn get_file_paths(&self) -> Vec<String> {
        let path = Path::new(self.source.as_str());
        let result = path.read_dir();
        let mut file_paths: Vec<String> = vec![];
        match result {
            Ok(result) => {
                for entry in result {
                    match entry {
                        Ok(entry) => {
                            let raw_file_path = entry.path();
                            let unfettered_file_path = raw_file_path.as_path().to_str();
                            let file_path: String;
                            if unfettered_file_path.is_some() {
                                file_path = unfettered_file_path.unwrap().to_string();
                            } else {
                                file_path = "".to_string();
                            }
                            if self.is_part_of_pattern(&file_path) {
                                file_paths.insert(file_paths.len(), file_path);
                            }
                        }
                        _ => { println!("Exception during Folder Scan") }
                    };
                }
            }
            _ => { println!("Exception during Folder Definition"); }
        };
        file_paths
    }

    fn is_part_of_pattern(&self, path: &String) -> bool {
        let regex_pattern = Regex::new(self.pattern.as_str());
        match regex_pattern {
            Ok(regex_pattern) => {
                regex_pattern.is_match(path.as_str())
            }
            _ => {
                println!("Exception during Pattern initialisation");
                false
            }
        }
    }
}