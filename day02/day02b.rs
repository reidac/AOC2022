use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

fn main() {
  let mut delta_res = 0;
  let mut res = 0;
  let _loss = 0;
  let _draw = 3;
  let _win = 6;
  if let Ok(lines) = read_lines("./day02.txt") {
    for line in lines {
      let ln = &line.unwrap();
      let c1 = &ln[0..1];
      let c2 = &ln[2..3];
      println!("{} - {}",c1,c2);
      if c1=="A" {
        if c2=="X" {
	  delta_res = _loss+3;
	}
	if c2=="Y" {
	  delta_res = _draw+1;
	}
	if c2=="Z" {
	  delta_res = _win+2;
	}
      }
      if c1=="B" {
        if c2=="X" {
	  delta_res = _loss+1;
	}
	if c2=="Y" {
	  delta_res = _draw+2;
	}
	if c2=="Z" {
	  delta_res = _win+3;
	}
      }
      if c1=="C" {
        if c2=="X" {
	  delta_res = _loss+2; 
	}
	if c2=="Y" {
	  delta_res = _draw+3; 
	}
	if c2=="Z" {
	  delta_res = _win+1; 
	}
      }
      println!("Delta-res: {}",delta_res);
      res += delta_res;
    }
    println!("Score: {}",res);
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
