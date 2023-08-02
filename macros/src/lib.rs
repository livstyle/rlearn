

// 这是声明宏
#[macro_export]
macro_rules! tvec {
    // 这里的匹配的是代码规则，正则表达式，底层的匹配规则是怎样的？？？
    // 常用的匹配表达式有
    // block | expr | ident | item | lifetime | literal
    // | meta | pat | pat_param | path | stmt | tt | ty | vis
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

macro_rules! ambiguity {
    ($($i:ident)* $j:ident) => { };
}



// 模块相关的宏
pub mod inner {
    #[macro_export]
    macro_rules! call_foo {
        () => { $crate::inner::foo() };
    }

    pub fn foo() {}
}


// 过程宏一般是 属性宏、Derive macros
// 对于属性宏的应用则在之前的一个项目中有用到 https://github.com/livstyle/livorm.git


