
// Integers basic ops (Addition, Subtraction & Multiplication)
pub const IADD: i8 = 1;
pub const ISUB: i8 = 2;
pub const IMULT: i8 = 3;

// Comparison (Less Than & Less Than)
pub const ILET: i8 = 4;
pub const IEQ : i8= 5;

// Branching (Branch; Branch if true & if false)
pub const BR : i8= 6;
pub const BRT: i8 = 7;
pub const BRF: i8 = 8;

pub const ICONST: i8 = 9; // Push constant integer
pub const LOAD: i8 = 10; // Load from local context
pub const GLOAD: i8 = 11; // Load from global memory
pub const STORE: i8 = 12; // Store in local context
pub const GSTORE: i8 = 13; // Store in global memory

pub const PRINT: i8 = 14; // Print stack top
pub const POP: i8 = 15;// Throw away top of stack

// Return with/without value
pub const CALL: i8 = 16;
pub const RET : i8= 17;
pub const HALT : i8= 18;


pub struct Instruction {
    pub name: String,
    // Instruction name
    pub agrs: i8, // Number of arguments
}

impl Instruction {
    pub fn str(&self) -> String {
        return format!("[name:{} args: {}]", self.name, self.agrs);
    }
}

pub fn instruction_new(_name: &str, _args: i8) -> Instruction {
    return Instruction { name: _name.to_string(), agrs: _args };
}

// Instructions mapping
pub fn inst_mapping(_opcode: i8) -> Instruction {
    match _opcode {
        IADD => instruction_new("iadd", 0),
        ISUB => instruction_new("isub", 0),
        IMULT => instruction_new("imult", 0),

        ILET => instruction_new("ilet", 0),
        IEQ => instruction_new("ieq", 0),
        BR => instruction_new("ieq", 1),
        BRT => instruction_new("brt", 1),
        BRF => instruction_new("brf", 1),

        ICONST => instruction_new("iconst", 1),
        LOAD => instruction_new("load", 1),
        GLOAD => instruction_new("gload", 1),
        STORE => instruction_new("store", 1),
        GSTORE => instruction_new("gstore", 1),

        PRINT => instruction_new("print", 0),
        POP => instruction_new("pop", 0),
        CALL => instruction_new("call", 1),

        RET => instruction_new("ret", 0),
        HALT => instruction_new("hat", 0),

        _ => instruction_new("nil", 0 )
    }
}

#[test]
fn test_instruction_byte_mapping() {

}

#[test]
fn test_instruction_mapping() {
    let iadd = inst_mapping(IADD);
    assert_eq!(iadd.name,   String::from("iadd"));

    let gstore = inst_mapping(GSTORE);
    assert_eq!(gstore.name,   String::from("gstore"));
}