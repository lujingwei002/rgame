use std::env;


fn main() {
    tonic_build::configure().out_dir(env::current_dir().unwrap().join("src")).compile_protos(
        &["proto/role.proto"],&["proto"]).unwrap();
}