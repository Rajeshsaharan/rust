fn main(){
    //boolean are simple in rust
    let is_valid:bool = true;
    let is_correct:bool =  false;

    //and if and else epxrssion are also used in
    if is_valid {
        println!("{}", "yes");
    }else{

    }
    //if else we can also use block scope to 

    let x:i32 = if is_valid{
        println!("{}", "this statment is valid");
        let b = 89;
        b //means return b if is_valid is true
    }else{
        let c = 90;
        c // if its false it return c and store to x
    }
}