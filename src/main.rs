fn main(s: &[u8]) -> &[u8] {
    if s.len() < 2 { return s; }

    let mut start: usize = 0;
    let mut sub: &[u8] = &s[0..1];
    let mut last = sub[0];
    let mut result = sub;

    let with_indices = s.iter().enumerate();
    for (index, element) in with_indices.skip(1) {
        if last < std::u8::MAX && last + 1 == *element {
            last = *element;
            sub = &s[start..(index + 1)];
            if sub.len() > result.len() {
                result = sub;
            }
        } else {
            start = index;
            sub = &s[start..(start+1)];
            last = sub[0];
        }
    }
    result
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
