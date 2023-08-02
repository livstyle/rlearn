
fn modify_by_address(address: usize) {
    // `address` is a memory address, there is an u32 at that address. try modify
    // the u32's value to 0xAABBCCDD
    unsafe {
        let ad = address as *mut u32;
        *ad = 0xAABBCCDD;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t:u32 = 0x12345678;
        let address = &mut t as *mut u32 as usize;
        modify_by_address(address); 
        assert!(t == 0xAABBCCDD);
    }

    #[test]
    fn mainwww() {
        let mut x = 42;
    
        // 获取变量 x 的内存地址，并创建一个指向该地址的原始指针
        let address = &mut x as *mut i32;
    
        // 在 unsafe 代码块中，通过指针解引用来修改变量的值
        unsafe {
            *address = 99;
        }
    
        println!("Modified x: {}", x); // 输出 "Modified x: 99"
    }
}