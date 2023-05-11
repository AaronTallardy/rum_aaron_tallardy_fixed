use crate::execute::Um;
use std::io;
use std::io::prelude::*;

pub fn cmov(umi: &mut Um, a: u32, b: u32, c: u32){
    //eprintln!("CMOV: program counter{} , inst_counter{}", umi.program_counter, umi.inst_counter);
    //println!("cmov");
    if umi.regs[c as usize] != 0{
        umi.regs[a as usize] = umi.regs[b as usize];
    }
    umi.program_counter += 1;
    umi.inst_counter += 1;
}

pub fn seg_load(umi: &mut Um, a: u32, b: u32, c: u32){
    //eprintln!("seg_load: program counter{} , inst_counter{}", umi.program_counter, umi.inst_counter);
    //println!("seg_load");
    //eprintln!("loading segment {} offsett at {} into {}", umi.regs[b as usize], umi.regs[c as usize], umi.regs[a as usize]);
    /* let b_val = umi.regs[b as usize] as usize;
    let c_val = umi.regs[c as usize] as usize;
    let mem_segs = &mut umi.mem_segs;
    let a_reg = a as usize;

    umi.regs[a_reg] = mem_segs[b_val][c_val]; */
    
    umi.regs[a as usize] = umi.mem_segs[umi.regs[b as usize] as usize][umi.regs[c as usize] as usize];
    umi.program_counter += 1;
    umi.inst_counter += 1;
}

pub fn seg_store(umi: &mut Um, a:u32, b: u32, c: u32){
    //eprintln!("Seg_store: program counter{} , inst_counter{}", umi.program_counter, umi.inst_counter);
    //println!("seg store");
    //eprintln!("storing {} into segment {} offset at {}", umi.regs[c as usize], umi.regs[b as usize], umi.regs[c as usize]);
    umi.mem_segs[umi.regs[a as usize] as usize][umi.regs[b as usize] as usize] = umi.regs[c as usize];
    
    umi.program_counter += 1;
    umi.inst_counter += 1;
}

pub fn add(umi: &mut Um, a: u32, b:u32, c:u32){
    //println!("add");
    //eprintln!("add: program counter{} , inst_counter{}", umi.program_counter, umi.inst_counter);
    umi.regs[a as usize] = umi.regs[b as usize].wrapping_add(umi.regs[c as usize]);
    umi.program_counter += 1;
    umi.inst_counter += 1;
}

pub fn mult(umi: &mut Um, a: u32, b:u32, c:u32){
    //eprintln!("mult: program counter{} , inst_counter{}", umi.program_counter, umi.inst_counter);
    //println!("mult");
    umi.regs[a as usize] = umi.regs[b as usize].wrapping_mul(umi.regs[c as usize]);
    //umi.regs[a as usize] = (umi.regs[b as usize] * umi.regs[c as usize]) % 2_u32.pow(32);
    umi.program_counter += 1;
    umi.inst_counter += 1;
}

pub fn div(umi: &mut Um, a: u32, b: u32, c: u32){
    //eprintln!("div: program counter{} , inst_counter{}", umi.program_counter, umi.inst_counter);
    //println!("div");
    umi.regs[a as usize] = umi.regs[b as usize].wrapping_div(umi.regs[c as usize]);
    umi.program_counter += 1;
    umi.inst_counter += 1;
}

pub fn bit_nand(umi: &mut Um, a: u32, b: u32, c: u32){
    //eprintln!("bit_nand: program counter{} , inst_counter{}", umi.program_counter, umi.inst_counter);
    //println!("bit_nand");
    umi.regs[a as usize] = !(umi.regs[b as usize] & umi.regs[c as usize]);
    umi.program_counter += 1;
    umi.inst_counter += 1;
}

pub fn map_seg(umi: &mut Um,b: u32, c: u32){
    //eprintln!("map_seg: program counter{} , inst_counter{}", umi.program_counter, umi.inst_counter);
    //resize
    //eprintln!("mapping at segment of capacity {} at register {}", umi.regs[c as usize], umi.regs[b as usize]);
    //println!("map_seg");
    // will pop the last element of reg_tracker off
    match umi.regs_tracker.pop() {
        Some(index) => {
            //create a new segment at the index that was popped off and stores that index in register b
            umi.mem_segs[index as usize] = vec![0; umi.regs[c as usize] as usize];
            umi.regs[b as usize] = index;
        }
        None => {
            //will push the new memory segment to the end of the mem_segs vector and store the index of that in register b
            umi.mem_segs.push(vec![0; umi.regs[c as usize] as usize]);
            umi.regs[b as usize] = umi.mem_segs.len() as u32 - 1;
        }
    }
    umi.program_counter += 1;
    umi.inst_counter += 1;
    //creates a new segment of the # of words in register c and has it full of zeros
    //umi.mem_segs.push(vec![0; umi.regs[c as usize] as usize]);

    //umi.mem_segs[umi.regs_tracker.last as usize].push(vec![0; umi.regs[c as usize] as usize]);
    //sets register b to the index of the new memory segment
    //umi.regs[b as usize] = umi.mem_segs.len() as u32 - 1;
}

pub fn unmap_seg(umi: &mut Um, c: u32){
    //eprintln!("unmap: program counter{} , inst_counter{}", umi.program_counter, umi.inst_counter);
    //println!("unmap");
    //eprintln!("unmapping a register at register{}", umi.regs[c as usize]);
    let test = umi.regs[c as usize];
    umi.mem_segs[test as usize].clear();
    umi.regs_tracker.push(test as u32);
    umi.program_counter += 1;
    umi.inst_counter += 1;
}

pub fn output(umi: &mut Um, c: u32){
    //eprintln!("output: program counter{} , inst_counter{}", umi.program_counter, umi.inst_counter);
    //println!("output");
    io::stdout().write(&[umi.regs[c as usize] as u8]).unwrap();
    umi.program_counter += 1;
    umi.inst_counter += 1;
}

pub fn input(umi: &mut Um, c: u32){
    //eprintln!("Input: program counter{} , inst_counter{}", umi.program_counter, umi.inst_counter);
    //println!("input");
    let mut buffer = [0; 1];
    match io::stdin().read_exact(&mut buffer) {

        Ok(()) => {
            umi.regs[c as usize] = buffer[0] as u32
        },

        Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => {
            umi.regs[c as usize] = u32::MAX
        }, // u32::MAX gives a max u32 of all 1's

        Err(_) => panic!(),
    }
    umi.program_counter += 1;
    umi.inst_counter += 1;
}

pub fn load_program(umi: &mut Um, b: u32, c: u32){
    //eprintln!("Load_program: program counter{} , inst_counter{}", umi.program_counter, umi.inst_counter);
    // println!("load_program");
    if umi.regs[b as usize] != 0{
        let dup_seg = umi.mem_segs[umi.regs[b as usize] as usize].clone();
        umi.mem_segs[0] = dup_seg;
    }
    //points to the memory segment at c
    umi.program_counter = umi.regs[c as usize];
    umi.inst_counter += 1;
}

pub fn load_value(umi: &mut Um, rl: u32, vl: u32){
    //eprintln!("load_value: program counter{} , inst_counter{}", umi.program_counter, umi.inst_counter);
    //println!("load_val");
    //println!("{rl:?} {vl:?}");
    umi.regs[rl as usize] = vl;
    umi.program_counter += 1;
    umi.inst_counter += 1;
}