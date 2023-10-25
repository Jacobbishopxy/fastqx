//! file: macros.rs
//! author: Jacob Xie
//! date: 2023/10/18 16:36:34 Wednesday
//! brief:

#[macro_export]
macro_rules! fqx {
    () => {
        $crate::adt::value::FqxValue::Null
    };
    (()) => {
        $crate::adt::row::FqxRow::default()
    };
    ($v:expr) => {
        $crate::adt::value::FqxValue::from($v)
    };
    ($($x:expr),+) => {{
        let mut v = vec![];
        $({
            v.push($crate::adt::value::FqxValue::from($x));
        })+
        $crate::adt::row::FqxRow(v)
    }};
    ($(($($x:expr),+ $(,)*)),+ $(,)*) => {{
        let mut v = vec![];
        $({
            let mut r = vec![];
            $(
                r.push($crate::adt::value::FqxValue::from($x));
            )*
            v.push(r);
        })+

        $crate::adt::data::FqxData::new_by_data(v)
    }};
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod tests_macros {

    #[test]
    fn test_fqx_val() {
        let a = fqx!();
        println!("{:?}", a);
        let a = fqx!(1);
        println!("{:?}", a);
    }

    #[test]
    fn test_fqx_row() {
        let a = fqx!(());
        println!("{:?}", a);
        let a = fqx!(1, "ab", 2.0);
        println!("{:?}", a);
    }

    #[test]
    fn test_fqx_data() {
        let a = fqx!(
            (1, "a", 1.1),
            (2, "b", None::<f32>),
            (3, "c", 3.3),
            (4, "d", 4.4),
            (5, "e", 5.5),
        );

        println!("{:?}", a);
    }
}
