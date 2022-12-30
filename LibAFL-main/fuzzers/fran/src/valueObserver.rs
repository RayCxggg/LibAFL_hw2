//! The `MapObserver` provides access a map, usually injected into the target

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::{
    fmt::Debug,
    hash::Hasher,
    iter::Flatten,
    marker::PhantomData,
    slice::{from_raw_parts, Iter, IterMut},
};

use ahash::AHasher;
use intervaltree::IntervalTree;
use num_traits::Bounded;
use serde::{Deserialize, Serialize};

use crate::{
    bolts::{
        ownedref::{OwnedRefMut, OwnedSliceMut},
        tuples::Named,
        AsIter, AsIterMut, AsMutSlice, AsSlice, HasLen,
    },
    executors::ExitKind,
    inputs::UsesInput,
    observers::{DifferentialObserver, Observer, ObserversTuple},
    Error,
};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(bound = "T: serde::de::DeserializeOwned")]
#[allow(clippy::unsafe_derive_deserialize)]
pub struct ValueObserver<const DIFFERENTIAL: bool>
{
    value: i32,
    name: String,
}

impl<'a, S, T> Observer<S> for ValueObserver<false>
where
    S: UsesInput,
    T: Bounded
        + PartialEq
        + Default
        + Copy
        + 'static
        + Serialize
        + serde::de::DeserializeOwned
        + Debug,
{
    #[inline]
    fn pre_exec(&mut self, _state: &mut S, _input: &S::Input) -> Result<(), Error> {
        self.reset_value()
    }
}

impl<'a, S, T> Observer<S> for ValueObserver<true>
where
    S: UsesInput,
    T: Bounded
        + PartialEq
        + Default
        + Copy
        + 'static
        + Serialize
        + serde::de::DeserializeOwned
        + Debug,
{
}

impl<const DIFFERENTIAL: bool> Named for ValueObserver<DIFFERENTIAL>
{
    #[inline]
    fn name(&self) -> &str {
        self.name.as_str()
    }
}
