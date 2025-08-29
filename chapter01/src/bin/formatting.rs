//Format Traits
/*
nothing ⇒ Display
? ⇒ Debug
x? ⇒ Debug with lower-case hexadecimal integers
X? ⇒ Debug with upper-case hexadecimal integers
o ⇒ Octal
x ⇒ LowerHex
X ⇒ UpperHex
p ⇒ Pointer
b ⇒ Binary
e ⇒ LowerExp
E ⇒ UpperExp
*/

use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { "N" } else { "S" };
        let lon_c = if self.lon >= 0.0 { "E" } else { "W" };

        write!(f, "{}: {:.3}° {} {:.3}° {}", 
                self.name, 
                self.lat.abs(),
                lat_c,
                self.lon.abs(),
                lon_c
            )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        //i want final out put to be smth like -> RGB (128, 255, 90) 0x80FF5A
        //since i already get the RGB values, i just need caclulate the seconde part!
        
        // let rgb = (self.red as u32 * 65536) + (self.green as u32 * 256) + self.blue as u32;
        // write!(f, "RGB ({}, {}, {}) 0x{:0>6X}", 
        //         self.red,
        //         self.green,
        //         self.blue,
        //         rgb
        //         )
        //OR
        let rgb = format!("{:0>2X}{:0>2X}{:0>2X}", self.red, self.green, self.blue);
        write!(f, "RGB ({}, {}, {}) 0x{}", 
                self.red,
                self.green,
                self.blue,
                rgb
                )
    }
}

fn main(){
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ] {
        println!("{}", city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{:?}", color);
        println!("{}", color)
    }
}