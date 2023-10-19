//! file: macros.rs
//! author: Jacob Xie
//! date: 2023/10/18 16:36:34 Wednesday
//! brief:

#[macro_export]
macro_rules! fqx_val {
    () => {
        $crate::adt::value::FqxValue::Null
    };
    ($v:expr) => {
        $crate::adt::value::FqxValue::from($v)
    };
}

#[macro_export]
macro_rules! fqx_row {
    () => {
        $crate::adt::row::FqxRow::default()
    };
    ($($x:expr),+) => {{
        let mut v = vec![];
        $({
            v.push($crate::fqx_val!($x));
        })+
        $crate::adt::row::FqxRow(v)
    }};
}

#[macro_export]
macro_rules! fqx {
    ($(($($x:expr),* $(,)*)),+ $(,)*) => {{
        let mut v = vec![];
        $({
            let mut r = vec![];
            $(
                r.push($crate::fqx_val!($x));
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
        let a = fqx_val!();
        println!("{:?}", a);
        let a = fqx_val!(1);
        println!("{:?}", a);
    }

    #[test]
    fn test_fqx_row() {
        let a = fqx_row!();
        println!("{:?}", a);
        let a = fqx_row!(1, "ab", 2.0);
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
