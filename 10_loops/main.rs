
fn main(){
    // Loop keyword usage
    loop_fn();

    // loop labels
    loop_labels();

     // while loops
    while_loop();

    // for loops
    for_loop();
}

// Loop keyword
fn loop_fn(){
    let mut counter = 8;

    let result = loop{

        counter += 1;

        if counter % 7 == 0 {
            break counter
        }

    };

    println!("First value divisible by 7 encountered: {result} \n")
}

fn loop_labels(){
    let mut count = 0;
    'counting_up: loop {
        println! ("count: {count}");
        let mut remaining = 10;
        loop{
            println! ("remaining: {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
}

fn while_loop(){
    let cities = ["Paris", "Ikeja", "Dubai", "Madrid"];
    let mut count = 0;

    while count < 2 {
        println!("City {} is {}", count + 1, cities[count]);
        count = count + 1;
    }

}

fn for_loop() {
    let animals = ["Dog", "Cat", "Mongoose", "Python"];

    for animal in animals {
        println!("{}", animal);
    }

    for number in 1..10 {
        println!("{number}");
    }
}