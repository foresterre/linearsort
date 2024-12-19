use crate::LinearSort;

#[test]
fn sort() {
    i_cant_wait_forever!();

    let mut subject = [4, -5, 1, -3, 2];

    assert!(!subject.is_sorted());
    subject.linear_sort_unstable();
    assert!(subject.is_sorted());
}

#[test]
fn already_sorted() {
    i_cant_wait_forever!();

    let mut subject = [-5, -3, 1, 2, 4];

    assert!(subject.is_sorted());
    subject.linear_sort_unstable();
    assert!(subject.is_sorted());
}
