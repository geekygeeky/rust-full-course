// Shadowing is a feature that allows you to assign a value to a variable
// Shadowing helps to reassign variables of different types.

fn main(){

    let x = 5;

    let x = x + 1;


    {
        let x = x * 2;
        println!("The value of x in the inner scope is {}", x);
    }

    println!("The value of x is {x}");

}