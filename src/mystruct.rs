use std::cell::Cell;



#[derive(Debug)]
enum Vehicle_color{
    Gold,
    Silver,
    Wine,
    White,
}

#[derive(Debug)]
pub struct Vehicle{
    name: String,
    model: u8,
    color: Vehicle_color,

}

pub fn vehicle_discri() {

    let mut  v1 = Vehicle{

        name:"Bughati".to_string(),
        model: 120,
        color:Vehicle_color::Gold,
    };

    v1.changeColor(Vehicle_color::White);

    println!("{:?} ", v1 );
}



impl Vehicle {
    
    fn changeColor(&mut self , new_Color:Vehicle_color) {
        
        self.color = new_Color;
    }
}




#[allow(dead_code)]

pub struct Person<'p>{
    name:Cell<& 'p str> ,
    number: u8,
}


pub fn structin()-> Person<'static> {

   
    let mut p1 = Person{

        name :Cell::new( "sulaimon"),
        number: 15,
    };

    p1.name.set("taye");   

    p1.test_impl(30);

    println!("{} {}", p1.name.get() , p1.number);
    return p1;
}


impl  Person<'_>{

    fn test_impl(&mut self, n:u8 ) {
        
        self.number = n;
    
    }

}
