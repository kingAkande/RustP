
#[allow(dead_code)]
pub  fn matching(){

let my_age = 40;

match my_age {

    0 =>   println!("new babby"),
      
    11..=22 => println!("youngeyy "),
        
    23..=40 => println!("wow"),

    _ => ()
}

}

#[allow(dead_code)]
pub fn match_str(){

println!("{}", "whats ur favourite?");
 let myinput  = &mut String::new();
 std::io::stdin().read_line(myinput).unwrap();
 let manufac = myinput.trim();

 match manufac {

     "toyota" => println!("the best"),
      "hyonda" => println!("also goood"),
     _ => println!("still tying"),
 }


}