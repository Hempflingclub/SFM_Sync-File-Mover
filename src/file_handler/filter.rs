use std::path::Path;
use regex::Regex;

pub(self) struct FilterObject {
    path: String,
    filter: Vec<String>,
    is_regex: bool,
}

pub(self) trait Filter {
    fn create(path: &String, filter: &String) -> FilterObject;
    fn is_in_filter(&self) -> bool;
    fn handle_parameters(&self) -> bool;
    fn is_in_regex(&self) -> bool;
    fn start_with(&self, text: &String) -> bool;
    fn ends_with(&self, text: &String) -> bool;
    fn ends_with_ext(&self, text: &String) -> bool;
    fn contains(&self, text: &String) -> bool;
    fn contains_x_times(&self, text: &String, amounts: u16) -> bool;
    fn get_filename(&self) -> String;
    fn get_full_filename(&self) -> String;
    fn get_extension(&self) -> String;
}

pub(self) enum FilterType {
    StartsWith,
    EndsWith,
    EndsWithExt,
    Contains,
    ContainsXTimes,
    Invert,
    MatchAll,
    NONE,
}

/// Basically the interface to use filter
pub(super) fn use_filter(path: &String, filter: &String) -> bool {
    let filter = FilterObject::create(path, filter);
    filter.is_in_filter()
}

fn split_arguments(filter: &String) -> Vec<String> {
    let mut arguments: Vec<String> = vec![];
    let mut old_index: usize = 0;
    for index in 0..filter.len() {
        if filter[index..index].eq(" ") {
            let argument = (&*filter.to_string())[old_index..=index].trim().to_string();
            arguments.push(argument);
            old_index = index;
        }
    }
    arguments
}

fn get_parameter(parameters: Vec<String>) -> (FilterType, Vec<String>) {
    let filter_type: FilterType;
    let mut final_params: Vec<String> = vec![];
    const MAX_PARAMS: u8 = 3u8; // May be adjusted in the future
    if parameters[0].starts_with("--") && parameters.len().ge(&2) && parameters.len().le(&(MAX_PARAMS as usize)) {
        let flag = &parameters[0][2..parameters[0].len()]; // Take Parameter slice off "--" and type_cast &str
        filter_type = match flag {
            "starts_with" => { FilterType::StartsWith }
            "ends_with" => { FilterType::EndsWith }
            "ends_with_ext" => { FilterType::EndsWithExt }
            "contains" => { FilterType::Contains }
            "contains_x_times" => { FilterType::ContainsXTimes }
            "invert" => { FilterType::Invert }
            "match_all" => { FilterType::MatchAll }
            _ => {
                FilterType::NONE
            }
        };
        for param in parameters[1..MAX_PARAMS as usize].to_vec() {
            final_params.push(param);
        }
    } else {
        filter_type = FilterType::NONE;
    }
    (filter_type, final_params)
}

impl Filter for FilterObject {
    fn create(path: &String, filter: &String) -> FilterObject {
        let is_regex: bool;
        let filter_list: Vec<String>;
        if filter.starts_with("--") {
            is_regex = false;
            filter_list = split_arguments(filter);
        } else {
            is_regex = true;
            filter_list = vec![filter.to_string()];
        }

        FilterObject { path: path.to_string(), filter: filter_list, is_regex }
    }

    fn is_in_filter(&self) -> bool {
        if self.is_regex {
            self.is_in_regex()
        } else {
            self.handle_parameters()
        }
    }
    /// Will match all passed parameters in as a logical OR
    /// match_all will change behaviour to an logical AND
    fn handle_parameters(&self) -> bool {
        let parameters: Vec<String> = self.filter.to_vec();
        let mut matches_params: bool = false;
        let mut matches: Vec<bool> = vec![];
        let mut invert: bool = false;
        let mut match_all: bool = false;
        for index in 0..parameters.len() {
            let parameter_slice = parameters[index..parameters.len()].to_vec();
            let (filter_type, params) = get_parameter(parameter_slice);
            match filter_type {
                FilterType::StartsWith => {
                    matches.push(self.start_with(&params[0]));
                }
                FilterType::EndsWith => {
                    matches.push(self.ends_with(&params[0]));
                }
                FilterType::EndsWithExt => {
                    matches.push(self.ends_with_ext(&params[0]));
                }
                FilterType::Contains => {
                    matches.push(self.contains(&params[0]));
                }
                FilterType::ContainsXTimes => {
                    matches.push(self.contains_x_times(&params[0], params[1].parse::<u16>().expect("Failed to parse number from argument 'contains_x_times'")));
                }
                FilterType::Invert => {
                    invert = true;
                }
                FilterType::MatchAll => {
                    match_all = true;
                }
                FilterType::NONE => {
                    continue;
                }
            };
        }
        if match_all { matches_params = true }
        for m in matches {
            if if match_all { !m } else { m } {
                matches_params = m;
                break;
            }
        }
        matches_params && !invert
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
        let mut counts: u16 = 0;
        let filename = self.get_filename();
        if text.len() > filename.len() { return false; }
        for index in 0u16..filename.len() as u16 {
            let slice = (&*filename)[index as usize..text.len()].to_string();
            if !(slice.eq(text)) { continue; }
            counts += 1;
        }
        counts == amounts
    }

    fn get_filename(&self) -> String {
        let filename: String;
        let full_filename = self.get_full_filename();
        let mut last_index: usize = 0;
        for index in 0..full_filename.len() {
            let slice = full_filename[index..index].to_string();
            if slice.eq(".") { last_index = index }
        }
        filename = (&*full_filename)[0..last_index].to_string();
        filename
    }

    fn get_full_filename(&self) -> String {
        fn nothing() -> String {
            "".to_string()
        }
        let file_path = Path::new(&*self.path);
        let unfiltered_filename = file_path.file_name();
        if unfiltered_filename.is_none() {
            return nothing();
        }
        let filename = unfiltered_filename.unwrap();
        if filename.to_str().is_none() {
            return nothing();
        }
        let filename_string = filename.to_str().unwrap().to_string();
        filename_string
    }

    fn get_extension(&self) -> String {
        let extension: String;
        let full_filename = self.get_full_filename();
        let mut last_index: usize = 0;
        for index in 0..full_filename.len() {
            let slice = full_filename[index..index].to_string();
            if slice.eq(".") { last_index = index }
        }
        extension = (&*full_filename)[(last_index + 1)..full_filename.len()].to_string();
        extension
    }
}