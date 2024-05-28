


#[derive(Debug,Clone,Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug,Clone,Copy)]
struct Employee {
    position : Position, 
    work_hours : i64, 
}

fn print_employee(emp: Employee) {
    println!("{:?}",emp);
}



fn main () {
    
    let me = Employee{ 
        position: Position::Manager,
        work_hours : 40,
    };

    println!("{:?}",me);

    print_employee(me);
    print_employee(me);
    print_employee(me);


}
