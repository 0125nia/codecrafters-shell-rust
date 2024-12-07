pub fn get_path(arg: &String) -> Option<String> {
    let paths = std::env::var("PATH").unwrap();
    paths
        .split(':')
        .find(|&path| std::fs::metadata(format!("{}/{}", path, arg)).is_ok())
        .map(|path| format!("{}/{}", path, arg))
}
