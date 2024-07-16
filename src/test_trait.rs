// struct Person<Type , Type2:Animal + Dangerous > where Type: Animal + NotDangerous {               //plus sign is used to add multiple
//     first_name:String,
//     pet : Type,
//     Pet2nd: Type2
// }


// trait Animal {
//     fn sound(&self)->();
// }

// trait NotDangerous {}
// trait Dangerous {}  

// struct  Dog{}
// impl Dangerous for Dog{}
// impl Animal for Dog{
//     fn sound(&self)->(){
//         println!("Dog bark");
//     }
// }

// #[allow(dead_code)]
//  struct Cat{}
//  impl NotDangerous for Cat{}
//  impl Animal for Cat{
//     fn sound(&self)->(){
//         println!("Cat meow");
//     }
//  }

//  #[allow(dead_code)]
//  struct Ram{}
//  impl NotDangerous for Dog{}
//  impl  Animal for Ram{
//     fn sound(&self)->(){
//         println!("Ram moo")
//     }
//  }


//  pub fn create_poerson(){

//     let pet1= Dog{};
//     let pet2=Cat{};
//     let pet3=Ram{};

//     let p1 = Person{
//         first_name:"Dele".to_string() ,
//         pet:pet2,
//         Pet2nd:pet1,
//     };

//    p1.pet.sound();
//    p1.Pet2nd.sound();
//  }







// struct Person<pet_type ,pet_type2:Animal + Wild> where pet_type:Animal + Domestic{
//     first_name: String,
//     pet: pet_type,
//     pet2:pet_type2,
// }


// trait Animal {
//  fn makesound(&self);
// }

// trait Domestic {}
// trait Wild {}

// #[derive(Debug)]
// struct Dog{}
// impl Animal for Dog{
//     fn makesound(&self){
//         println!("DOg bark");
//     }
// }
// impl Wild for Dog{}

// #[derive(Debug)]
// struct Cat{}
// impl Animal for Cat{
//     fn makesound(&self){
//         println!("Cat meow");
//     }
// }
// impl Domestic for Cat{}


// pub fn creat_p(){

//     let pet1=Dog{};
//     let pet2=Cat{};


//     let p1 = Person{
//         first_name:"lanre".to_string(),
//         pet:pet2,
//         pet2:pet1
//     };

//     println!("{} {:?} {:?}", p1.first_name, p1.pet, p1.pet2);
//     p1.pet.makesound();
//     p1.pet2.makesound();
// }

use std::string;


struct School<subjet_type:Subject>{
    class_teacher: String,
    subject_teaching:subjet_type,
}


trait Subject {
    fn difficulty_level(&self);
}

struct Math{}
impl Subject for Math{
    fn difficulty_level(&self) {
        println!("Math is fun")
    }
}

struct English{}
impl Subject for English{
    fn difficulty_level(&self) {
        println!("English is easy")
    }
}

struct Biology{}
impl Subject for Biology{
    fn difficulty_level(&self) {
        println!("Biology is crazy")
    }
}


pub fn teacher(){
    let sub1 = Math{};
    let sub2=English{};
    let sub3=Biology{};

       let c1 = School{
            class_teacher:"Mrs Gbenro".to_string(),
            subject_teaching:sub2,
        };
        c1.subject_teaching.difficulty_level();

}