
 use std::{io::{self, BufReader, BufRead}, result, vec, fs::File};


fn main() {
    

        // USING MATCH TO HANDLE ERRORS
        let file: Result<File, io::Error> = File::open("a_file.txt");
        match file {
            // MATCH TO CHECK IF IT IS A FILE OR ERROR
            Ok(file) => {
                // IF THE FILE EXISTED THEN IT WOULD PRINT OUT THE LINES
                let reader: BufReader<File> = BufReader::new(file);
                for line in reader.lines(){
                    println!("{}", line.unwrap());
                }
            },
            // IF THE FILE DIDNT EXIST THEN IT WILL PROVIDE AN ERROR MESSAGE
            Err(error) => {
                // NESTED MATCH TO HANDLE DIFFERENT ERRORS
                match error.kind() {
                    std::io::ErrorKind::NotFound => {
                        // CAN USE println! INSTEAD OF panic IF IT IS SET UP RIGHT
                        println!("File not found: {}", error)
                        //panic!("File not found: {}", error)        
                    } 
                    _ => {
                        // CAN USE println! INSTEAD OF panic IF IT IS SET UP RIGHT
                        println!("Error opening file: {}", error)
                        //panic!("Error opening file: {}", error)
                    }
                }   
        
            }
        };
    }


