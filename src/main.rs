mod file_handler;

use crate::file_handler::{Mover, MvObj};


fn main() {
    // TODO: implement argument to be passed
    // TODO: argument parsing to make multiple MvObj possible
    let movable_test = MvObj::create_ref_str_default(".", "..");
    movable_test.move_files();
}
