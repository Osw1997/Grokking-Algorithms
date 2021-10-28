fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


fn get_smallest_number(subvector: Vec<i32>) -> usize {
    // println!("This VECTOR: {:?}", subvector);
    let mut smallest_number_index: usize = 0;
    for index_number in 1..subvector.len() {
        // println!("Go: {}, indx: {}", subvector[index_number], index_number);
        if subvector[index_number] < subvector[smallest_number_index] {
            smallest_number_index = index_number;
        }
    }
    smallest_number_index    
}


fn selection_sort(vector: &mut Vec<i32>, ) -> Vec<i32> {
    let mut new_vector: Vec<i32> = vec![-1; vector.len()];
    // println!("Original array: {:?}", vector);
    // println!("new_vector initialized: {:?}", new_vector);
    for index in 1..vector.len() {
        let smallest_number_index: usize = get_smallest_number(vector.to_vec());
        new_vector[index] = vector[smallest_number_index];
        // println!("Smallest number: {}", vector[smallest_number_index]);
        vector.remove(smallest_number_index);        
    }
    new_vector
}


fn main() {
    // Let's create the second algorithm presented in the book "Grokking algorithms"
    // Selection sort algorithm
    
    // Given an array, this algorithm sorts increasingly its elements
    // let mut array: [i32; 10] = [10,204,28,2,50,203, 19, 39, 940, 1];
    let mut vector: Vec<i32> = vec![10,204,28,2,50,203, 19, 39, 940, 1, 0304, 48, -2, 40, 304];
    let sorted_vector = selection_sort(&mut vector);  // Type: &mut [i32]
    println!("Sorted array: {:?}", sorted_vector);
    print_type_of(&sorted_vector);

}
