use structs::{School, Student, Class};

mod trates;
mod structs;
mod manage;
fn main() {
    let mut school = School::init(Some("Liv".to_owned()));
    for i in 1..5 as u32{
        let mut name = String::from("Student");
        name.push_str(&i.to_string());
        let student = Student {
            id: i,
            name,
            email: String::from("email@example.com"),
            phone: 1212122121.to_string(),
            stu_no: i.to_string(),
            class_no: None,
            grade: 1.to_string(),
        };
        let _ = school.add_student(student);
    }
    for i in 1..3 as u32{
        let mut name = String::from("Class");
        name.push_str(&i.to_string());
        let class = Class {
            id: i,
            name,
            students: vec![],
        };
        school.add_class(class);
    }

    let _ = school.addstu_toclass(1, 2);
    let _ = school.addstu_toclass(2, 2);
    let _ = school.addstu_toclass(3, 3);
    let _ = school.addstu_toclass(4, 3);
    let stu3 = school.get_student(3).unwrap();
    println!("查询stu3的信息: {:#?}\n===============================", stu3);
    school.remove_student(3).unwrap();
    println!("操作了删除stu3操作后再查询该学号的信息: {:?}\n============================", school.get_student(3));

    println!("原来的stu2学号的信息: {:#?}", school.get_student(2));
    let stu2 = Student {
        id: 2,
        name: "2号学生".to_owned(),
        email: "test@gmail.com".to_owned(),
        phone: "123456".to_owned(),
        stu_no: 2.to_string(),
        class_no: Some(2),
        grade: 2.to_string(),
    };
    school.update_student(stu2).unwrap();
    println!("操作了更新操作后的stu2信息: {:#?}\n==================================", school.get_student(2));

}

// 学校、班级、教师、学生、

#[cfg(test)]
mod test {
    #[test]
    fn add_student() {

    }
}
