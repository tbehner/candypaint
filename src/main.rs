extern crate clap;

use clap::{App,Arg}; 
use std::env;
use std::path::Path;

 
fn main() { 
    let matches = App::new("candypaint")
       .version("0.1.0")
       .about("candy coated prompts for the ion shell")
       .author("Coleman Emery McFarland")
       .arg(Arg::with_name("theme").help("theme name").required(false))
       .get_matches(); 

    let prompt = match matches.value_of("theme") {
        Some(theme) => {
            match theme {
                "chad" => chad(),
                "darkside" => darkside(),
                _ => chad(),
            }
        },
        _ => None, 
    };

    println!("{}", prompt.unwrap_or(String::from("# ${c::reset}")));
}

/// chad is our default theme.
fn chad() -> Option<String> {

    let mut ret = String::new();

    let range: Vec<i32> = (0xd6..0xde).rev().collect();

    if let Ok(user) = env::var("USER") {
        for (i, c) in user.chars().enumerate() {
            if let Some(num) = range.get(i) {
                ret.push_str("${c::0x");
                let s = format!("{:X},bold}}{}", num, c);
                ret.push_str(&s);
            } else { 
                break 
            }
        }
        ret.push_str("${c::0xd7}:")
    }

    if let Ok(path) = env::current_dir() {
        if let Some(pwd) = path.file_name() {
            ret.push_str("${c::0xd6}");
            let s = format!("{}", pwd.to_str().unwrap_or(""));
            ret.push_str(&s)
        }
    }

    if is_git() {
        ret.push_str(" (${git::branch}) ${c::0x05}# ${c::reset}");
    } else {
        ret.push_str(" ${c::0x05}# ${c::reset}");
    }

    Some(ret)
}

fn darkside() -> Option<String> {
    let mut ret = String::new();

    // fixed width
    // dark -> light
    // 45 chars
    //

    // push current dir
    if let Ok(path) = env::current_dir() {
        let p = path.as_path();
        if let Some(s) = p.to_str() {
            ret.push_str(s);
        }
    }

    // black -> light grey
    let range: Vec<i32> = (0xe8..0xfe).collect();

    let mut temp = String::new();
    let length = ret.len();

    let mut idx = 0;
    for (i, c) in ret.chars().enumerate() {
        let mut inc = false;
        if length > range.len() {
            if i < (length - range.len()) {
                continue
            }
            inc = true;
        } else {
            inc = true;
        }
        if let Some(num) = range.get(idx) {
            temp.push_str("${c::0x");
            let s = format!("{:X},bold}}{}", num, c);
            temp.push_str(&s);
        } else { 
            break 
        }
        if inc {
            idx += 1; 
        }
    }
    temp.push_str(" ${c::0x34}>> ${c::reset}");

    Some(temp)
}

fn is_git() -> bool {
    Path::new(".git").exists()
}


fn apply_range(s: &mut String, range: Vec<i32>) {

    let length = s.len();
    // how many of our range do we need to repeat to get to length?
    let repeat = length / range.len();
    let mut new_s = String::new(); 

    for (i, x) in s.chars().enumerate() {

    }

}
