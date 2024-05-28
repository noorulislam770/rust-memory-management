struct LineItem {

    name : String,
    count : i32,
}

fn print_name (name : &str) { 
    
    println!("name {}",name);
  
}


fn main () {

    let recipt = vec![
        LineItem {
            name : "cereal".to_owned(),
            count : 1,
        },
        LineItem {
            name : String::from ("fruit"),
            count : 3,
        },
    ];

    for r in recipt {
        print_name(&r.name);
    }


}
