use std::fs::File;
use std::io::Read;
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_input<P>(filename: P) -> String
where
    P: AsRef<Path>,
{
    let mut file = File::open(filename).unwrap();
    let mut output = String::new();
    file.read_to_string(&mut output).unwrap();
    output.trim().to_owned()
}
