use crate::file_handler::{Mover, MvObj};

mod file_handler;

fn main() {
    let movable_test= MvObj::create_ref_str_default(".","..",false);
    movable_test.move_files();
    let source_files = movable_test.get_file_paths(&movable_test.source);
    for file in source_files{
        let timestamp = movable_test.get_newest_timestamp_recursive(&file);
        println!("Newest Timestamp of {} is: {:?}",file,timestamp);
    }
}
