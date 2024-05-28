enum Ticket {
    Backstage(i32,String), 
    VIP(i32,String),
    Standard( i32),
}

fn main() {
    
    let sales = [
        Ticket::Backstage(30,"Ali".to_owned()),
        Ticket::VIP(90,"Hamza".to_owned()),
        Ticket::Standard(10),
    ];

    for sale in sales {
        match sale {
            Ticket::Backstage( price, name) => println!("name : {} , price {}", name, price),
            Ticket::VIP( price, name) => println!("name : {} , price {}", name, price),
            Ticket::Standard( price) => println!("price {}", price),
            other => ()
        }
    }

}





