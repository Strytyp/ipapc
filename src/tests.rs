use crate::{Init, IpaScheme};
use ark_ff::One;
use ark_pallas::{Fr, PallasParameters};
use ark_poly::{Polynomial, UVPolynomial};
use rand::thread_rng;

#[test]
fn test_hiding() {
    let mut rng = thread_rng();
    let scheme = IpaScheme::<PallasParameters, true>::init(Init::Seed(1), 3);
    let poly = [1, 2, 3, 4, 5, 6, 7, 8].map(Fr::from).to_vec();
    let commit = scheme.commit_hiding(poly.clone(), &mut rng);
    let point = Fr::from(5);
    let eval = {
        let poly = ark_poly::univariate::DensePolynomial::<Fr>::from_coefficients_slice(&*poly);
        poly.evaluate(&point)
    };
    let proof = scheme.open_hiding(commit.into(), &poly, point, eval, &mut rng);
    let bad_proof = scheme.open_hiding(commit.into(), &poly, point, eval + Fr::one(), &mut rng);
    assert_eq!(scheme.verify_hiding(commit.into(), proof).unwrap(), eval);
    assert!(scheme.verify_hiding(commit.into(), bad_proof).is_none());
}

#[test]
fn test_binding() {
    let scheme = IpaScheme::<PallasParameters, false>::init(Init::Seed(1), 3);
    let poly = [1, 2, 3, 4, 5, 6, 7, 8].map(Fr::from).to_vec();
    let commit = scheme.commit(poly.clone());
    let point = Fr::from(5);
    let eval = {
        let poly = ark_poly::univariate::DensePolynomial::<Fr>::from_coefficients_slice(&*poly);
        poly.evaluate(&point)
    };
    let proof = scheme.open(commit, &poly, point, eval);
    let bad_proof = scheme.open(commit, &poly, point, eval + Fr::one());
    assert_eq!(scheme.verify(commit, proof).unwrap(), eval);
    assert!(scheme.verify(commit, bad_proof).is_none());
}
