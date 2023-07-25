use std::fs::File;
use std::io;
use std::ops::Add;
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
    fn create_ref_str_default(source: &str, target: &str, exclude: bool) -> MvObj;
    fn print_src_files(&self);
    fn print_target_files(&self);
    fn get_file_paths(&self, path: &String) -> Vec<String>;
    fn is_part_of_pattern(&self, path: &String) -> bool;
    fn move_targeted_files(&self, paths: Vec<String>);
    fn move_files(&self);
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
    fn create_ref_str_default(source: &str, target: &str, exclude: bool) -> MvObj {
        Self::create_ref_str(source, target, ".*", exclude)
    }
    fn print_src_files(&self) {
        let paths: Vec<String> = self.get_file_paths(&self.source.to_string());
        for path in paths {
            println!("Path: {}", path);
        }
    }
    fn print_target_files(&self) {
        let paths: Vec<String> = self.get_file_paths(&self.target.to_string());
        for path in paths {
            println!("Path: {}", path);
        }
    }
    fn get_file_paths(&self, path: &String) -> Vec<String> {
        let path = Path::new(path.as_str());
        let result = path.read_dir();
        let mut file_paths: Vec<String> = vec![];
        match result {
            Ok(result) => {
                for entry in result {
                    match entry {
                        Ok(entry) => {
                            let raw_file_path = entry.path();
                            let real_file_path = raw_file_path.as_path();
                            let unfettered_file_path = real_file_path.to_str();
                            let file_path: String;
                            if unfettered_file_path.is_some() {
                                file_path = unfettered_file_path.unwrap().to_string();
                            } else {
                                file_path = "".to_string();
                            }
                            if self.is_part_of_pattern(&file_path) {
                                file_paths.insert(file_paths.len(), file_path.to_string());
                            }
                            if Path::is_dir(real_file_path) {
                                let paths: Vec<String>;
                                paths = self.get_file_paths(&file_path);
                                for path in paths {
                                    file_paths.insert(file_paths.len(), path);
                                }
                            }
                        }
                        _ => {
                            println!("Exception during Folder Scan");
                            continue;
                        }
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

    fn move_targeted_files(&self, paths: Vec<String>) {
        if !Path::exists(Path::new(&self.target)) {
            std::fs::create_dir_all(Path::new(&self.target)).expect(&*format!("Failed to create target folder {}", self.target));
        }
        for path in paths {
            let source_path = Path::new(&*path);
            let source_parent_folder_raw = Path::parent(source_path);
            if !source_parent_folder_raw.is_some() {
                println!("Error finding parent folder of: {}", source_path.display().to_string());
                continue;
            }
            let source_parent_folder = source_parent_folder_raw.unwrap();
            let source_parent_relative_raw = Path::to_str(source_parent_folder);
            if !source_parent_relative_raw.is_some() {
                println!("Error finding relative folder of: {}", source_parent_folder.display().to_string());
                continue;
            }
            let mut source_parent_relative = source_parent_relative_raw.unwrap();
            source_parent_relative = &source_parent_relative[if source_parent_relative.len() > self.source.len() { self.source.len() + 1 } else { self.source.len() }..source_parent_relative.len()];
            let mut source_parent_relative_string: String = source_parent_relative.to_string();
            let target_parent_relative: String;
            // Fixing Slash after target
            target_parent_relative = self.target.to_string().add(if self.target.ends_with("\\") { &*source_parent_relative_string } else {
                source_parent_relative_string.insert_str(0, "\\");
                &*source_parent_relative_string
            });
            let target_parent_relative_path = Path::new(target_parent_relative.as_str());
            // Create relative Folder Path at Target
            if !Path::exists(target_parent_relative_path) {
                std::fs::create_dir_all(target_parent_relative_path).expect(&*format!("Couldn't create relative path {}", target_parent_relative));
            }
            let filename: String;
            if source_path.file_name().is_some() {
                filename = source_path.file_name().unwrap().to_string_lossy().parse().unwrap();
            } else {
                println!("Error parsing Unicode of: {}", path);
                continue;
            }
            let mut relative_filename: String;
            relative_filename = filename.to_string();

            let target_path_relative: String;
            // Fixing Slash before filename
            target_path_relative = target_parent_relative.to_string().add(if target_parent_relative.ends_with("\\") { &*relative_filename } else {
                relative_filename.insert_str(0, "\\");
                &*relative_filename
            });
            let target_path_relative_path: &Path = Path::new(&*target_path_relative);
            let source_file = File::open(source_path);
            //Reader
            let source_reader = match source_file {
                Ok(source_file) => {
                    let source_reader = io::BufReader::new(source_file);
                    Some(source_reader)
                }
                _ => {
                    println!("Failed to open file: {}", path);
                    None
                }
            };
            if !source_reader.is_some() { continue; }
            //Writer
            let target_file;
            if !Path::exists(target_path_relative_path) {
                target_file = File::create(target_path_relative_path);
            } else {
                target_file = File::open(target_path_relative_path);
            }
            let target_writer = match target_file {
                Ok(target_file) => {
                    let target_writer = io::BufWriter::new(target_file);
                    Some(target_writer)
                }
                _ => {
                    println!("Failed to write file: {}", target_path_relative);
                    None
                }
            };
            if !target_writer.is_some() { continue; }
            std::io::copy(&mut source_reader.unwrap(), &mut target_writer.unwrap()).expect(&*format!("Failed to copy: {} | to: {}", path.to_string(), target_path_relative.to_string()));
            /*
            Check Checksum of source and target -> delete original
             */
        }
    }

    fn move_files(&self) {
        self.move_targeted_files(self.get_file_paths(&self.source))
    }
}