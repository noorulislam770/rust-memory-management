struct Person {
    name : String,
    age : i32,
    color : String,
}

fn print(name : &str, color : &str) {
    println!("{} likes {} color", name, color);
    
}


fn main () {

    let users = vec![
        Person{ 
            name: String::from("ali"),
            age : 11,
            color : "Red".to_owned(),
        },

        Person{ 
            name: String::from("Noor"),
            age : 9,
            color : "Blue".to_owned(),
        },
        
        Person{ 
            name: String::from("Ahmad"),
            age : 8,
            color : "Green".to_owned(),
        },

        Person{ 
            name: String::from("Hera"),
            age : 19,
            color : "Yellow".to_owned(),
        },
        Person{ 
            name: String::from("Hammad"),
            age : 5,
            color : "Purple".to_owned(),
        },
        ];
    for user in users{ 

        if user.age <= 10 {
            print(&user.name,&user.color);
        }

    }
}
