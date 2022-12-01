use std::fmt::Debug;
use std::str::FromStr;

pub mod test;
pub mod util;

pub fn get_input_strings() -> Vec<String> {
    get_input(|s| s.to_owned())
}

pub fn get_input<T, F>(f: F) -> Vec<T>
where
    F: Fn(&str) -> T,
{
    let e = std::env::current_exe().unwrap();
    let p = e.file_name().unwrap().to_str().unwrap();

    let path = format!("./{p}/input.txt");
    util::get_input(&path, f)
}

pub fn get_numbers<T>() -> Vec<T>
where
    T: FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    let e = std::env::current_exe().unwrap();
    let p = e.file_name().unwrap().to_str().unwrap();

    let path = format!("./{p}/input.txt");
    util::get_numbers(&path)
}
