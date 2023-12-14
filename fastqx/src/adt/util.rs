//! file: util.rs
//! author: Jacob Xie
//! date: 2023/11/27 20:18:18 Monday
//! brief:

use std::{borrow::Cow, collections::HashSet};

use crate::adt::FromTo;

pub(crate) fn slice_vec<'a, E>(mut vec: Vec<E>, range: impl FromTo) -> Vec<E> {
    let (start, end) = range.from_to(vec.len());
    if start > end {
        return Vec::default();
    }

    vec.drain(..start);
    vec.truncate(end - start + 1);

    vec
}

pub(crate) fn takes_vec<'a, E>(vec: Vec<E>, indices: impl IntoIterator<Item = usize>) -> Vec<E> {
    let indices = indices.into_iter().collect::<HashSet<_>>();

    vec.into_iter()
        .enumerate()
        .filter_map(|(i, e)| if indices.contains(&i) { Some(e) } else { None })
        .collect()
}

pub(crate) fn slice_cow<'a, E>(cow: Cow<'a, [E]>, range: impl FromTo) -> Cow<'a, [E]>
where
    [E]: ToOwned<Owned = Vec<E>>,
{
    let (start, end) = range.from_to(cow.len());
    if start > end {
        return Cow::Borrowed(&[]);
    }

    match cow {
        Cow::Borrowed(slice) => slice
            .get(start..=end)
            .map(Cow::Borrowed)
            .unwrap_or(Cow::Borrowed(&[])),
        Cow::Owned(mut vec) => {
            vec.drain(..start);
            vec.truncate(end - start + 1);
            Cow::Owned(vec)
        }
    }
}

pub(crate) fn takes_cow<'a, E>(
    cow: Cow<'a, [E]>,
    indices: impl IntoIterator<Item = usize>,
) -> Cow<'a, [E]>
where
    [E]: ToOwned<Owned = Vec<E>>,
    E: Clone,
{
    let indices = indices.into_iter().collect::<HashSet<_>>();

    match cow {
        Cow::Borrowed(slice) => {
            let v = slice
                .iter()
                .enumerate()
                .filter_map(|(i, e)| {
                    if indices.contains(&i) {
                        Some(e.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            Cow::Owned(v)
        }
        Cow::Owned(vec) => {
            let v = vec
                .into_iter()
                .enumerate()
                .filter_map(|(i, e)| if indices.contains(&i) { Some(e) } else { None })
                .collect::<Vec<_>>();

            Cow::Owned(v)
        }
    }
}
