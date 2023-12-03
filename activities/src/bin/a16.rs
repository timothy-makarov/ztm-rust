// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker_number: Option<i32>
}

fn print_student_info(student: Student) {
    match student.locker_number {
        Some(n) => println!("{:?} assigned to a locker number {:?}" , student.name, n),
        None => println!("{:?} has no locker assigned to.", student.name)
    }
}

fn main() {
    let s1 = Student {
        name: "Peter".to_owned(),
        locker_number: Some(42)
    };
    print_student_info(s1);

    let l2 = Student {
        name: "Kyle".to_owned(),
        locker_number: None
    };
    print_student_info(l2);
}
