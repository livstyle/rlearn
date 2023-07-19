use std::collections::HashMap;

struct Student {
    id: u32,
    name: String,
}

struct Class<'a> {
    id: u32,
    name: String,
    students: Option<Vec<&'a Student>>,
}

struct School<'a> {
    name: String,
    students: Vec<Student>,
    classes: HashMap<u32, Class<'a>>
}

// impl后面也要紧跟着生命周期标注
impl<'a> School<'static> {
    fn new() -> School<'static> {
        let students = Vec::<Student>::new();
        let classes = HashMap::<u32, Class>::new();
        Self {
            students, classes,
            name: "aaa".to_owned(),
        }       
    }
    pub fn add_class(&'a mut self, class: Class<'static>) {
        let id = class.id;
        self.classes.insert(id, class);
    }

    pub fn add_student(&'a mut self, student: Student) {
        self.students.push(student);
    }

    pub fn stu2class(&'static mut self, class_id: u32, student_id: u32) {
        let student = self.students.iter().find(|s| s.id == student_id).unwrap();
        let stu = self.classes.get_mut(&class_id).unwrap().students.as_mut().unwrap();
        stu.push(student);
    }

    pub fn class_students(&self, class_id: u32) -> &Option<Vec<&'static Student>>{
        &self.classes.get(&class_id).unwrap().students
    }
}

fn main() {
    let student = Student {
        id: 12,
        name: "NO12".to_owned(),
    };
    let mut school: School<'static> = School::new();
    school.add_student(student);
    let class = Class {
        id: 1,
        name: "Class1".to_owned(),
        students: None,
    };
    school.add_class(class);
    // school.stu2class(1, 12);
}
