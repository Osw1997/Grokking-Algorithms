// This exercise is for creating a recursive function that is able to return that factorial of a number.


// Remember: It's a STACK
// When you call the recursive case, you are adding a new block
// to the STACK. 
// When you call the base case, you are adding the last block to the STACK.
fn factorial(x:u128) -> u128 {
    // Base case
    if x == 0 || x == 1 {
        x
    }
    // Recursive case
    else {
        x * factorial(x - 1)
    }
}

fn main() {
    let number: u128 = 30;
    let result: u128 = factorial(number);
    print!(">>> Factorial of {} is {}", number, result);
}
