use path_clean::PathClean;
use std::borrow::Cow;
use std::env;
use std::path;

fn absolutify(arg: &str) -> Cow<str> {
    let mut path_buf = path::PathBuf::from(arg);
    if !path_buf.is_absolute() {
        let cur_dir = if let Ok(cur_dir) = env::current_dir() {
            cur_dir
        } else {
            return Cow::from(arg);
        };
        path_buf = cur_dir.join(path_buf);
    }
    Cow::from(path_buf.clean().to_string_lossy().to_string())
}

fn main() {
    let mut first = true;
    let args: Vec<_> = env::args().skip(1).collect();
    for arg in args {
        if first {
            first = false;
        } else {
            print!(" ");
        }
        print!("{}", absolutify(&arg));
    }
}

#[test]
fn test() {
    assert_ne!(absolutify("~/."), "~/.".to_string());
    assert_ne!(absolutify("~/.."), "~/..".to_string());
    assert_ne!(absolutify("./~/.."), "./~/..".to_string());
}
