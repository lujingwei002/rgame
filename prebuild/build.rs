use std::env;
use std::fs;
use std::io::Write;
use std::ops::Add;
use std::path::Path;
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap_or_else(|_| "target/build".to_string());
    let out_dir_path = Path::new(&out_dir);
    fs::create_dir_all(out_dir_path).unwrap(); // 创建目录（如果尚未存在）
    let script_path = out_dir_path.join("custom_build_script.sh"); // 自定义脚本的路径

    // 生成自定义的构建脚本，这可以包含任意内容，比如配置步骤或生成某些文件。
    let script_content = r#"
#!/bin/sh
# 这里是一些自定义的构建脚本代码或命令行命令111
echo "This is a custom build script" > /path/to/some/file.txt
    "#;
    fs::write(&script_path, script_content).unwrap(); // 写入脚本内容到文件系统。
    // 赋予文件执行权限（如果需要）的 shell 命令。这一步依赖于系统配置和脚本的实际内容。
    // 请注意：这一步可能不必要或无法工作，具体取决于你的需求和操作系统。
    // ... 执行其他自定义的构建步骤 ...
    tonic_build::configure().out_dir(env::current_dir().unwrap().join("src")).compile_protos(
        &["proto/helloworld.proto"],&["proto"]).unwrap();
}