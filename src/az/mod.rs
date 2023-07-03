pub mod az1 {
    pub fn pt() {
        for i in 'a'..'z' {
            print!("{}", i);
        }
        for j in 'A'..'Z' {
            print!("{}", j);
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn az1_test() {
        az1::pt()
    }

    #[test]
    fn az2_test() {
        az1::az2::pt();
    }
}
