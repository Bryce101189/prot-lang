use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    // Get file path
    if env::args().len() <= 1 {
        eprintln!("error: no input file(s) provided");
        eprintln!(" -h or --help for usage guide and command list");
        return;
    }

    let args: Vec<String> = env::args().collect();
    let script_path = args.get(1).unwrap();

    // Check if path is valid
    if !script_path.ends_with(".prot") {
        eprintln!("error: unrecognized file extension(s)");
        eprintln!(" please make sure all input files use the '.prot' file extension");
        return;
    }

    // Open file
    let mut script: File = {
        let file: File;

        match File::open(script_path) {
            Ok(f) => {
                file = f;
            }

            Err(..) => {
                eprintln!("error: could not open file '{}'", script_path);
                return;
            }
        }

        file
    };

    // Read file contents into string
    let mut contents = String::new();

    if let Err(..) = script.read_to_string(&mut contents) {
        eprintln!("error: failed to read from file '{}'", script_path);
        return;
    }
}
