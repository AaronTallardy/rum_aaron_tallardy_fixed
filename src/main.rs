use std::env;
pub mod execute;
pub mod functions;

fn main() {
        let input = env::args().nth(1);
        let instructions = load(input.as_deref());
        //println!("{} instructions", instructions.len());
        let mut um = execute::Um::new();
        um.mem_segs.push(instructions.clone());
        while true {
            
            um.execute(um.mem_segs[0][um.program_counter as usize]);
            //print!("total intstructions: {}", um.program_counter);
        }
        
        
}
pub fn load(input: Option<&str>) -> Vec<u32> {
    let mut buf_reader: Box<dyn std::io::BufRead> = match input {
        None => Box::new(std::io::BufReader::new(std::io::stdin())),
        Some(filename) => Box::new(std::io::BufReader::new(
            std::fs::File::open(filename).unwrap(),
        )),
    };

    let mut instructions = Vec::new();
    loop {
        let mut chunk = [0; 4];
        match buf_reader.read_exact(&mut chunk) {
            Ok(()) => {
                instructions.push(u32::from_be_bytes(chunk));
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    break;
                } else {
                    panic!("Error reading input file: {}", e);
                }
            }
        }
    }

    instructions
}

/* pub fn load(input: Option<&str>) -> Vec<u32> {
    let mut raw_reader: Box<dyn std::io::BufRead> = match input {
        None => Box::new(std::io::BufReader::new(std::io::stdin())),
        Some(filename) => Box::new(std::io::BufReader::new(
        std::fs::File::open(filename).unwrap(),)),
    };
    let mut buf = Vec::<u8>::new();
    
    raw_reader.read_to_end(&mut buf).unwrap();
    
    let instructions: Vec<u32> = buf
    .chunks_exact(4)
    .map(|x| u32::from_be_bytes(x.try_into().unwrap()))
    .collect();
    instructions
    
} */
