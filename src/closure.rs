

struct Person{

    firstname: String,
    lastname: String,
    class: u8,
}



#[allow(dead_code)]
pub  fn closure_test(){

    // let add = || println!("trust");    //closure without argumen

    // add();

    let add = |x:u8| println!("print {}", x);   //closure with a single parameter
    add(67);

    let sub = |x:u8, y:u8| {   //closure with multiple lines of code

     let result=  x-y;
        
        print!("insert {} and {}, then get {}", x, y,result );

    };

    sub(10,8);


    let mut p1 = Person{ firstname:"kasim".to_string(), lastname:"ola".to_string() , class: 3};

    let mut changing = || p1.class = 5u8;

    changing();

    // let mut change_name = || p1.firstname = "kareem".to_string();
    // change_name();

    let mut change_name = |j:&str| p1.firstname = j.to_string();
    change_name("kareem");

   
    println!("{} {} {} ", p1.firstname , p1.lastname, p1.class)
}

#[allow(dead_code)]
pub fn try_anony(){

struct Hnd1 {

    name:String,
    matric: u8,
}

let mut s1 = Hnd1{ name:"Wale".to_string() , matric: 78, };

let mut to_change = |y:&str| s1.name = y.to_string();
to_change("jamal");
to_change("sulaimon");

println!("{} {} ", s1.name , s1.matric); 


}