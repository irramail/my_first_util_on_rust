use std::fs;

fn main() {
    let need = String::from("1need.txt");
    let full = String::from("full.txt");

    let contents = fs::read_to_string(need)
        .expect("Something went wrong reading the file");

    let full_contents = fs::read_to_string(full)
        .expect("Something went wrong reading the file");

    for line in contents.lines() {
        for line2 in full_contents.lines() {
            if line == &line2[..line2.find('\t').unwrap_or(line2.len())] {
                println!("{}", line2);
            }
        }
    }
}
