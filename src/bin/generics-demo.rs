//a generic is a data type that contains other types like
//a Vector data type can contain any number of data type like i32, f64, String, and custom types
//
//


enum Click {
    LeftClick,
    RightClick,
    MiddleClick,
}



fn main() {
    //this is a generic vector data type that in this cause is used 
    //for storing a list of i32 integers.
    let numbers : Vec<i32> = vec![1,2,3];
    
    //in this case using a vector to store characters.
    let letter : Vec<char> = vec!['a','b','c','d','e','f','g','h','i','j'];

    // and in this case using a vector data type to store Custom data types
    let clicks : Vec<Click>  = vec![Click::LeftClick,Click::RightClick,Click::MiddleClick];

}   
