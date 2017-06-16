extern crate rand;

use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use rand::distributions::{IndependentSample, Range};
use rand::{thread_rng, Rng};
use std::path::Path;

struct RandomLiner {
  reader: BufReader<File>,
  positions: Vec<usize>,
}

impl RandomLiner {
  fn new<P: AsRef<Path>>(path: P) -> RandomLiner {
    let mut reader = BufReader::new(File::open(path).unwrap());
    let mut positions = vec![0];
    let mut current_position = 0;
    let mut _buffer = Vec::new();
    loop {
      let count = reader.read_until(b'\n', &mut _buffer).unwrap();
      if count == 0 {
        positions.pop().unwrap();
        break;
      }
      current_position += count;
      positions.push(current_position);
    }
    RandomLiner {
      reader: reader,
      positions: positions,
    }
  }

  fn get(&mut self, buf: &mut Vec<u8>) {
    if self.positions.is_empty() {
      panic!("no lines");
    }
    let mut rng = rand::thread_rng();
    let index = Range::new(0, self.positions.len()).ind_sample(&mut rng);
    self.reader.seek(SeekFrom::Start(self.positions[index] as u64)).unwrap();
    self.reader.read_until(b'\n', buf).unwrap();
  }
}

pub fn generate(s: &str) {
  let mut r = RandomLiner::new(s);
  let mut buf = Default::default();
  r.get(&mut buf);
  buf.pop();
  println!("{}-{:02x}", String::from_utf8(buf).unwrap(), thread_rng().gen::<u8>());
}
