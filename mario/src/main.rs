struct Person {
    name: String,
    transformation: Transformation
}

impl Person {
    fn new(name:String) -> Person {
        Person {
            name: name,
            transformation: Transformation::Mario,
        }
    }

    fn display(&self) {
        //println!("je suis {}: {}", self.name, self.transformation);
        println!("je suis {}: {}", self.name, self.transformation.display());
        //self.transformation.display();
    }

    fn eat(&mut self, food: Food) {
        // match (self.transformation, food){
        match (&self.transformation, food) {
            (Transformation::Mario, Food::Mushroom) => self.transformation = Transformation::SuperMario,
            (Transformation::Mario, Food::Fire) => self.transformation = Transformation::FireMario,
            (Transformation::Mario, Food::Feather) => self.transformation = Transformation::CapeMario,

            (Transformation::FireMario, Food::Feather) => self.transformation = Transformation::CapeMario,
            (Transformation::FireMario, Food::Mushroom) => self.transformation = Transformation::CapeMario,
            (Transformation::FireMario, Food::Fire) => self.transformation = Transformation::FireMario,

            (Transformation::CapeMario, Food::Fire) => self.transformation = Transformation::FireMario,
            (Transformation::CapeMario, Food::Mushroom) => self.transformation = Transformation::CapeMario,
            (Transformation::CapeMario, Food::Feather) => self.transformation = Transformation::CapeMario,

            (Transformation::SuperMario, Food::Fire) => self.transformation = Transformation::FireMario,
            (Transformation::SuperMario, Food::Mushroom) => self.transformation = Transformation::SuperMario,
            (Transformation::SuperMario, Food::Feather) => self.transformation = Transformation::CapeMario,

            //(_, _) => todo!(),
            //_ => todo!(),
        }
    }

}

enum Transformation {
    Mario,
    SuperMario,
    FireMario,
    CapeMario,
}

impl Transformation {
    fn display(&self) -> String {
        match self {
            //Transformation::Mario => println!("Mariooooo"),
            Transformation::Mario => String::from("Mariooooo"),
            Transformation::SuperMario => String::from("SM!"),
            Transformation::FireMario => String::from("Fire!"),
            Transformation::CapeMario => String::from("Fly!"),
        }
        /*};
        // String::from("Fake") // implicit return
        // return String::from("Fake2"); // explicit return
        */
    }
}

enum Food {
    Mushroom,
    Fire,
    Feather,
}

// use std::fmt;
// impl fmt::Display for Transformation {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Transformation::Mario => write!(f, "Mariooooo"),
//             Transformation::SuperMario => write!(f, "Super Mario!"),
//             Transformation::FireMario => write!(f, "Fire Mario!"),
//             Transformation::CapeMario => write!(f, "Cape Mario!"),
//         }
//     }
// }

fn main() {
    let mut person = Person::new(String::from("Mario"));
    person.display();
    person.eat(Food::Fire);
    person.display();

    person.eat(Food::Mushroom);
    person.display();

    person.eat(Food::Feather);
    person.display();
}
    /*let mut a = 4;
    a = a + 3; 
    let mon_string = String::from("Hello devfest Paris!");
    //display(mon_string);
    //display(mon_string);
    display(&mon_string);
    display(&mon_string);
    println!("Hello, world! : {}", a);
}

fn display(a: &String) {
    println!("{}", a)
} */