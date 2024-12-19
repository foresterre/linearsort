use crate::LinearSort;

#[test]
fn sort() {
    i_cant_wait_forever!();

    let mut subject = [4_i32, -5, 1, -3, 2];

    subject.linear_sort_by_cached_key(|&a| a.abs().to_string());
    assert_eq!(subject, [1, 2, -3, 4, -5]);
}

#[test]
fn already_sorted() {
    i_cant_wait_forever!();

    let mut subject = [1_i32, 2, -3, 4, -5];

    subject.linear_sort_by_cached_key(|&a| a.abs().to_string());
    assert_eq!(subject, [1, 2, -3, 4, -5]);
}
