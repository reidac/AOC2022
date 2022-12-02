use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

struct TopThree {
  data: [i32;3],
}

impl TopThree {
  fn update(&mut self, v:i32) {
    if v > self.data[2] {
      self.data[0] = self.data[1];
      self.data[1] = self.data[2];
      self.data[2] = v;
      return;
    }
    if v > self.data[1] {
      self.data[0] = self.data[1];
      self.data[1] = v;
      return;
    }
    if v > self.data[0] {
      self.data[0] = v;
    }
  }
  fn sum(&self) -> i32 {
    return self.data[0]+self.data[1]+self.data[2];
  }
}

fn main() {
  let mut _tops = TopThree{ data: [0;3] };
  let mut _curval = 0;

  if let Ok(lines) = read_lines("./day01.txt") {
    for line in lines {
      if let Ok(ip) = line {
        println!("{}",ip);
        if ip.chars().count() > 0 {
          _curval += ip.parse::<i32>().unwrap();
        } else {
          println!("Value: {}",_curval);
	  _tops.update(_curval);
	  _curval = 0;
        }
      }
    }
    // After eof, need to check again.
    _tops.update(_curval);
    println!("Top value seen: {}.", _tops.data[2]);
    println!("Sum of top 3: {}.", _tops.sum() );
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
