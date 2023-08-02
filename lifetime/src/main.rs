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
impl<'a> School<'a> {
    fn new() -> School<'a> {
        let students = Vec::<Student>::new();
        let classes = HashMap::<u32, Class>::new();
        Self {
            students, classes,
            name: "aaa".to_owned(),
        }       
    }
    pub fn add_class(&mut self, class: Class<'a>) {
        let id = class.id;
        self.classes.insert(id, class);
    }

    pub fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }

    pub fn stu2class(&mut self, class_id: u32, student_id: u32) {
        let student: &Student = self.students.iter().find(|s| s.id == student_id).unwrap();
        // let stu = self.classes.get_mut(&class_id).unwrap().students.as_mut().unwrap();
        // stu.push(student);
        self.classes.get_mut(&class_id).unwrap().students.clone().unwrap().push(student);
        // let class = self.classes.get_mut(&class_id).unwrap();
        // let stuvec = class.students.as_mut().unwrap();
        // stuvec.clone().push(student);
    }

    pub fn class_students(&self, class_id: u32) -> &Option<Vec<&'a Student>>{
        &self.classes.get(&class_id).unwrap().students
    }
}

fn main() {
    let student = Student {
        id: 12,
        name: "NO12".to_owned(),
    };
    let student2 = Student {
        id: 13,
        name: "NO12".to_owned(),
    };
    let mut school = School::new();
    let class = Class {
        id: 1,
        name: "Class1".to_owned(),
        students: None,
    };
    school.add_student(student);
    school.add_student(student2);

    // sch.add_class(class);
    school.stu2class(1, 12);
    school.stu2class(1, 13);
}
