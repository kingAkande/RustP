
pub mod test_trait;
 pub mod mystruct;
pub mod closure;
pub mod martch;
pub mod optionenum;
 fn main() {
    
    // cond_stat();
    // if_stat();
    // conditional();

    // closure::closure_test();
    // closure::try_anony();

    // martch::matching();
    //martch::match_str()
    // let p = optionenum::option_test();
    // println!("{}" , p.unwrap());

//    let h = optionenum::option_string();
//     println!("{}", h.unwrap());

    // let w = optionenum::choose_chr();

    // if choose_chr().is_some(){
    //     println!("you have selected a character");
    // }else {
    //     println!("no selected character");
    // }
    // println!("{}" , w.unwrap().to_string() );

    // let d  = mystruct::structin();

    // let m = mystruct::vehicle_discri(); 
    // test_trait::create_poerson();
        // test_trait::creat_p();
        test_trait::teacher();
}



#[allow(dead_code)]
fn cond_stat(){


let  age_to_drive = 16u8;

println!("Enter the person's age");
let myinput= &mut String::from("");
std::io::stdin().read_line(myinput).unwrap();

let age = myinput.replace("\n", "").parse::<u8>().unwrap();

if age >= age_to_drive {
    println!("you are old enough");
} else {
    println!("not eligible");
}

}

#[allow(dead_code)]

fn if_stat(){
    let drive_age = 18u8;
    println!("what is ur age");

    let  userinput = &mut String::from("");

    std::io::stdin().read_line( userinput).unwrap();
    let age = userinput.replace("\n", "").parse::<u8>().unwrap();
    
    if age>= drive_age {
        println!("eligible")
    }
    

}

#[allow(dead_code)]
fn conditional(){
    let elig_age = 18u8;
    println!("whats ur age");

    let myinput: &mut String = &mut String::from("");
    std::io::stdin().read_line(myinput).unwrap();

    let age = myinput.replace("\n", "").parse::<u8>().unwrap();

        if age >= elig_age {
            println!("good");
        }else {
            println!("bad");
        }

}