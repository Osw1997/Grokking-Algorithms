use rand::Rng;
// This file contains exercises and the Quicksort algorithm

use std::{thread, time};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


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

fn recursive_binary_search(left_index: usize, right_index: usize, number: i32, arr: Vec<i32>) -> i32 {
    // Everything is about the use of index.
    // 1.- Get middle index (PIVOT)
    let pivot_indx: usize;
    if arr.len() == 0 {
        return -1;
    } else if (left_index == right_index) && (arr[left_index] != number) {
        return -1;
    } else if ((right_index + left_index) % 2) == 0 { // ODD
        pivot_indx = (right_index + left_index) / 2;
        println!(">>> Odd!");
    } else {
        pivot_indx = (right_index + left_index - 1) / 2;
        println!(">>> Even!");
    }

    println!("L: {} >> {}[{}] << R: {}", left_index, pivot_indx, arr[pivot_indx], right_index);
    // println!("index: {} -- value: {}", pivot_indx, arr[pivot_indx]);
    print!("[");
    for n in left_index..=right_index {
        print!("{}, ", arr[n]);
    }
    println!("]");

    // 2.- Evaluate if value associated to index is the number that you are looking for
    //  2.1.- [BASE CASE] If number is the looked for, return the index
    if number == arr[pivot_indx] {
        return pivot_indx as i32;
    }
    //  2.2.- [RECURSIVE CASE] If not, call again this same function and repeat the same steps using the left/right (</>) half of current array.
    let new_left_index: usize;
    let new_right_index: usize;
    if arr[pivot_indx] < number {
        new_left_index = pivot_indx + 1;
        new_right_index = right_index;
    } else {
        new_left_index = left_index;
        new_right_index = pivot_indx - 1;
    }

    thread::sleep(time::Duration::from_secs(1));
    recursive_binary_search(new_left_index, new_right_index, number, arr)

}

fn main2() {
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
    println!("\t\t Recursive binary search");
    let array: Vec<i32> = vec![0, 2,5,8, 23, 56, 78, 3034, 3049, 79545, 304040];
    // let number: i32 = 30404;
    // println!("Number {} in Array: {:?}", number, array);
    // let mut left_index: usize = 0;
    // let mut right_index: usize = array.len();
    for n in 0..array.len() {
        println!("========================================================================================================");
        println!("========================================================================================================");
        println!("========================================================================================================");
        let bkp_array = vec![0, 2,5,8, 23, 56, 78, 3034, 3049, 79545, 304040];
        println!("Number {} in Array: {:?}", bkp_array[n], array);
        let index_number: i32 = recursive_binary_search(0, bkp_array.len() - 1, bkp_array[n], bkp_array);
        if index_number > -1 {
            let bkp_array = vec![0, 2,5,8, 23, 56, 78, 3034, 3049, 79545, 304040];
            print!("The index of number {} in the array {:?} is the ", bkp_array[n], bkp_array);
            println!("{}th", index_number);
            assert_eq!(index_number as usize, n);
        } else {
            let bkp_array = vec![0, 2,5,8, 23, 56, 78, 3034, 3049, 79545, 304040];
            println!("The number {} does not exist in the array {:?}", bkp_array[n], bkp_array)
        }    
    }
    println!("========================================================================================================");
    println!("========================================================================================================");
    println!("========================================================================================================");
    let bkp_array = vec![0, 2,5,8, 23, 56, 78, 3034, 3049, 79545, 304040];
    println!("Number {} in Array: {:?}", 4, array);
    let index_number: i32 = recursive_binary_search(0, bkp_array.len() - 1, 4, bkp_array);
    let bkp_array = vec![0, 2,5,8, 23, 56, 78, 3034, 3049, 79545, 304040];
    print!("The index of number {} in the array {:?} is the ", 4, bkp_array);
    println!("{}th", index_number);
    println!("====================================================");


}

fn quicksort(unsorted_list: Vec<i32>) -> Vec<i32> {


    println!("");
    println!("========================================================================================================");
    // Base case
    if unsorted_list.len() == 2 {
        if unsorted_list[0] > unsorted_list[1] {
            return vec![unsorted_list[1], unsorted_list[0]];
        } else {
            return unsorted_list;
        }
    } else if unsorted_list.len() < 2 {
        println!("Empty or one element: {}", unsorted_list.len());
        return unsorted_list;
    } else { // Recursive case
        let lenght_ul: i32 = unsorted_list.len() as i32;
        println!("Length of unsorted_list is: {} elements", lenght_ul);
        // for n in 1..6 {
        //     println!("A random number between 0 and {} is {}", lenght_ul - 1, rand::thread_rng().gen_range(0..lenght_ul - 1));
        // }

        // Let's choose a random pivot
        let random_pivot_index: usize = rand::thread_rng().gen_range(0..lenght_ul - 1) as usize;
        let pivot: i32 = unsorted_list[random_pivot_index];
        println!("Random index: {} --- Pivot: {}", random_pivot_index, pivot);
        
        let mut list_less_than_pivot: Vec<i32> = Vec::new();
        let mut list_greater_than_pivot: Vec<i32> = Vec::new();

        for cursor in 0..lenght_ul {
            if cursor as usize != random_pivot_index { // It must be different to current pivot
                if unsorted_list[cursor as usize] <= pivot {
                    list_less_than_pivot.push(unsorted_list[cursor as usize]);
                } else {
                    list_greater_than_pivot.push(unsorted_list[cursor as usize]);
                }
            }
        }

        println!("QS([{:?}]) + [{:?}] + QS([{:?}])", list_less_than_pivot, vec![pivot], list_greater_than_pivot);

        let sorted_list_less: Vec<i32> = quicksort(list_less_than_pivot);
        let sorted_list_greater: Vec<i32> = quicksort(list_greater_than_pivot);

        // Creation of sorted cursor
        let mut sorted_list: Vec<i32> = Vec::new();

        // Left size
        for cursor in 0..sorted_list_less.len() {
            sorted_list.push(sorted_list_less[cursor as usize]);
        }
        // Pivot
        sorted_list.push(pivot);
        // Right side
        for cursor in 0..sorted_list_greater.len() {
            sorted_list.push(sorted_list_greater[cursor as usize]);
        }
        println!("New sorted list: {:?}", sorted_list);
        
        // Return sorted list
        sorted_list
    }
}

fn main() {
    println!("========================================================================================================");
    println!("Quicksort algorithm - D&C");
    println!("========================================================================================================");

    // Let's put in practice ownership knowledge
    let unsorted_list: Vec<i32> = vec![4,2,5,8,20,4, -1, 0, -100, 10000];
    println!("Unsorted list: {:?}", unsorted_list);
    let sorted_list = quicksort(unsorted_list); // the ownership of unsorted_list it's passed to quicksort function. It not longer exists.
    println!("Sorted list: {:?}", sorted_list); // The ouwnership of returned list is passed to main function
}
