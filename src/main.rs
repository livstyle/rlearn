mod az;
mod sa;


/* 添加合适的生命周期标注，让下面的代码工作 */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


// `print_refs` 有两个引用参数，它们的生命周期 `'a` 和 `'b` 至少得跟函数活得一样久
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

/* 让下面的代码工作 */
fn failed_borrow<'a>() {
    let _x = 12;

    // ERROR: `_x` 活得不够久does not live long enough
    let y: &i32 = &_x;

    // 在函数内使用 `'a` 将会报错，原因是 `&_x` 的生命周期显然比 `'a` 要小
    // 你不能将一个小的生命周期强转成大的
}


fn main() {
    let (four, nine) = (4, 9);
    
    print_refs(&four, &nine);
    // 这里，four 和 nice 的生命周期必须要比函数 print_refs 长
    
    failed_borrow();
    // `failed_borrow`  没有传入任何引用去限制生命周期 `'a`，因此，此时的 `'a` 生命周期是没有任何限制的，它默认是 `'static`
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_1() {
        /* 增加合适的生命周期标准，让代码工作 */
        // `i32` 的引用必须比 `Borrowed` 活得更久
        #[derive(Debug)]
        struct Borrowed<'a>(&'a i32);

        // 类似的，下面两个引用也必须比结构体 `NamedBorrowed` 活得更久
        #[derive(Debug)]
        struct NamedBorrowed<'a, 'b> {
            x: &'a i32,
            y: &'b i32,
        }

        #[derive(Debug)]
        enum Either<'a> {
            Num(i32),
            Ref(&'a i32),
        }

        fn dd() {
            let x = 18;
            let y = 15;

            let single = Borrowed(&x);
            let double = NamedBorrowed { x: &x, y: &y };
            let reference = Either::Ref(&x);
            let number    = Either::Num(y);

            println!("x is borrowed in {:?}", single);
            println!("x and y are borrowed in {:?}", double);
            println!("x is borrowed in {:?}", reference);
            println!("y is *not* borrowed in {:?}", number);
        }

        dd()

    }


    fn test_2() {
        /* 让代码工作 */
        #[derive(Debug)]
        struct NoCopyType {}
        
        #[derive(Debug)]
        struct Example<'a, 'b> {
            a: &'a u32,
            b: &'b NoCopyType,
        }
        
        fn main() {
            let var_a = 35;
            let example: Example;
            let var_b = NoCopyType {};
        
            {
                // 使用引用的引用，确保引用的有效性不会超过其引用的数据
                example = Example { a: &var_a, b: &var_b };
            }
        
            println!("(Success!) {:?}", example);
        }
        

    }



    fn test_3() {
        #[derive(Debug)]
        struct NoCopyType {}

        #[derive(Debug)]
        #[allow(dead_code)]
        struct Example<'a, 'b> {
            a: &'a u32,
            b: &'b NoCopyType
        }

        /* 修复函数的签名 */
        fn fix_me<'a>(foo: &'a Example) -> &'a NoCopyType{ 
            foo.b 
        }

        fn main() {
            let no_copy = NoCopyType {};
            let example = Example { a: &1, b: &no_copy };
            fix_me(&example);
            println!("Success!")
        }

    }

    struct B {
        id: u32,
    }

    struct A<'a> {
        vecs: Vec<&'a B>,
    }

    impl<'a> A<'a> {
        fn new() -> Self{
            Self {
                vecs: Vec::new(),
            }
        }
        
        fn push(&mut self, v: &'a B) {
            self.vecs.push(v);
        }
    }

    fn test_4() {
        let mut a = A::new();
        let b1 = B{
            id: 32
        };
        let b2 = B{
            id: 38
        };
        a.push(&b1);
        a.push(&b2);
    }



}
