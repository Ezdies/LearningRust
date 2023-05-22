#[derive(Debug, PartialEq, Default, Clone)]
struct Student{
    name: String,
    surname: String,
    birth_year: u32,
    subject: String,
}

impl Student {
    fn new_student(name: String, surname: String, birth_year: u32, subject: String) -> Student{
        Student { name, surname, birth_year, subject }
    }
    fn get_name(&self) -> String{
        let name = self.name.clone();
        name
    }
    fn get_surname(&self) -> String{
        let surname = self.surname.clone();
        surname
    }
    fn get_birth_year(&self) -> u32{
        let birth_year = self.birth_year.clone();
        birth_year
    }
    fn get_subject(&self) -> String{
        let subject = self.subject.clone();
        subject
    }
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
    fn set_surname(&mut self, surname: String){
        self.surname = surname;
    }
    fn set_birth_year(&mut self, birth_year: u32){
        self.birth_year = birth_year;
    }
    fn set_subject(&mut self, subject: String) {
        self.subject = subject;
    }
    fn is_older_than(&self, student: &Student)-> bool{
        self.birth_year < student.birth_year
    }
    fn is_on_the_same_subject(&self, student: &Student) -> bool{
        self.subject == student.subject
    }

}

fn main() {
    let mut st = Student::new_student("Maks".to_string(), "Dudziak".to_string(), 1988, "Computer Science".to_string());
    println!("{:?}", st);
    let st_name = st.get_name();
    st.set_name("Aleks".to_string());
    println!("{:?}", st);
    println!("{}", st_name);
    let changed_name = st.get_name();
    println!("{}", changed_name);
    let st2 = Student::new_student("Jan".to_string(), "Kowalski".to_string(), 1998, "Law".to_string());
    let is_older = st.is_older_than(&st2);
    println!("{}", is_older);
    let is_on_the_same_subject = st.is_on_the_same_subject(&st2);
    println!("{}", is_on_the_same_subject);
}
