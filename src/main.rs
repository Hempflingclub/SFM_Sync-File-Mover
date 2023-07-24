use crate::file_handler::{Mover, MvObj};

mod file_handler;

fn main() {
    let movable_test= MvObj::create_ref_str(".","..",".*",false);
    movable_test.move_files();
}
