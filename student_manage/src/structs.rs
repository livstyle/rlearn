use std::{collections::HashMap, sync::Mutex};
use std::io::{Error, ErrorKind};

use crate::trates::Persion;

#[derive(Debug)]
pub struct School {
    pub name: String,
    pub classes: HashMap<u32, Class>,
    pub students: Vec<Student>,
    pub teachers: Vec<Teacher>,
}

impl School {
    pub fn init(name: Option<String>) -> School{
        let students = Vec::<Student>::new();
        let classes = HashMap::<u32, Class>::new();
        let teachers = Vec::<Teacher>::new();
        School {
            name: if let Some(s) = name { s } else { "LIVSTYLE".to_owned() },
            classes,
            students,
            teachers,
        }
    }
    pub fn get_classes(&self) -> &HashMap<u32, Class> {
        &self.classes
    }
    fn get_mut_classes(&mut self) -> &mut HashMap<u32, Class> {
        &mut self.classes
    }
    fn get_mut_class(&mut self, class_id: u32) -> Option<&mut Class> {
        self.classes.get_mut(&class_id)
    }
    pub fn get_class(&self, class_id: u32) -> Option<&Class> {
        self.classes.get(&class_id)
    }
    pub fn add_class(&mut self, class: Class) {
        let id = class.id;
        self.classes.insert(id, class);
    }
    pub fn update_classname(&mut self, class_id: u32, name: String) -> Result<(), std::io::Error> {
        let mut class = self.get_mut_class(class_id);
        if let Some(c) = class {
            c.name = name;
        } else {
            let error = Error::from(ErrorKind::NotFound);
            return Err(error);
        }
        Ok(())
    }
    pub fn remove_class(&mut self, class_id: u32) -> Result<(), std::io::Error> {
        // 判断是否有传入的班级
        let mut class = self.get_class(class_id);
        if let Some(c) = class {
            // 判断班级中是否有学生
            if c.students.is_empty() {
                self.get_mut_classes().remove(&class_id);
            } else {
                return Err(Error::from(ErrorKind::AlreadyExists));
            }
        } else {
            let error = Error::from(ErrorKind::NotFound);
            return Err(error);
        }
        Ok(())
    }
    pub fn get_students(&self) -> Result<&Vec<Student>, std::io::Error> {
        Ok(&self.students)
    }
    pub fn get_student(&self, student_id: u32) -> Result<&Student, std::io::Error> {
        let stu = self.students.iter().find(|student| {
            student.id == student_id
        });
        if let Some(student) = stu {
            Ok(student)
        } else {
            Err(Error::from(ErrorKind::NotFound))
        }
    }
    pub fn add_student(&mut self, student: Student) -> Result<(), std::io::Error>{
        // 判断是否已经存在
        let student_id = student.id;
        let stu = self.students.iter().find(|student| {
            student.id == student_id
        });
        if let None = stu {
            self.students.push(student);
            Ok(())
        } else {
            Err(Error::from(ErrorKind::AlreadyExists))
        }
    }
    pub fn update_student(&mut self, student: Student) -> Result<(), std::io::Error> {
        let student_id = student.id;
        let stu = self.students.iter_mut().find(|student| {
            student.id == student_id
        });
        if let Some(student_) = stu {
            *student_ = student;
            Ok(())
        } else {
            Err(Error::from(ErrorKind::NotFound))
        }
    }
    pub fn remove_student(&mut self, student_id: u32) -> Result<(), std::io::Error> {
        let stu = self.students.iter().position(|student| {
            student.id == student_id
        });
        if let Some(student_index) = stu {
            // 判断是否加入了班级如果有加入班级则删除班级里面的这个学号
            let st = self.students.get(student_index).unwrap();
            if let Some(class_no) = st.class_no {
                let stu_index = self.classes.get(&class_no).unwrap().students.iter().position(|id| *id == student_id).unwrap();
                self.classes.get_mut(&class_no).unwrap().students.remove(stu_index);
            }
            // 删除学生信息
            self.students.remove(student_index);
            Ok(())
        } else {
            Err(Error::from(ErrorKind::NotFound))
        }
    }
    pub fn addstu_toclass(&mut self, student_id: u32, class_id: u32) -> Result<(), std::io::Error> {
        // 判断班级和学生是否同时存在
        if let Some(class) = self.classes.get_mut(&class_id) {
            if let Some(stuedent) = self.students.iter_mut().find(|stu| stu.id == class_id) {
                class.students.push(student_id);
                stuedent.class_no = Some(class_id);
                Ok(())
            } else {
                Err(Error::from(ErrorKind::NotFound))
            }
        } else {
            Err(Error::from(ErrorKind::NotFound))
        }
    }

}

#[derive(Debug)]
pub struct Student {
    pub id:         u32,
    pub name:       String,
    pub email:      String,
    pub phone:      String,
    pub stu_no:     String,
    pub class_no:   Option<u32>,
    pub grade:      String,
}

impl Persion for Student {
    fn self_introduction(&self) {
        println!("I am {}; I'm a student", self.name);
    }
}

impl Student {
    fn learn(&self) {
        println!("I am {}, I'm learning;", self.name);
    }
}

#[derive(Debug)]
pub struct Teacher {
    pub id: u32,
    pub name: String,
}

impl Persion for Teacher {
    fn self_introduction(&self) {
        println!("I am {}, I'm a teacher;", self.name);
    }
}

#[derive(Debug)]
pub struct Subject {
    id: u32,
    name: String,
}

#[derive(Debug)]
pub struct Class {
    pub id: u32,
    pub name: String,
    pub students: Vec<u32>,
}

// 课程、班级、教师
#[derive(Debug)]
pub struct ClassTeach {
    id: u32,
    class_id: u32,
    teacher_id: u32,
    subject_id: u32,
}

// 课程安排
#[derive(Debug)]
pub struct ClassCourse {
    pub id: u32,
    pub weekday: u32,
    pub session: u32, // 节次 1:上午；2:下午；
    pub class_id: u32, // 班级
    pub subject_id: u32, //课程ID
}

// 班级课程表
pub type CourseTable = HashMap<u32, Option<Mutex<Vec<ClassCourse>>>>;