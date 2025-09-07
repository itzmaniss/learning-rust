fn main () {
    //integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    //integer subtraction
    println!("1 - 2 = {}\n", 1i32 - 2);

    //scientific notation
    println!("1e4 is {}, -2.5e-3 is {}\n", 1e4, -2.5e-3);

    //Short Circuiting Boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}\n", !true);

    //Bitwise operations
    println!("0011 AND 0101 is {:0>4b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:0>4b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:0>4b}", 0b0011u32 ^ 0b0101);
    //these 2 are for shifting bits, 
    //shift the bits on the left by direction of arrow for number on the right bits
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}\n", 0x080u32 >> 2);

    //can use underscore to seperate long numbers for better readability
    //the type is optional
    println!("one million is written as {}", 1_000_000u32);


}