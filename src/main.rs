use std::{collections::HashMap, fs::read_to_string, ptr::null};

/// .
fn main() {
    let mut emps = Vec::new();
    emps.push("mengyao");
    emps.push("yangyan");

    let mut dep_map = HashMap::new();

    dep_map.insert(String::from("dev"), vec!["yangyan"]);
    dep_map.insert(String::from("it"), vec!["mengyao", "liqiang"]);
    dep_map.insert(String::from("po"), vec![]);

    dep_map.entry(String::from("qa")).or_insert(vec!["lalala"]);
    dep_map
        .entry(String::from("po"))
        .and_modify(|v| v.push("xun"));

    let d = dep_map.contains_key("dev");

    println!("deps {:?} , {} ", dep_map, d);

    let c = read_content();

    let content = c;

    println!("{}", content.ok().unwrap())
}

fn read_content() -> Result<String, std::io::Error> {
    read_to_string("/etc/hosts")
}
