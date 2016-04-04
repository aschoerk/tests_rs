
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io;
use std::path::Path;
use std::env;

fn read_file(name: &str) {
    let path = Path::new(&name);
    let mut linenum = 0;    
    if path.exists() {        
        let mut file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
        let mut datafound = false;
        for line in reader.lines() {
            let line = line.unwrap();
            linenum += 1;
            println!("line: {}: {}",linenum, line);
            
            let mut chars = line.chars();
            println!("char: {}",chars.next().unwrap());
            println!("char: {}",chars.next().unwrap());
            
        }
    }    
}

fn main() {
    // let name = env::args().nth(1).unwrap();
    // println!("arg: {}",env::args().nth(1).unwrap());
    read_file("NewP7-2009-03-16.rle");
}

#[cfg(test)]
mod tests {
    use super::read_file;

    #[test]
    fn read_new_p7_2009_03_16() {
        read_file("NewP7-2009-03-16.rle");
    }
}
