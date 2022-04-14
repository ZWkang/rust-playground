fn if_and_else(n: i32) {
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }
}

fn loop_test() -> u32 {
    let mut count = 0u32;
    println!("let's start loop print !!!");

    let result = loop {
        count += 1;
        
        if count == 3 {
            println!("three");

            continue;
        }

        println!("count: {}",  count);


        if count == 100 {
            println!("ok, stop print");
            break count * 2;
        }
    };
    result
}

fn while_test() {
    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // 计数器值加 1
        n += 1;
    }
}

fn for_loop_test() {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

fn for_loop_iterator() {
    // let names = vec!["Bob", "Frank", "Ferris"];

    // for name in names.iter() {
    //     match name {
    //         &"Ferris" => println!("There is a rustacean among us!"),
    //         _ => println!("Hello {}", name),
    //     }
    // }

    // let mut names = vec!["Bob", "Frank", "Ferris"];

    // for name in names.iter_mut() {
    //     *name = match name {
    //         &mut "Ferris" => "There is a rustacean among us!",
    //         _ => "Hello",
    //     }
    // }
    // println!("names: {:?}", names);

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}


fn main() {
    // println!("Hello, world!");

    // if_and_else(22);

    // let num = loop_test();
    // println!("{}", num);


    // while_test();
    // for_loop_test();
    for_loop_iterator();
}
