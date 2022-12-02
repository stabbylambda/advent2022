use std::fmt::Debug;
use std::str::FromStr;

pub mod test;
pub mod util;

pub fn get_input_strings() -> Vec<String> {
    get_input(|s| s.to_owned())
}

fn get_current_path() -> String {
    let e = std::env::current_exe().unwrap();
    let p = e.file_name().unwrap().to_str().unwrap();
    p.to_owned()
}

pub fn get_raw_input() -> String {
    util::get_raw_input(&get_current_path())
}

pub fn get_input<T, F>(f: F) -> Vec<T>
where
    F: Fn(&str) -> T,
{
    let path = format!("./{}/input.txt", get_current_path());
    util::get_input(&path, f)
}

pub fn get_numbers<T>() -> Vec<T>
where
    T: FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    let path = format!("./{}/input.txt", get_current_path());
    util::get_numbers(&path)
}
