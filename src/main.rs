fn main(s: &[u8]) -> &[u8] {
    if s.len() < 2 { return s; }

    let mut current_start: usize = 0;
    let mut current_end: usize   = 0;
    let mut result_start = current_start;
    let mut result_end   = current_end;
    let mut previous_element = s[current_end];

    let with_indices = s.iter().enumerate();
    for (index, element) in with_indices.skip(1) {
        if previous_element < std::u8::MAX && previous_element + 1 == *element {
            previous_element = *element;
            current_end = index;
            if current_end - current_start > result_end - result_start {
                result_start = current_start;
                result_end = current_end;
            }
        } else {
            previous_element = *element;
            current_start = index;
            current_end = current_start;
        }
    }
    &s[result_start..=result_end]
}

// These three lines are to inform rust (cargo) that there are tests:
//   1. in a separate file
//   2. and in a submodule, so the test file can access all functions in this file
// This hoop-jumping is to allow segregating tests of a binary crate into a separate file.
// See http://xion.io/post/code/rust-unit-test-placement.html
#[cfg(test)]
#[path = "./main_test.rs"]
mod main_test;

// example function for proof of test-organizing concept:
fn forty_two() -> isize { 42 }
