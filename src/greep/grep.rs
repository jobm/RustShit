use anyhow::Error;
use std::{collections::HashMap, io::Read};
/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
#[derive(Debug)]
pub struct Flags(Vec<String>);

impl Flags {
    pub fn new(flags: &'static [&'static str]) -> Self {
        // *: Order of flags => ["-i", "-x", "-n", "-v", "-l"]
        // ## Each flag needs to search for the pattern, seperate
        // ## out searching, then apply the flags to each search accor-
        // ## dingly.
        // 1: -i (search all),
        // 2: -x (search only lines that exactly match),
        // 3: -n (add line # to results of -i),
        // #: might need to swap 2,3 to be more effecient.
        // 4: -v (get the inverse of the search results so far),
        // 5: -l (if anything matches, return the file name).
        let keys = ["-i", "-x", "-n", "-v", "-l"];
        let mut ord_flags = flags.to_vec();
        let ord_map: HashMap<_, _> = keys.iter().enumerate().map(|(i, &val)| (val, i)).collect();
        ord_flags.sort_by_key(|item| ord_map.get(&item[..]).unwrap());
        Flags::from_vec(ord_flags.clone().iter().map(|f| f.to_string()).collect())
    }

    fn from_vec(flags: Vec<String>) -> Self {
        Flags(flags)
    }
}

struct LocalFile;

impl LocalFile {
    fn new(file: &str) -> Result<String, Error> {
        let abs_path: &str = "/Users/jobm/Success/algos_and_ds/rust-test/src";
        let mut file_content = String::new();
        let mut file = std::fs::File::open(format!("/{abs_path}/{file}"))?;
        file.read_to_string(&mut file_content)?;
        Ok(file_content)
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    // can work together; {-n, -i}
    // circuting; {p2->(-v), p1->(-x) p0->(-l)}
    let mut lines = Vec::new();
    let mut buff = String::new();
    for file in files {
        for flag in &flags.0 {
            match flag.as_str() {
                "-i" => {
                    buff = match_any_case(get_file_contents(file), pattern, &flags.0.clone());
                    lines.push(buff.clone())
                }
                "-x" => {
                    let mut file_content = get_file_contents(file);
                    if flags.0.contains(&"-i".to_string()) {
                        file_content = buff.clone();
                    }

                    buff = match_exact(file_content.clone(), pattern, &flags.0);
                    // println!("BUFF | FC: {:?} | {:?},", buff, file_content);
                    lines = vec![buff.clone()]
                }
                "-n" => {
                    println!("Here:3");
                    buff = line_file_name(pattern, buff, file, flags.0.contains(&"-i".to_string()));
                    // if res.is_some() {
                    //     lines.push(res.unwrap());
                    // }
                }
                "-l" => {
                    //println!("Here:2");
                    // lines.push(line_num(
                    //     pattern,
                    //     get_file_contents(file),
                    //     flags.0.contains(&"-i".to_string()),
                    // ));
                }
                _ => {}
            }
        }
    }

    fn extract_line(line: &str) -> Option<&str> {
        if let Some(sc) = line.find(":") {
            Some(line[sc + 1..].trim())
        } else {
            if !line[..1].eq("") {
                return Some(line);
            }
            None
        }
    }

    fn match_exact(file_buff: String, pattern: &str, flags: &Vec<String>) -> String {
        let mut res = String::new();
        fn match_ignore_case<'a>(line: &'a str, pattern_: &'a str) -> &'a str {
            if line.eq_ignore_ascii_case(pattern_) {
                return line;
            }
            ""
        }
        for (i, line) in file_buff.split('\n').enumerate() {
            if line.eq("") {
                continue;
            }
            let line_no_num = extract_line(line);
            if line_no_num.is_none() {
                if flags.contains(&"-i".to_owned()) {
                    res.extend([format!(
                        "{}: {}",
                        i,
                        match_ignore_case(line_no_num.unwrap(), pattern)
                    )])
                } else {
                    if line_no_num.unwrap().eq(pattern) {
                        res.extend([format!("{}: {}", i, line_no_num.unwrap())])
                    }
                }
            }
            if flags.contains(&"-i".to_owned()) {
                res.extend([match_ignore_case(line_no_num.unwrap(), pattern)])
            } else {
                if line_no_num.eq(&Some(pattern)) {
                    res.extend([line_no_num.unwrap()])
                }
            }
        }
        res
    }

    fn match_any_case(file_buff: String, pattern: &str, flags: &Vec<String>) -> String {
        let mut res = String::new();
        for (i, line) in file_buff.lines().enumerate() {
            if line.to_lowercase().contains(&pattern.to_lowercase()) {
                if flags.contains(&"-n".to_string()) {
                    res.extend([format!("{}:", i + 1).as_str(), line]);
                    res.push('\n');
                } else {
                    res.extend([line]);
                }
            }
        }
        res
    }

    fn get_file_contents(file: &str) -> String {
        // let mut res = String::new(); // see if we can avoid creating a new string every time.
        let file_contents = LocalFile::new(file);
        if file_contents.is_err() {
            return String::new();
        }
        file_contents.ok().unwrap()
    }

    // -n
    fn line_num(pattern: &str, file_buff: String, is_cstive: bool) -> String {
        let mut res = String::new();
        for (i, line) in file_buff.lines().enumerate() {
            if is_cstive {
                if line.to_lowercase().contains(&pattern.to_lowercase()) {
                    res.extend([format!("{}:", i + 1).as_str(), line])
                }
            } else {
                if line.contains(pattern) {
                    res.extend([format!("{}:", i + 1).as_str(), line])
                }
            }
        }
        res
    }

    // -l
    fn line_file_name(pattern: &str, file_buff: String, file: &str, is_cstv: bool) -> String {
        for line in file_buff.lines() {
            if is_cstv {
                if line.to_lowercase().contains(&pattern.to_lowercase()) {
                    return String::from(file);
                }
            } else {
                if line.contains(pattern) {
                    return String::from(file);
                }
            }
        }
        String::new()
    }
    Ok(lines)
}
