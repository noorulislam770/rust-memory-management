struct GroceryItem {
    id : i32,
    quantity : i32,
}


fn display_quantity (item : &GroceryItem) {
    println!("the quantity of the item/s is {}", item.quantity);

}

fn display_id (item: &GroceryItem) { 
    println!("the ID of the item is {}", item.id);
}


fn main () {
    let item = GroceryItem { 
        id : 1990,
        quantity : 203,
    };

    display_quantity(&item);
    display_id(&item);

}
