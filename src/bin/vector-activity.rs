
fn main() {
    let my_numbers = vec![10,20,30,40];


    for num in &my_numbers {
        match num {
            30 => println!("thirty"),
            _ => println!("{}",num),
        }
    }

    println!("length of the vector is {} ", my_numbers.len());
}
