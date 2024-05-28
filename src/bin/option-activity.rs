struct StudentLocker {
    name : String,
    number : Option<i32>,
}



fn main() {
    

    let mut new_batch_lockers : Vec<StudentLocker> = Vec::new();
    let my_locker = StudentLocker {
        name : "Noor ul islam".to_owned(),
        number : Some(923)
    };

    let your_locker = StudentLocker {
        name : "Hamza".to_owned(),
        number: None
    };

    let his_locker = StudentLocker {
        name : "Adibas Locker".to_owned(), 
        number: Some(2903),
    };
    
    new_batch_lockers.push(my_locker);
    new_batch_lockers.push(your_locker);
    new_batch_lockers.push(his_locker);
    for locker in new_batch_lockers {
        match locker.number {
            Some(number) => println!("the locker number for {} is {}",locker.name,number),
            None => println!("sorry no locker for {} is known",locker.name),
        }
       // println!("{}",locker.name);
    }





}
