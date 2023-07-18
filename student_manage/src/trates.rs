// 教师教学
pub trait Teach {
    type Item;
    fn teach() -> Self::Item;
}

pub trait Persion {
    fn self_introduction(&self);
}
