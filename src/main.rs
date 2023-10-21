fn get_highest <'a>(first_number: & 'a i8, second_number: & 'a i8) -> & 'a i8 {

    if first_number > second_number {
        first_number
    }  else { 
    second_number
    }
}

fn filter<'a, 'b> (f1_number: & 'a i8, f2_number: & 'b i8) -> & 'a i8 {
    if f1_number < f2_number { &0 
    } else {
        f1_number
    }
}


fn main() {
    /* let one: &i8;
    {
        let two: i8 = 2;
        one = &two;
    } // two lifetime stops here

    println!("r:{}", one); */

    // lifetimes
    let new_one: i8 = 1;
    let outcome: &i8;
    {
        let new_two: i8 = 2;
        outcome = filter(&new_one, &new_two);
    }

    println!("Outcome is {}", outcome);


}
