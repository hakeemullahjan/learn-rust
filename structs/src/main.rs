fn main() {
    let name = String::from("Crow");
    let bird = Bird { name, attack: 23 };
    bird.print_name();
    bird.print_attack();
    println!("canFLy {}", bird.can_fly());
    println!("isAnimal {}", bird.is_animal());

}

#[derive(Debug)]
struct Bird {
    name: String,
    attack: u8,
}

impl Bird {
    fn print_name(&self) {
        println!("name->{}", self.name,);
        println!("{:?}", self);
    }
    fn print_attack(&self) {
        println!("attack{}", self.attack);
    }
}

trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool {
        true
    }
}

impl Animal for Bird {
    fn can_fly(&self) -> bool {
        true
    }
    fn is_animal(&self) -> bool {
        false
    }
}
