use crate::Fr;
use ark_ec::{
    short_weierstrass_jacobian::{GroupAffine, GroupProjective},
    AffineCurve, ProjectiveCurve, SWModelParameters,
};
use ark_ff::Field;
use std::{
    iter::repeat,
    ops::{Add, Mul},
};

pub fn compress_basis<P: SWModelParameters>(
    left: &[GroupAffine<P>],
    right: &[GroupAffine<P>],
    challenge: Fr<P>,
) -> Vec<GroupAffine<P>> {
    assert_eq!(left.len(), right.len());
    let inverse = challenge.inverse().unwrap();
    let left = left.iter().map(|elem| elem.mul(inverse));
    let right = right.iter().map(|elem| elem.mul(challenge));
    left.zip(right)
        .map(|(a, b)| (a + b).into_affine())
        .collect()
}
pub fn compress<P: SWModelParameters>(
    left: &[Fr<P>],
    right: &[Fr<P>],
    challenge: Fr<P>,
) -> Vec<Fr<P>> {
    assert_eq!(left.len(), right.len());
    let inverse = challenge.inverse().unwrap();
    let left = left.iter().map(|elem| *elem * inverse);
    let right = right.iter().map(|elem| *elem * challenge);
    left.zip(right).map(|(a, b)| a + b).collect()
}

pub fn inner_product<P: SWModelParameters>(
    a: &[GroupAffine<P>],
    b: &[Fr<P>],
) -> GroupProjective<P> {
    assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| a.mul(*b))
        .reduce(Add::add)
        .unwrap()
}
pub fn scalar_inner_product<P: SWModelParameters>(a: &[Fr<P>], b: &[Fr<P>]) -> Fr<P> {
    assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| *a * *b)
        .reduce(Add::add)
        .unwrap()
}
pub fn split<T>(slice: &[T]) -> (&[T], &[T]) {
    let len = slice.len();
    assert_eq!(len % 2, 0);
    (&slice[0..len / 2], &slice[len / 2..])
}

pub(crate) struct PolySegment<P: SWModelParameters> {
    inverse: Fr<P>,
    challenge: Fr<P>,
    exp: u64,
}
#[derive(Default)]
pub(crate) struct SPoly<P: SWModelParameters>(Vec<PolySegment<P>>);

impl<P: SWModelParameters> SPoly<P> {
    pub(crate) fn eval(self, point: Fr<P>) -> Fr<P> {
        self.0
            .into_iter()
            .map(|segment| {
                let PolySegment {
                    inverse,
                    challenge,
                    exp,
                } = segment;
                inverse + challenge * point.pow([exp])
            })
            .reduce(Mul::mul)
            .unwrap()
    }
    pub(crate) fn new(size_hint: usize) -> Self {
        Self(Vec::with_capacity(size_hint))
    }
    pub(crate) fn add_term(mut self, inverse: Fr<P>, challenge: Fr<P>, exp: u64) -> Self {
        let term = PolySegment {
            inverse,
            challenge,
            exp,
        };
        self.0.push(term);
        self
    }
}

pub(crate) fn s_vec<P: SWModelParameters>(challenges: Vec<(Fr<P>, Fr<P>)>) -> Vec<Fr<P>> {
    let size = challenges.len();
    let size = 2_usize.pow(size as u32) as usize;
    let mut challenges = challenges
        .into_iter()
        .enumerate()
        .map(|(i, (challenge, inverse))| {
            let segment_size = size / (2_usize.pow(i as u32 + 1));
            let challenge_segment = repeat(challenge).take(segment_size);
            let inverse_segment = repeat(inverse).take(segment_size);
            let combined = inverse_segment.chain(challenge_segment);
            combined.cycle()
        })
        .collect::<Vec<_>>();
    let f = || {
        let elem = challenges
            .iter_mut()
            .filter_map(|iter| iter.next())
            .reduce(Mul::mul);
        elem
    };
    let s = std::iter::from_fn(f).take(size).collect::<Vec<_>>();
    debug_assert_eq!(size, s.len());
    s
}
