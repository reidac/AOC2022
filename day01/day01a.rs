use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

fn main() {
  let mut _maxval = 0;
  let mut _curval = 0;
  if let Ok(lines) = read_lines("./day01.txt") {
    for line in lines {
      if let Ok(ip) = line {
        println!("{}",ip);
        if ip.chars().count() > 0 {
          _curval += ip.parse::<i32>().unwrap();
        } else {
          println!("Value: {}",_curval);
	  if _curval > _maxval {
	    _maxval = _curval;
	  }
	  _curval = 0;
        }
      }
    }
    // After eof, need to check again.
    if _curval > _maxval {
      _maxval = _curval;
    }
    println!("Max value seen: {}.", _maxval);
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
