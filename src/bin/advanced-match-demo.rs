enum Discount {
    Percent(i32),
    Flat(i32)
}

struct Ticket  {
    event : String,
    price : i32,
}



fn main() {

    let n = 3;
    match n  {
        3 => println!("three"),
        _ => println!("number: {:?} ",n),
    }

    let flat = Discount::Flat(4);
    let percent = Discount::Percent(50);

    match flat {
        Discount::Flat(4) => println!("flat 4 discounted"),
        Discount::Flat(amount) => println!("flat discount of {}." ,amount),
        other => ()
    }
    
    let concert = Ticket {
        event : "concert".to_owned(),
        price : 50,
    };

    match concert {
        Ticket{price: 50, event} => println!("price 50 and event is {}",event),
        Ticket{price, ..} => println!("price == {:?} ",price),
    }

}
