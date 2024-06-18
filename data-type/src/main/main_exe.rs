
#![allow(unused)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use std::{ffi::c_uchar, sync::atomic};

pub fn main() -> Result<(), usize> {
    println!("Fundamental Data type.");

    //REM: PREMITIVE DATA TYPE
    let boolean: bool;              //REM: 1 byte (8 bits) 
                                    //REM: but only the least significant bit set to 1 representing TRUE, otherwise set all to 0 representing FALSE
                                    //REM: 0 to 1
    
    let char32Bit: char;            //REM: In Rust char have 4 bytes (32 bits)
                                    //REM: Unicode Scalar Value
                                    //REM: 0 to (0x10FFFF or 1_114_111)

    let byte: i8;                   //REM: 1 byte (8 bits)            
                                    //REM: -128 to +127

    let unsignedByte: u8;           //REM: unsigned 1 byte (8 bits)   
                                    //REM: 0 to 255

    let short: i16;                 //REM: 2 bytes (16 bits)        
                                    //REM: -32_768 to 32_767
    
    let unsignedShort: u16;         //REM: unsigned 2 bytes (16 bits)
                                    //REM: 0 to 65_535

    let integer32Bit: i32;          //REM: integer (4 bytes)
                                    //REM: -2_147_483_648 to 2_147_483_647
                                    //REM: BILLION (10^9)

    let unsignedInteger32Bit: u32;  //REM: unsigned integer (4 bytes)
                                    //REM: 0 to 4_294_967_295
                                    //REM: BILLION (10^9)

    let integer64Bit: i64;          //REM: long (8 bytes)
                                    //REM: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
                                    //REM: QUINTILLION (10^18)

    let unsignedInteger64Bit: u64;  //REM: unsigned long (8 bytes)
                                    //REM: 0 to 18,446,744,073,709,551,615
                                    //REM: QUINTILLION (10^18)

    let float32Bit: f32;            //REM: float (6 to 7 precision) (4 bytes)
                                    //REM: ~1.2E-38 to ~3.4E+38

    let float64Bit: f64;            //REM: double ( 15 to 16 precision) (8 bytes)
                                    //REM: ~2.3E-308 to ~1.7E+308

    let integer128Bit: i128;        //REM: signed 16 bytes integer
                                    //REM: -170,141,183,460,469,231,731,687,303,715,884,105,727 to 170,141,183,460,469,231,731,687,303,715,884,105,727
                                    //REM: UNDECILLION (10^36)

    let unsignedInteger128Bit: u128;//REM: unsigned 16 bytes integer
                                    //REM: 0 to 340,282,366,920,938,463,463,374,607,431,768,211,455
                                    //REM: UNDECILLION (10^36)
    
    let signedSize: isize;          //REM: signed integer (architect dependent)
                                    //REM: eithter i32 or i64 bits.

    let unsignedSize: usize;        //REM: unsigned integer (architect dependent)
                                    //REM: eithter u32 or u64 bits.

    //REM: WRAPPER DATA TYPE
    let charSequence: String;       //REM: 24 bytes (192 bits)
                                    //REM: 0 to 6_277_101_735_386_680_763_835_789_423_207_666_416_102_355_444_464_034_512_895
                                    //REM: OCTODECILLION (10^57)

    //REM: There are much more wrapper data type or wrapper classes/struct...

    return Ok(());
}
