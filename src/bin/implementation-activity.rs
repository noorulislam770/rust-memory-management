enum Color {
    Red,
    Green, 
    Blue 

}

impl Color {
    fn print (&self) {
        match self{
            Color::Red => println!("Red"),
            Color::Green => println!("Green"),
            Color::Blue => println!("Blue"),

        }
    }
}


struct Dimensions { 
    length : i32,
    width : i32,
    height: i32
}

impl Dimensions  {
    fn print (&self) { 
        println!("Dimensions");
        println!("width : {}", self.width);
        println!("height : {}", self.height);
        println!("length : {}", self.length);
    
    }
}

struct ShippingBox { 
    dimensions : Dimensions,
    weight : i32,
    color : Color
}

impl ShippingBox {


    fn new() -> Self {

        Self {
            dimensions : Dimensions { length : 1, width: 1, height: 1},
            weight : 20,
            color : Color::Red
        }
    }

    fn print_characteristics (&self) {
        println!("the box details are as follows");
        self.dimensions.print();
        self.color.print();
        println!("Weights is {} ", self.weight);
    
    }
}



fn main () {

    let temp_box = ShippingBox::new();
   temp_box.print_characteristics();
}



