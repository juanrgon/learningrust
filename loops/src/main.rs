fn main() {
    infinite_loop();
    value_from_loop();
    while_loop();
    for_loop();
    reverse_for_loop();
}

fn infinite_loop() {
    loop {
        break;
    }
}

fn value_from_loop() {
    let mut counter = 0;

    let _result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
}

fn while_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn reverse_for_loop() {
    for number in (1..11).rev() {
        println!("{}!", number);
    }
    println!("HAPPY NEW YEAR!!!");
}
