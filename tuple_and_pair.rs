fn main(){
    let base_ = (1,2,true, 4.5); //can be any type of data 
    //can be return multiple type of data
    let (a,b) = out_funct_tuple();
    println!("{}, {}", a, b);
    //or we can use either the indexing on tuple
    println!("{} {} ", base_.1, base_.3);
}
fn out_funct_tuple()->(i32, bool){
    (3, true)
}