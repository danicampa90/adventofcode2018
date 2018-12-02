use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::thread;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn read_file_to_str(fname: &str) -> Result<String, std::io::Error> {
    println!("Loading {0}", fname);
    let mut file : File = File::open(Path::new(fname))?;
    let mut s = String::new();
    let size = file.read_to_string(&mut s)?;
    println!("Read {} bytes",size );
    return Ok(s)
}

#[derive(Debug)]
pub enum FileProcessingErr<E>{
    IoError(std::io::Error),
    ProcessingError(E)
}

impl<E> std::convert::From<std::io::Error> for FileProcessingErr<E> {
    fn from(err: std::io::Error) -> Self {
        FileProcessingErr::IoError(err)
    }
}

pub fn read_file_foreach_line<F, E>(fname: &str, action: &mut F) -> Result<(), FileProcessingErr<E>>
    where F: FnMut(String)-> Result<(), E> 
{
    let f = try!(File::open(fname));
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        if let Err(res) = action(l)
        {
            return Err(FileProcessingErr::ProcessingError::<E>(res));
        }
        
    }
    Ok(())
}

pub fn read_file_lines_in_chan(fname: &str) -> Result<Receiver<String>, std::io::Error>
{
    let f = try!(File::open(fname));
    let (sender, recv) = channel();
    thread::spawn(move || {
        let file = BufReader::new(&f);
        
        for line in file.lines() {
            let l = line.unwrap();
            if sender.send(l).is_err() {return;};
        } 
    });
    Ok(recv)
}

