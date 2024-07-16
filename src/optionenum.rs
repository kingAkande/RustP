
#[allow(dead_code)]

pub fn option_test() -> Option<u8>{

    let mut y: Option<u8> = None;
    
    y = Some(20);

    return y

    
}



#[allow(dead_code)]
pub fn option_string()-> Option<String>{

        let mut z: Option<String> = None;
        
        z = Some("you are coming".to_string());
        return z;

}

#[allow(dead_code)]
pub fn choose_chr()-> Option<Character> {
        let mut q: Option< Character> = None;
        q = Some(Character::Broch);
        return  q;
}

pub enum Character{

Archer,
Broch,
Rust,


}


impl Character {

   pub fn to_string(&self)-> String{

            match self {
                Character::Archer => "Archer",
                Character::Broch => "Broch",
                Character::Rust => "Rust",
            }.to_string()

    }

}