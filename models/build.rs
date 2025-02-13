use std::env;
use std::fs;
use std::io::Write;
use std::ops::Add;
use std::path::Path;



fn main() {
    tonic_build::configure().out_dir(env::current_dir().unwrap().join("src")).compile_protos(
        &["proto/role.proto"],&["proto"]).unwrap();
}