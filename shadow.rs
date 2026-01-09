fn main(){

    //______________immuatable concept ____________________
    //as we know all variable in rust are by default immuatble
    let a:i32 = 3;
    //if we later we are using the value we are making a copy of it and using that
    let b = a; // here we are making a copy of it
    // and making copy is only possible with int float primitive data type like int float char boolean
    //important

    //if we are using any variable data type we are making a copy of it or passing a immuatable reference to it
    println!("{}", b);
    println!("{}", a);
    //or we can use any time that copy or immuatable reference
    println!("{}", a); // no problem
    let c: &i32 = &a; // its making a immuatable reference
    println!("{}", c);
    //and we also know we cant make direct copy from string
    let name:str = "rajesh"; // this is not possible
    //we can also dont this
    let name2:String = String::from("rajesh");
    let name_r:&str = name2; // we also cant use like this becuase in the case of string its not possible to making a copy
    //we can do this given below
    let name_rr:&String = &name2;
    let name_rrr:&str = & name2;
    println!("{} {}",name_rr, name_rrr); //here we are using and reading name_rr and name_rrr by makin refrence not copying

    //__________multable concept______
    //as we know rust all variable are by defualt immutable
    //to do mutable
    let mut x:i32 = 34;
    // but if some variable mutalbe we can reference mutalbe to mutate it
    let y:&mut i32 = &mut x;
    // but if once mutalbe refrence is created we cant use it refrence to use it
    let z = x + 2; // this is not possible 
    println!("{}", x); // this is not also possible
    //but we can modify it by using star opertaor
    *y = 3;
    // but we still cant use it
    // to do right thing we need to do in the block scope

    {
        let y:&mut i32 = &mut x;
        *y = 5;
    }
    let r = x +1; // but now we can use here 
    println!("{}", x)// also used here;
}