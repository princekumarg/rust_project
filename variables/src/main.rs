fn main() {
    let mut age=24;
    println!("you number is {age}");
    println!("Hello, world!");
    age=25;
    println!("your new number is {age}");
    //we can't use mut with const we can only used with let.
    const Z:u32=32;
    println!("your number is {Z}");
    //****Shadowing****//in this first variable is shadowed with second variables
    /*let app=10;
    let app=20;
    println!("your shadowing number is {app}");*/
    //***main concept of shadowing***//
    let x=32;
    let x=x+1;
    {
        let x=x*2;
        println!("your inner number is:{x}");
    }
    println!("your real number is:{x}");
}
