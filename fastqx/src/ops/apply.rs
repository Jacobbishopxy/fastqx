//! file: apply.rs
//! author: Jacob Xie
//! date: 2023/09/23 18:34:46 Saturday
//! brief:

use anyhow::Result;

use crate::adt::FqxD;

// ================================================================================================
// OpApply
// ================================================================================================

pub trait OpApply<const OWNED: bool> {
    type Item;

    fn apply<O, F>(self, f: F) -> Vec<O>
    where
        F: Fn(Self::Item) -> O;

    fn try_apply<O, F>(self, f: F) -> Result<Vec<O>>
    where
        F: Fn(Self::Item) -> Result<O>;
}

// ================================================================================================
// Impl
// ================================================================================================

impl<U> OpApply<true> for U
where
    U: FqxD,
{
    type Item = U::RowT;

    fn apply<O, F>(self, f: F) -> Vec<O>
    where
        F: Fn(Self::Item) -> O,
    {
        self.data()
            .into_iter()
            .map(|r| f(r.clone()))
            .collect::<Vec<_>>()
    }

    fn try_apply<O, F>(self, f: F) -> Result<Vec<O>>
    where
        F: Fn(Self::Item) -> Result<O>,
    {
        self.data()
            .into_iter()
            .map(|r| f(r.clone()))
            .collect::<Result<Vec<_>>>()
    }
}

impl<'a, U> OpApply<false> for &'a U
where
    U: FqxD,
{
    type Item = &'a U::RowT;

    fn apply<O, F>(self, f: F) -> Vec<O>
    where
        F: Fn(Self::Item) -> O,
    {
        self.data().into_iter().map(|r| f(r)).collect::<Vec<_>>()
    }

    fn try_apply<O, F>(self, f: F) -> Result<Vec<O>>
    where
        F: Fn(Self::Item) -> Result<O>,
    {
        self.data()
            .into_iter()
            .map(|r| f(r))
            .collect::<Result<Vec<_>>>()
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_apply {
    use super::*;
    use crate::fqx;
    use crate::ops::mock::data::D1;
    use crate::ops::OpSelect;

    #[test]
    fn apply_self_success() {
        let data = D1.clone();

        // &FqxData
        let foo = (&data).apply(|r| &r[2] * &fqx!(2));
        println!("{:?}", foo);

        // FqxData
        let foo = data.apply(|r| r[2].clone() * 2.into());
        println!("{:?}", foo);
    }

    #[test]
    fn apply_select_success() {
        let data = D1.clone();

        // Vec<FqxRowSelect<&FqxValue>>
        let select = (&data).select([0, 2].as_slice());
        let foo = select.apply(|s| &s[0] + &s[1]);
        println!("{:?}", foo);

        // Vec<FqxRowSelect<FqxValue>>
        let select = data.select([0, 2, 1].as_slice());
        let foo = select.apply(|s| (s[0].clone(), s[2].clone()));
        println!("{:?}", foo);
    }
}
