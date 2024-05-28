struct Customer { 
    age : Option<i32>,
    email : String, 
}

struct GroceryItem {
    name :String, 
    quantity : i32
}

fn find_quantity(name : &str, groceries: &Vec<GroceryItem> ) -> Option<i32> {
    
    for item in groceries {
        if item.name == name {
            return Some(item.quantity)
        }
    }
    None

}


fn main () {

    let mark = Customer {
        age : Some(22), email : "mark@emample.com".to_owned(),
    };


    let becky = Customer {
        age: None, email: "becky@example.com".to_owned(),
    };

    match becky.age{
        Some(age) => { println!("Customer is {:?} years old.",age)},
        None => println!("Customer Age is not Provided")
    }
    

    let groceries : Vec<GroceryItem> = vec! [
        GroceryItem{ name : "Apples".to_owned(),quantity : 99},
        GroceryItem{ name : "Bananas".to_owned(),quantity : 21},
        GroceryItem{ name : "Mangoes".to_owned(),quantity : 31},
        GroceryItem{ name : "Pineapples".to_owned(),quantity : 20},
        GroceryItem{ name : "cats".to_owned(),quantity : 230},
        GroceryItem{ name : "Oranges".to_owned(),quantity : 23},
    ];

    let name = "car";

    let quantity : Option<i32> = find_quantity(name,  &groceries);
    println!("{:?}",quantity);
}



