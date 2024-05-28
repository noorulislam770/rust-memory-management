struct Test {
    score: i32,
}




fn main () {
    // this approach is using the vec! macro //
    // remember that macros in rust expand to a code when compiling
    let my_score  = vec![
        Test {score: 90},
        Test {score: 99},
        Test {score: 80},
        Test {score: 100},
    ];
    let my_numbers = vec![1,2,3];

    //this is the approach we use which is what the above vec! macro will
    //expand into once the code is about to compile
    //

    let mut my_nums = Vec::new();

    my_nums.push(1);
    my_nums.push(2);
    my_nums.push(3);
    my_nums.push(4);
    my_nums.pop();
    my_nums.len();

    for test in my_score {
        println!("{}", test.score);
    }

    let two = my_numbers[1];
    for num in my_numbers {

        //println!("{}",my_numbers);
        println!("{}",num);
    }
}

