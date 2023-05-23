use std::{fs::create_dir_all, path::PathBuf};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{args:?}");
    let project_dir = args.get(1).expect("Need to pass directory name");
    let path: PathBuf = project_dir.into();

    let src_path = path.join("src");
    println!("Path to src dir: {src_path:?}");
    create_dir_all(src_path).expect("Failed to create src dir");
}
