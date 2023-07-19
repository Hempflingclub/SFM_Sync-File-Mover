use std::path::Path;

pub struct MvObj {
    pub source: String,
    pub target: String,
    pub pattern: String,
    pub exclude: bool,
}

pub trait Mover {
    fn create(source: String, target: String, pattern: String, exclude: bool) -> MvObj;
    fn create_ref_str(source: &str, target: &str, pattern: &str, exclude: bool) -> MvObj;
    fn print_info(&self);
    fn print_src_files(&self);
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
    fn print_info(&self) {
        println!("src: {} | dest: {} | ptrn: {} | excl: {}", self.source, self.target, self.pattern, self.exclude)
    }
    fn print_src_files(&self) {
        let path = Path::new(self.source.as_str());
        let result = path.read_dir();
        match result {
            Ok(result) => {
                for entry in result{
                    match entry {
                        Ok(entry) => {
                            println!("File: {} | Path: {} | File?: {:?} | Metadata: {:?}",entry.file_name().to_str().unwrap(),entry.path().to_str().unwrap(),entry.file_type().unwrap(),entry.metadata().unwrap())

                        }
                        _ => {}
                    };
                }

            }
            _ => {}
        };
    }
}