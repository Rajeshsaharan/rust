fn main(){
    // basic expression of rust 
    // by default all rust variable unmutable but can be used/read later
    let a = 2;  
    let b = a + 1;
    let c:i32 = b + 1; // rust with type
    let _:i32 = c + 1; //if we dont use
    // block are also eval to expression to result 
    //block also helps in scope 
    let result = {
        let a = 3;
        let b = 4;
        a + b // by default result will be store a + b because it doesnt have comma ending
    };
    println!("{}", result);
    let result2 = {
        let a = 5;
        let b = 7;
        a + b ;
    };
    println!("{}", result2); // will give error because result2 dont have a + b;
}