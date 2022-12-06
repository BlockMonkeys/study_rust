struct Student {
    name : String,
    age : u8,
    grade : u8
}
#[derive(Debug)]
struct Rectangle {
    length : u32,
    width : u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

fn main() {
    let mut minseo : Student = addStudent("Minseo", 31, 4);
    let mut blockmonkey = Student {
        name : String::from("Blockmonkey"),
        ..minseo
    };
    println!("{}", minseo.name);
    println!("{}", blockmonkey.name);

    let myRect : Rectangle = Rectangle {length: 50, width: 30};
    let result = myRect.area();
    println!("{}", result);
    println!("my React : {:#?}", myRect);
}

fn addStudent(_name : &str, _age : u8, _grade : u8) -> Student {
    Student {
        name : String::from(_name),
        age : _age,
        grade : _grade
    }
}

