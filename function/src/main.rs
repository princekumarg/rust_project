fn main() {
    println!("Hello, world!");
    my_function();
    my_datatypes(5,true,'A');
    println!("{}",sq(4));
}
fn my_function(){
    println!("My Function");
}
fn my_datatypes(roll:i32,student:bool,sec:char){
    println!("RollNo {roll} is a Student{student} in section {sec}")
}
fn sq(x:i32)->i32{
    x*x
}
