//! file: test_itertools.rs
//! author: Jacob Xie
//! date: 2023/10/10 08:33:12 Tuesday
//! brief:

use itertools::Itertools;

#[test]
fn merge_by_success() {
    let a = (0..).zip("ab".chars());
    let b = (0..).zip("cde".chars());

    let it = a.merge_by(b, |x, y| x.0 == y.0);
    let res = it.collect::<Vec<_>>();
    println!("{:?}", res);

    ///////////////////////////////////////////////////////////////////////////////////////////////////

    let a = (0..).zip("ab".chars());
    let b = (0..).zip("cde".chars());

    let it = a.merge_by(b, |x, y| x.1 <= y.1);
    let res = it.collect::<Vec<_>>();
    println!("{:?}", res);
}

#[test]
fn merge_join_by_success() {
    let a = (0..).zip("ab".chars());
    let b = vec![(0, 0.1, "x"), (2, 2.2, "z")].into_iter();

    let it = a.merge_join_by(b, |x, y| (x.0).cmp(&y.0));
    let res = it.collect::<Vec<_>>();
    println!("{:?}", res);
}
