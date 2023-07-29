pub struct FilterObject {
    path: String,
    filter: Vec<String>,
    is_regex: bool,
    is_inverted: bool
}

pub trait Filter{
    fn create(path: &String, filter: &String) -> FilterObject;
    fn split_arguments(filter: &String) -> Vec<String>;
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

impl Filter for FilterObject {
    fn create(path: &String, filter: &String) -> FilterObject {
        todo!()
    }

    fn split_arguments(filter: &String) -> Vec<String> {
        todo!()
    }

    fn is_in_filter(&self) -> bool {
        todo!()
    }

    fn is_in_regex(&self) -> bool {
        todo!()
    }

    fn start_with(&self, text: &String) -> bool {
        todo!()
    }

    fn ends_with(&self, text: &String) -> bool {
        todo!()
    }

    fn ends_with_ext(&self, text: &String) -> bool {
        todo!()
    }

    fn contains(&self, text: &String) -> bool {
        todo!()
    }

    fn contains_x_times(&self, text: &String, amounts: u16) -> bool {
        todo!()
    }

    fn get_filename(&self) -> String {
        todo!()
    }

    fn get_full_filename(&self) -> String {
        todo!()
    }

    fn get_extension(&self) -> String {
        todo!()
    }
}