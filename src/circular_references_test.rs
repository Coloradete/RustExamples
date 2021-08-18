/* FIRST ATTEMPT
struct Student<'a>{
    name: String,
    courses: Vec<&'a Course<'a>>
}

impl<'a> Student<'a>{

    fn new(name: &str) -> Student<'a>{
        Student{
            name: name.into(),
            courses: Vec::new()
        }
    }
}

struct Course<'a>{
    name: String,
    students: Vec<&'a Student<'a>>
}

impl<'a> Course<'a>{

    fn new(name: &str) -> Course<'a>{
        Course{
            name: name.into(),
            students: Vec::new()
        }
    }

    fn add_student(&'a mut self, student: &'a mut Student<'a>){
        student.courses.push(self);// Esto no funciona porque digo que self es una referencia muteable y aquí lo estoy pasando como algo inmutable ERROR
        self.students.push(student); //Sin embargo acá si que lo necesito como muteable ya que le estoy añadiendo cosas, AUXILIO
        //Esto se soluciona con RefCells, pero tambien hay problemas al dropear
    }
}
*/

/*Second Attempt con Rc y RefCell

use std::cell::RefCell;
use std::rc::Rc;

struct Student{
    name: String,
    courses: Vec<Rc<RefCell<Course>>>
}

impl Student{

    fn new(name: &str) -> Student{
        Student{
            name: name.into(),
            courses: Vec::new()
        }
    }
}

struct Course{
    name: String,
    students: Vec<Rc<RefCell<Student>>>
}

impl Course {
    fn new(name: &str) -> Course {
        Course {
            name: name.into(),
            students: Vec::new()
        }
    }

    fn add_student(course: Rc<RefCell<Course>>, student: Rc<RefCell<Student>>)
    {
        student.borrow_mut().courses.push(course.clone());
        course.borrow_mut().students.push(student.clone());
    }
}
 */

/* Third attempt, cambia la estrategia a "Database normalization", en este caso tendremos
Students que no toman referencias a cursos
Courses que no toman referencias a estudiantes
y un tercer tipo de estructura -> Enrollment que toma un par curso, estudiante y contiene toda la informacion sobre la asociacion de estos pares
 */

struct Student{
    name: String
}

impl Student{
    fn courses(&self, platform: &Platform) -> Vec<String>{

        platform.enrollments.iter()
            .filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()
    }
}

struct Course{
    name: String
}

struct Enrollment<'a> //Aca tendremos que introducir lifetime para asegurarnos que los estudiantes viven lo mismo que los cursos y al revés
{
    student: &'a Student,
    course: &'a Course
}

impl<'a> Enrollment<'a>{
    fn new(student: &'a Student, course: &'a Course) -> Enrollment<'a>
    {
        Enrollment { student, course }
    }
}

struct Platform<'a>{
    enrollments: Vec<Enrollment<'a>>
}

impl<'a> Platform<'a>{
    fn new() -> Platform<'a>{
        Platform{enrollments: Vec::new()}
    }

    fn enroll(&mut self, student: &'a Student, course: &'a Course){
        self.enrollments.push(
            Enrollment::new(student,course)
        );
    }
}

pub fn call_circular_references_test(){
    /* First Attempt
    let mut jhon = Student::new("Jhon");

    let mut course = Course::new("Rust Course");

    course.add_student(&mut jhon); //etnocnes cuando dropeemos las variables vamos a tener un problema, si dropeamos jhon primero entonces course tendra
                                            // una referencia a algo que ya no existe y lo mismo al reves, esto se puede resolver con Rc
     */

    /* Second Attempt This one works but it gets a little messy with all the cloning and borrowing, also lost Rust checking functionallities
    let mut jhon = Rc::new(
        RefCell::new(
            Student::new("Jhon")
        )
    );

    let mut jane = Rc::new(
        RefCell::new(
            Student::new("Jane")
        )
    );
    let mut course = Course::new("Rust Course");

    let magic_course = Rc::new(RefCell::new(course));

    Course::add_student(magic_course.clone(),jhon);
    Course::add_student(magic_course.clone(),jane);
    */

    let john = Student{ name:"John".into() };

    let rust = Course{ name:"Intro to Rust".into() };
    let spanish = Course{ name:"Spanish".into() };

    let mut students_platform = Platform::new();

    students_platform.enroll(&john,&rust);
    students_platform.enroll(&john,&spanish);

    for c in john.courses(&students_platform){
        println!("Jhon is taking {}", c);
    }
}