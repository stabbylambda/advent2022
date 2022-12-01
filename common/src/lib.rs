use std::fmt::Debug;
use std::{fs, str::FromStr};

fn _get_input<T, F>(path: &str, f: F) -> Vec<T>
where
    F: Fn(&str) -> T,
{
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    contents.split('\n').map(f).collect()
}

fn _get_numbers<T>(path: &str) -> Vec<T>
where
    T: FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let input = contents.split('\n').next().unwrap();
    let input: Vec<T> = input.split(',').map(|s| s.parse::<T>().unwrap()).collect();
    input
}

pub fn get_input<T, F>(f: F) -> Vec<T>
where
    F: Fn(&str) -> T,
{
    let e = std::env::current_exe().unwrap();
    let p = e.file_name().unwrap().to_str().unwrap();

    let path = format!("./{p}/input.txt");
    _get_input(&path, f)
}

pub fn get_numbers<T>() -> Vec<T>
where
    T: FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    let e = std::env::current_exe().unwrap();
    let p = e.file_name().unwrap().to_str().unwrap();

    let path = format!("./{p}/input.txt");
    _get_numbers(&path)
}

pub fn get_test_input<T, F>(f: F) -> Vec<T>
where
    F: Fn(&str) -> T,
{
    _get_input("./test.txt", f)
}
pub fn get_test_numbers<T>() -> Vec<T>
where
    T: FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    _get_numbers("./test.txt")
}
