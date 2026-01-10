fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
    println!("pow function {}", pow(3,2));
    println!("{}", cube(3));
}

fn add_one(x:i32)->i32{
    return x +1;
}

fn cube(x:i32)->i32{
    pow(x,3)
}

fn pow(x:i32, y:i32)->i32{
    if y == 0 {
        1
    }else{
        x * pow(x, y-1)
    }
}


fn plus_one(x: i32) -> i32 {
    x + 1 // if without ; means it returning
}