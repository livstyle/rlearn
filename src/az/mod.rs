pub mod az1 {
    pub fn pt() {
        let a_lcode: u32 = 'a'.into();
        for i in a_lcode..(a_lcode + 26) {
            print!("{}", char::from(i as u8));
        }
        let a_ucode: u32 = 'A'.into();
        for j in a_ucode..(a_ucode + 26) {
            print!("{}", char::from(j as u8));
        }
    }
    pub mod az2 {
        pub fn pt() {
            let a_ucode: u32 = 'A'.into();
            let a_lcode: u32 = 'a'.into();
            for i in a_ucode..(a_ucode + 26) {
                print!("{}", char::from(i as u8));
            }
            for i in a_lcode..(a_lcode + 26) {
                print!("{}", char::from(i as u8));
            }
        }
    }
}

pub fn run() {
    println!("a->Z");
    az1::pt();
    println!("\n A->z");
    az1::az2::pt();
    println!("\n end");
}
