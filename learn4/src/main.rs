/**
    使用枚举包裹三个不同的类型，并放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。
    定义三个不同的类型，使用Trait Object，将其放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。
    同时，说明其上两种不同实现方法的区别。
*/

/// 使用trait Object 和 enum 的区别
/// enum 拓展性不太好，trait Object的拓展比较容易，只要实现了trait就可以了
mod learn4_1 {

    #[derive(Debug)]
    pub struct A {}
    impl A {
        pub fn test(&self) {
            println!("I am: A");
        }
    }

    #[derive(Debug)]
    pub struct B {}
    impl B {
        pub fn test(&self) {
            println!("I am: B");
        }
    }

    #[derive(Debug)]
    pub struct C {}
    impl C {
        pub fn test(&self) {
            println!("I am: C");
        }
    }

    #[derive(Debug)]
    pub enum EnumVec {
        V1(A),
        V2(B),
        V3(C),
    }

    pub fn iter_enum() {
        let enumvec = vec![EnumVec::V1(A{}), EnumVec::V2(B{}), EnumVec::V3(C{})];
        enumvec.iter().for_each(|item| {
            println!("枚举值为：{:?}", item);
            match item {
                EnumVec::V1(a) => { a.test(); },
                EnumVec::V2(b) => { b.test(); },
                EnumVec::V3(c) => { c.test(); }
            }
        });
    }

    pub trait TC {
        fn test(&self){}
    }

    #[derive(Debug)]
    pub struct CA {}

    impl TC for CA {
        fn test(&self) {
            println!("I am CA");
        }
    }

    #[derive(Debug)]
    pub struct CB;
    impl TC for CB {
        fn test(&self) {
            println!("I am CB");
        }
    }

    #[derive(Debug)]
    pub struct CC();
    impl TC for CC {
        fn test(&self) {
            println!("I am CC");
        }
    }

    pub fn test_2() {
        let mut to_vec = Vec::<Box<dyn TC>>::new();
        to_vec.push(Box::new(CA{}));
        to_vec.push(Box::new(CB));
        to_vec.push(Box::new(CC()));

        to_vec.iter().for_each(|item| {
            item.test();
        });

    }


}


// 5. 为你自己定义的一个类型或多个类型实现加法运算（用符号 +），并构思使用Trait Object实现类型方法的调用。
mod learn5 {
    use std::ops::Add;

    pub trait StringNumber {}

    #[derive(Debug, Clone)]
    pub struct A {
        x: String
    }

    #[derive(Debug, Clone)]
    pub struct B {
        y: u32
    }

    impl StringNumber for A {}
    impl StringNumber for B {}

    impl Add<B> for A {
        type Output = A;

        fn add(self, rhs: B) -> Self::Output {
            let mut x_ = self.x;
            let str_y = rhs.y.to_string();
            x_.push_str(&str_y); 
            A {
                x: x_
            }
        }
    }

    impl Add<A> for B {
        type Output = A;

        fn add(self, rhs: A) -> Self::Output {
            let str_y = self.y.to_string();
            let mut x_ = rhs.x;
            x_.push_str(&str_y); 
            A {
                x: x_
            }
        }
    }

    pub fn t_fn() {
        // let vec: Vec<Box<dyn StringNumber>> = vec![Box::new(A{x: "abc".to_owned()}), Box::new(B{y: 76})];
        let a = A{x: "abc".to_owned()};
        let b = B{y: 34};
        let c = a.clone() + b.clone();
        let d = b + a;
        println!("c:   {:?}", c);
        println!("d:   {:?}", d);
    }

}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_1() {
        learn4_1::iter_enum();
    }

    #[test]
    fn test_2() {
        learn4_1::test_2();
    }

    #[test]
    fn test3() {
        learn5::t_fn();
    }
}
