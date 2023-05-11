use std::process::exit;

use crate::functions;

// use rum::functions;

pub struct Field {
    width: u32,
    lsb: u32,
    }
    
static RA: Field = Field {width: 3, lsb: 6};
static RB: Field = Field {width: 3, lsb: 3};
static RC: Field = Field {width: 3, lsb: 0};
static RL: Field = Field {width: 3, lsb: 25};
static VL: Field = Field {width: 25, lsb: 0};
static OP: Field = Field {width: 4, lsb: 28};

pub struct Um {
    pub regs: Vec<u32>,
    pub mem_segs: Vec<Vec<u32>>,
    pub program_counter: u32,
    pub regs_tracker: Vec<u32>,
    pub inst_counter: u64
}

impl Um{
    pub fn new() -> Self{
        Um{
            regs: vec!{0;8},
            mem_segs: vec!{},
            program_counter : 0,
            regs_tracker: vec!{},
            inst_counter: 0
        }
    }
    //pub fn mask(&self, bits: u32) -> u32 { (1 << bits) - 1 }

    /* pub fn get(&self, field: &Field, inst: u32) -> u32 {
        (inst >> field.lsb) & self.mask(field.width)
    } */
    const MASKS: [u32; 33] = [
        0x0000_0000, 0x0000_0001, 0x0000_0003, 0x0000_0007, 0x0000_000f, 0x0000_001f, 0x0000_003f,
        0x0000_007f, 0x0000_00ff, 0x0000_01ff, 0x0000_03ff, 0x0000_07ff, 0x0000_0fff, 0x0000_1fff,
        0x0000_3fff, 0x0000_7fff, 0x0000_ffff, 0x0001_ffff, 0x0003_ffff, 0x0007_ffff, 0x000f_ffff,
        0x001f_ffff, 0x003f_ffff, 0x007f_ffff, 0x00ff_ffff, 0x01ff_ffff, 0x03ff_ffff, 0x07ff_ffff,
        0x0fff_ffff, 0x1fff_ffff, 0x3fff_ffff, 0x7fff_ffff, 0xffff_ffff
    ];

    pub fn get(&self, field: &Field, inst: u32) -> u32 {
        // Use lookup table to retrieve precomputed mask
        let mask = Um::MASKS[field.width as usize];
        (inst >> field.lsb) & mask
    }

    pub fn execute (&mut self, inst: u32){
        enum Opcode{
            CMov,
            SegLoad,
            SegStore,
            Add,
            Mult,
            Div,
            BitNand,
            Halt,
            MapSeg,
            UnmapSeg,
            Output,
            Input,
            LoadProgram,
            LoadValue
        }

        match self.get(&OP, inst){
            o if o == Opcode::CMov as u32 => {
                functions::cmov(self, self.get(&RA, inst), self.get(&RB, inst), self.get(&RC, inst))
            },
            o if o == Opcode::SegLoad as u32 => {
                functions::seg_load(self, self.get(&RA, inst), self.get(&RB, inst), self.get(&RC, inst))
            },
            o if o == Opcode::SegStore as u32 => {
                functions::seg_store(self, self.get(&RA, inst), self.get(&RB, inst), self.get(&RC, inst))
            },
            o if o == Opcode::Add as u32 => {
                functions::add(self, self.get(&RA, inst), self.get(&RB, inst), self.get(&RC, inst))
            },
            o if o == Opcode::Mult as u32 => {
                functions::mult(self, self.get(&RA, inst), self.get(&RB, inst), self.get(&RC, inst))
            },
            o if o == Opcode::Div as u32 => {
                functions::div(self, self.get(&RA, inst), self.get(&RB, inst), self.get(&RC, inst))
            },
            o if o == Opcode::BitNand as u32 => {
                functions::bit_nand(self, self.get(&RA, inst), self.get(&RB, inst), self.get(&RC, inst))
            },
            o if o == Opcode::Halt as u32 => {
                exit(0);
            },
            o if o == Opcode::MapSeg as u32 => {
                functions::map_seg(self, self.get(&RB, inst), self.get(&RC, inst))
            },
            o if o == Opcode::UnmapSeg as u32 => {
                functions::unmap_seg(self, self.get(&RC, inst))
            },
            o if o == Opcode::Output as u32 => {
                functions::output(self, self.get(&RC, inst))
            },
            o if o == Opcode::Input as u32 => {
                functions::input(self, self.get(&RC, inst))
            },
            o if o == Opcode::LoadProgram as u32 => {
                functions::load_program(self, self.get(&RB, inst), self.get(&RC, inst))
            },
            o if o == Opcode::LoadValue as u32 => {
                //format!("See semantics for “other instruction”.")
                functions::load_value(self, self.get(&RL, inst), self.get(&VL, inst))
            },
            _ =>{
                panic!("ahhhhhhhhhhhhhhhhh");
            }
        }
    }
}