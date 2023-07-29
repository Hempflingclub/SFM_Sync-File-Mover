use std::path::Path;
use regex::Regex;

pub struct FilterObject {
    path: String,
    filter: Vec<String>,
    is_regex: bool,
    is_inverted: bool
}

pub trait Filter{
    fn create(path: &String, filter: &String) -> FilterObject;
    fn is_in_filter(&self) -> bool;
    fn is_in_regex(&self) -> bool;
    fn start_with(&self, text:&String) -> bool;
    fn ends_with(&self, text:&String) -> bool;
    fn ends_with_ext(&self, text:&String) -> bool;
    fn contains(&self, text:&String) -> bool;
    fn contains_x_times(&self, text:&String, amounts:u16) -> bool;
    fn get_filename(&self) -> String;
    fn get_full_filename(&self) -> String;
    fn get_extension(&self) -> String;
}

fn split_arguments(filter: &String) -> Vec<String> {
    let mut arguments:Vec<String> = vec![];
    let mut old_index:usize=0;
    for index in 0..filter.len(){
        if  filter[index..index].eq(" "){
            let argument = (&*filter.to_string())[old_index..index].trim().to_string();
            arguments.insert(argument.len(),argument);
            old_index = index;
        }
    }
    arguments
}

impl Filter for FilterObject {
    fn create(path: &String, filter: &String) -> FilterObject {
        let is_regex:bool;
        let is_inverted:bool;
        let filter_list:Vec<String>;
        if filter.starts_with("-"){
            is_regex=false;
            is_inverted=false; // TODO: parse argument to check for inversion flag
            filter_list = split_arguments(filter);
        } else{
            is_regex=true;
            is_inverted=false;
            filter_list = vec![filter.to_string()];
        }

        FilterObject { path: path.to_string(), filter: filter_list, is_regex, is_inverted }
    }

    fn is_in_filter(&self) -> bool {
        todo!()
    }

    fn is_in_regex(&self) -> bool {
        let regex_pattern = Regex::new(&*self.filter[0]);
        match regex_pattern {
            Ok(regex_pattern) => {
                regex_pattern.is_match(&*self.get_full_filename())
            }
            _ => {
                println!("Exception during Pattern initialisation");
                false
            }
        }
    }

    fn start_with(&self, text: &String) -> bool {
        self.get_filename().starts_with(text)
    }

    fn ends_with(&self, text: &String) -> bool {
        self.get_filename().ends_with(text)
    }

    fn ends_with_ext(&self, text: &String) -> bool {
        self.get_extension().eq(text)
    }

    fn contains(&self, text: &String) -> bool {
        self.get_filename().contains(text)
    }

    fn contains_x_times(&self, text: &String, amounts: u16) -> bool {
        let mut counts:u16=0;
        let filename = self.get_filename();
        if text.len() > filename.len() {return false}
        for index in 0u16..filename.len() as u16{
            let slice = (&*filename)[index as usize..text.len()].to_string();
            if !(slice.eq(text)){continue}
            counts += 1;
        }
        counts==amounts
    }

    fn get_filename(&self) -> String {
        let filename:String;
        let full_filename = self.get_full_filename();
        let mut last_index:usize=0;
        for index in 0..full_filename.len(){
            let slice = full_filename[index..index].to_string();
            if slice.eq(".") {last_index=index}
        }
        filename = (&*full_filename)[0..(last_index-1)].to_string();
        filename
    }

    fn get_full_filename(&self) -> String {
        fn nothing() -> String{
            "".to_string()
        }
        let file_path = Path::new(&*self.path);
        let unfiltered_filename =  file_path.file_name();
        if unfiltered_filename.is_none(){
            return nothing()
        }
        let filename = unfiltered_filename.unwrap();
        if filename.to_str().is_none(){
            return nothing()
        }
        let filename_string = filename.to_str().unwrap().to_string();
        filename_string
    }

    fn get_extension(&self) -> String {
        let extension:String;
        let full_filename = self.get_full_filename();
        let mut last_index:usize=0;
        for index in 0..full_filename.len(){
            let slice = full_filename[index..index].to_string();
            if slice.eq(".") {last_index=index}
        }
        extension = (&*full_filename)[(last_index+1)..full_filename.len()].to_string();
        extension
    }
}