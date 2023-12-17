//! file: macros.rs
//! author: Jacob Xie
//! date: 2023/10/18 16:36:34 Wednesday
//! brief:

#[macro_export]
macro_rules! fqx {
    () => {
        $crate::adt::val::value::FqxValue::Null
    };
    (()) => {
        $crate::adt::row::FqxRow::default()
    };
    ($v:expr) => {
        $crate::adt::val::value::FqxValue::from($v)
    };
    ($($x:expr),+) => {{
        let mut v = vec![];
        $({
            v.push($crate::adt::val::value::FqxValue::from($x));
        })+
        $crate::adt::row::FqxRow::new(v)
    }};
    ($(($($x:expr),+ $(,)*)),+ $(,)*) => {{
        let mut v = vec![];
        $({
            let mut r = vec![];
            $(
                r.push($crate::adt::val::value::FqxValue::from($x));
            )*
            v.push(r);
        })+

        $crate::adt::dat::data::FqxData::new_by_data(v)
    }};
}

#[macro_export]
macro_rules! fqxt {
    ($t:expr) => {
        $crate::adt::val::value::FqxValueType::unchecked_from_str($t)
    };
    ($($t:expr),+ $(,)*) => {{
        let mut v = vec![];
        $({
            v.push($crate::adt::val::value::FqxValueType::unchecked_from_str($t))
        })+
        v
    }};
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod tests_macros {

    #[test]
    fn test_fqx_val() {
        // create a `null` value
        let a = fqx!();
        println!("{:?}", a);
        // create a value
        let a = fqx!(1);
        println!("{:?}", a);
    }

    #[test]
    fn test_fqx_row() {
        // create an empty row
        let a = fqx!(());
        println!("{:?}", a);
        // create a row
        let a = fqx!(1, "ab", 2.0);
        println!("{:?}", a);
    }

    #[test]
    fn test_fqx_data() {
        // create a data
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
