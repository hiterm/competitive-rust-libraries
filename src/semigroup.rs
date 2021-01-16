use std::marker::PhantomData;

use ac_library_rs::{MapMonoid, Monoid};

// ac-library

// SemiGroup

pub trait SemiGroup {
    type S: Clone;
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S;
}

#[derive(Clone)]
enum AdjIdElem<SG: SemiGroup> {
    Identity,
    Other(SG::S),
}

struct AdjId<SG> {
    phantom: PhantomData<SG>,
}
impl<SG> Monoid for AdjId<SG>
where
    SG: SemiGroup + Clone,
{
    type S = AdjIdElem<SG>;

    fn identity() -> Self::S {
        AdjIdElem::Identity
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        use AdjIdElem::{Identity, Other};
        match (a, b) {
            (Identity, Identity) => Identity,
            (Other(a), Identity) | (Identity, Other(a)) => Other(a.clone()),
            (Other(a), Other(b)) => Other(SG::binary_operation(a, b)),
        }
    }
}

// MapMonoid

pub trait SemiGroupMapMonoid {
    type M: Monoid;
    type F: Clone;

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S;
    fn composition(f: &Self::F, g: &Self::F) -> Self::F;
}

#[derive(Clone)]
enum AdjIdMapElem<SGM: SemiGroupMapMonoid> {
    Identity,
    Other(SGM::F),
}

struct AdjIdMap<SGM> {
    phantom: PhantomData<SGM>,
}
impl<SGM> MapMonoid for AdjIdMap<SGM>
where
    SGM: SemiGroupMapMonoid + Clone,
{
    type M = <SGM as SemiGroupMapMonoid>::M;
    type F = AdjIdMapElem<SGM>;

    fn identity_map() -> Self::F {
        AdjIdMapElem::Identity
    }

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        use AdjIdMapElem::*;

        match f {
            Identity => x.clone(),
            Other(f) => SGM::mapping(f, x),
        }
    }
    fn composition(f: &Self::F, g: &Self::F) -> AdjIdMapElem<SGM> {
        use AdjIdMapElem::*;

        match (f, g) {
            (Identity, Identity) => Identity,
            (Identity, Other(f)) | (Other(f), Identity) => Other(f.clone()),
            (Other(f), Other(g)) => Other(SGM::composition(f, g)),
        }
    }
}
