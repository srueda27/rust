fn add(num_one: i32, num_two: i32) -> i32 {
    // the return is optional, Rust will return the last line if the last line is an expression
    return num_one + num_two;
}

fn main() {
    // let mut my_name = "Santiago";

    // my_name = "Rueda";

    let foo = add(5, 20);

    // You know is a macro instead of a function 'cause the !
    println!("{} {0}", foo);

    // another way to use placeholders on print, for complex values
    println!("{:?}", foo);

    let mut total = add(10, 4);
    let mut free_shipping = false;

    if total > 50 {
        println!("You qualify");
        free_shipping = true;
    } else if total > 20 {
        println!("Add more")
    } else {
        println!("You don't")
    }

    // match free_shipping {
    //     true => total = total + 0,
    //     false => total = total + 5,
    // }

    total = match free_shipping {
        true => total + 0,
        false => total + 5,
    };

    match total {
        1 => println!("1"),
        2 => println!("2"),
        // wildcart for every other option
        _ => println!("no match"),
    };

    println!("{:?}", total);

    let items: [i32; 5] = [1, 4, 5, 6, 7];

    println!("{:?}", items);
    println!("{:#?}", items);

    let vector_items = vec![1, 4, 5, 6, 7];
    let mut vector_items_2:Vec<i32> = Vec::new();

    println!("{:?}", vector_items);
    println!("{:?}", vector_items_2);
}
