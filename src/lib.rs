/* Check file difference */

/* Accepts a string with a name.
If the name is "Marco", returns "Polo".
If the name is "any other value", it returns "Marco".
*/
use std::fs;

pub fn check_diff(file1: &str, file2: &str) -> bool {
    let contents1 = fs::read_to_string(file1).expect("Error reading file 1");
    let contents2 = fs::read_to_string(file2).expect("Error reading file 2");

    contents1 == contents2
}
