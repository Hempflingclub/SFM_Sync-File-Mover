mod file_handler;

use std::env;
use std::thread::sleep;
use std::time::Duration;
use crate::file_handler::{Mover, MvObj};


fn main() {
    let args: Vec<String> = env::args().collect();
    let args: Vec<String> = args[1..args.len()].to_vec();
    let mv_obj_list: Vec<MvObj> = argument_parser(args);
    if mv_obj_list.len().le(&0) {
        println!("No Valid Mv_Obj's were created");
        return;
    }
    loop
    {
        sleep(Duration::from_secs(30u64));
        for index in 0..mv_obj_list.len() {
            let mv_obj = &mv_obj_list[index];
            mv_obj.move_files();
        }
    }
}

fn argument_parser(raw_args: Vec<String>) -> Vec<MvObj> {
    const NEW_OBJ_FLAG: &str = "--new";
    const SRC_FLAG: &str = "--src";
    const TRG_FLAG: &str = "--target";
    let mut old_index = 0u32;
    let mut src_index = 0u32;
    let mut trg_index = 0u32;
    let mut mv_obj_list: Vec<MvObj> = vec![];
    for index in 0..raw_args.len() as u32 {
        let arg: &str = &*raw_args[index as usize];
        match arg {
            NEW_OBJ_FLAG => {
                if !(src_index == (trg_index - 2)) && !(old_index >= src_index) {
                    println!("call --src *PATH* --target *PATH* right after --new (for each obj the last statement is --new)");
                    continue;
                }
                let src: String = raw_args[(src_index + 1) as usize].to_string();
                let target: String = raw_args[(trg_index + 1) as usize].to_string();
                let filter: Vec<String> = raw_args[(trg_index + 2) as usize..index as usize].to_vec();
                let mut total_filter: String = "".to_string();
                for unchecked_filter in filter {
                    total_filter.push_str(&*unchecked_filter);
                    total_filter.push_str(" ");
                }
                let mv_obj: MvObj = argument_handler(src.as_str(), target.as_str(), total_filter.as_str());
                mv_obj_list.push(mv_obj);
                old_index = index;
                continue;
            }
            SRC_FLAG => {
                src_index = index;
                continue;
            }
            TRG_FLAG => {
                trg_index = index;
                continue;
            }
            _ => { continue; }
        }
    }
    mv_obj_list
}

fn argument_handler(source_path: &str, target_path: &str, filter: &str) -> MvObj {
    MvObj::create_ref_str(source_path, target_path, filter)
}