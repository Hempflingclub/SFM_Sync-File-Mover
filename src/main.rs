use crate::file_handler::{Mover, MvObj};

mod file_handler;

fn main() {
    let movable_test= MvObj::create_ref_str_default(".","..",false);
    movable_test.move_files();
}
