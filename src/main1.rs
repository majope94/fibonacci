fn main() {
    let mut num_array = vec![0, 1]; // inital vector
    let n = 45; // the nth fibonacci number

    // for loop adding values of first two elements, then increase index by one and go through
    // vector appending result of summation
    for i in 0..n {
        let value_one = num_array[i];
        let value_two = num_array[i+1];
        num_array.push(value_one + value_two);

    }

    // for item in num_array {
    //     println!("{}", item);
    // }


    // print the nth fibonacci number
    println!("{}", num_array.last().unwrap());
}
