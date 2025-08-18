// If Else [If expression] [Else expression]

fn main(){

    let age: u16 = 18;

    if age >= 18{
        println!("You can drive a car");
    }else{
        println!("You can't drive a car!");
    }

    // Multiple condtions with else if:
    let num = 6;

    if num % 4 == 0 {
        println!("Number is divisible by 4");
    } else if num % 3 == 0 {
        println!("Number is divisible by 3");
    } else if num % 2 == 0 {
        println!("Number is divisible by 2");
    } else{
        println!("Number is not divisible by 2, 3 or 4");
    }

    // Using if in a let statement
    let condition = true;
    let result = if condition { 5 } else { 6 };
    println!("Result is {result}");

}