use super::{ main, forty_two };

#[test]
fn returns_empty_array() {
    let empty: [u8; 0] = [];
    let result: [u8; 0] = [];
    assert_eq!(&result, main(&empty));
}

#[test]
fn returns_first_element_if_no_sequence() {
    assert_eq!(
        &[1],
        main(&[1, 3, 5, 7, 5, 3])
    );
}

#[test]
fn it_finds_one_element() {
    assert_eq!(&[2], main(&[2]));
}

#[test]
fn example_from_readme() {
    assert_eq!(
        &[4, 5, 6, 7],
        main(&[1, 2, 4, 4, 5, 6, 7, 3, 2, 7, 8, 9, 1])
    );
}

#[test]
fn head() {
    assert_eq!(&[1, 2, 3], main(&[1, 2, 3, 5]));
}

#[test]
fn tail() {
    assert_eq!(&[1, 2, 3], main(&[8, 1, 2, 3]));
}

#[test]
fn full() {
    assert_eq!(&[1, 2, 3], main(&[1, 2, 3]));
}

#[test]
fn minimum() {
    assert_eq!(&[0, 1, 2], main(&[2, 1, 0, 1, 2, 5, 6]));
}

#[test]
fn maximum() {
    assert_eq!(&[253, 254, 255], main(&[2, 5, 2, 253, 254, 255, 1]));
}

// proving access to other functions in main.rs
#[test]
fn test_forty_two() {
    assert_eq!(42, forty_two());
}
