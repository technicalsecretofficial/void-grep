
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
  
    value: String,

    file: String,
    
}

fn main() -> io::Result<()>   {
    let args = Args::parse();


    let file = File::open(args.file)?;

    let reader = BufReader::new(file);

    let mut word = String::new();

    for line in reader.lines() {
        let line = line?;
        
        word = line.clone();
        
        if word.contains(&args.value) {
            println!("Found: {}", args.value);
            break;
        }
            
     }

        if !word.contains(&args.value)  {
            println!("Not Found");
        }



    Ok(())
 
}
