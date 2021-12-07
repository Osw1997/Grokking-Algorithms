// This file contains exercises and the Quicksort algorithm

// Sum recursive function
fn recursive_sum(arr: &mut Vec<i32>) -> i32 {
    // I need to sum the first element and call again this same function 
    // but with a reduced array[offset:end], where offset = 1
    // Base case
    if arr.len() == 0 {
        0
    }
    else if arr.len() == 1{
        arr[0]
    }
    // Recursive case
    else{
        // let mut sub_vector: Vec<i32> = arr;
        // sub_vector.remove(0);
        // arr[0] + recursive_sum(sub_vector.to_vec())
        let first_element: i32 = arr[0];
        arr.remove(0);
        first_element + recursive_sum(arr)
    }
}

// Count the numbers of list 
fn recursive_counter(arr: &mut Vec<i32>) -> i32 {
    // Sum one if the array is not empyt. Call again the function "count" if array's lenght is grater than 2
    if arr.len() == 1  || arr.len() == 0 {
        arr.len() as i32
    } else {
        arr.remove(0);
        1 + recursive_counter(arr)
    }
}

// Find the maximum number in a list
fn recursive_max(arr: &mut Vec<i32>, max: i32) -> i32 {
    // I check if the first value is grater that the argument, that is the previous maximum value.
    // I call the same function again if there are more elements in the array.
    let mut aux: i32 = -1;
    // // base case(s)
    if arr.len() == 0 {
        return max;
    }
    if arr[0] > max {
        aux = arr[0];
    }
    arr.remove(0);
    recursive_max(arr, aux)
    // recursive case
}

fn main() {
    println!("====================================================");
    let mut array: Vec<i32> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 10];
    println!("\t\t Recursive sum");
    print!("The sum of {:?} is ", array);
    let sum_res: i32 = recursive_sum(&mut array);
    println!("{}", sum_res);
    println!("====================================================");
    let mut array: Vec<i32> = vec![1,2,3,4];
    println!("\t\t Recursive counter");
    print!("The number of elements in {:?} is ", array);
    let counter_elements: i32 = recursive_counter(&mut array);
    println!("{}", counter_elements);
    println!("====================================================");
    let mut array: Vec<i32> = vec![-1, -23433434, 1,2,3,4, 23, 0, 9999];
    println!("\t\t Recursive maximum number finder");
    print!("The greatest number in {:?} is ", array);
    let max_val: i32 = recursive_max(&mut array, -1);
    println!("{}", max_val);
    println!("====================================================");
}
