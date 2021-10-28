/**
 * Script that makes binary search in a sorted array.
 */

// Arguments:
// array: [i32; _]
// number: i32
fn binary_search(array: &mut [i32], value_to_find: i32) -> i32 {

    let mut min: usize = 0;
    let mut max: usize = array.len();

    let mut index: f32 = array.len() as f32 / 2.0;
    index = index.ceil();
    // The way how I am going to implement this algorithm
    //  * 1.- Get center of array and its index
    loop {
        println!("Min {} -- INDX: {} -- Max: {}", min, index, max);
        println!("val_to_find: {} VS {} @ {}", value_to_find, array[index as usize], index);
        
        // println!("Center's index: {} --- Center's value: {}", index as usize, array[index as usize]);
        //  * 2.- Value if variable value_to_find equals to such value at center of array
        //  * 3.- IF center value is the value_to_find, return the index associated
        if value_to_find == array[index as usize] {
            return index as i32;
        //  * 4.- IF the value_to_find is not the center value
        //  *  4.2.- IF is lower, GOTO the first half of current array
        } else if index == (((max + min) / 2) as f32).ceil() {
            return -1;
        } else if value_to_find < array[index as usize] {
            println!("LEFT! GOTO first half");
            max = index as usize;
        //  *  4.1.- Value if value_to_find is greater than center value, GOTO the SECOND half of current array
        } else if value_to_find > array[index as usize] {
            println!("RIGHT! GOTO second half");
            min = index as usize;
        } 
        index = (((max as f32 + min as f32) / 2.0) as f32).ceil();
        // index = index.ceil();
        println!("\n");
    }
} 

fn main() {
    let mut array: [i32; 11] = [14, 16, 19, 27, 49, 50, 59, 79, 81, 94, 120];
    let value_to_find: i32 = 81;
    let index: i32 = binary_search(&mut array, value_to_find);

    if index != -1 {
        println!("index of value {} in array is {}", value_to_find, index);
    } else {
        println!("Value to find [{}] is not in the array {:?}", value_to_find, array);
    }   
}
