use ff::{Field, PrimeField};
use halo2_proofs::poly::{Basis, Polynomial};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::marker::PhantomData;
use std::ptr::read;

//------write
/// Write a Polynomial into file.(without montgomery)
#[allow(unused_must_use)]
pub(crate) fn write_field_into_file<F: PrimeField>(file_name: &str, coeff: &F) {
    let mut file = File::create(file_name).unwrap();
    file.write(coeff.to_repr().as_ref());
}
#[allow(unused_must_use)]
pub(crate) fn write_fields_into_file<F: PrimeField>(file_name: &str, coeffs: &Vec<F>) {
    let mut file = File::create(file_name).unwrap();

    for coeff in coeffs {
        file.write(coeff.to_repr().as_ref());
    }
}
#[allow(unused_must_use)]
pub(crate) fn write_poly_into_file<F: PrimeField, B: Basis>(
    file_name: &str,
    poly: &Polynomial<F, B>,
) {
    let mut file = File::create(file_name).unwrap();

    for coeff in &poly.values {
        file.write(coeff.to_repr().as_ref());
    }
}
/// Write a Polynomial into file.(without montgomery)
#[allow(unused_must_use)]
pub fn write_poly_vec_into_file<F: PrimeField, B: Basis>(
    file_name: &str,
    polys: &[Polynomial<F, B>],
) {
    let mut file = File::create(file_name).unwrap();

    for poly in polys {
        for coeff in &poly.values {
            file.write(coeff.to_repr().as_ref());
        }
    }
}

//------read
#[allow(unused_must_use)]
pub(crate) fn read_field_from_file<F: PrimeField>(file_name: &str) -> F {
    // open file
    let mut file = File::open(file_name).unwrap();

    // assert len
    let file_len = fs::metadata(file_name).unwrap().len();
    assert!(file_len >= (32), "file don't have enough bytes");
    // read bytes
    let mut buf = [0; 32];
    file.read(&mut buf);
    // convert to repr
    let mut repr = F::Repr::default();
    repr.as_mut().copy_from_slice(&buf);
    // new field
    F::from_repr(repr).unwrap()
}

#[allow(unused_must_use)]
pub(crate) fn read_fields_from_file<F: PrimeField>(file_name: &str, num_coeffs: u64) -> Vec<F> {
    // open file
    let mut file = File::open(file_name).unwrap();

    // assert len
    let file_len = fs::metadata(file_name).unwrap().len();
    assert!(
        file_len >= (num_coeffs * 32),
        "file don't have enough bytes"
    );

    let mut coeffs: Vec<F> = vec![];

    for _ in 0..num_coeffs {
        // read bytes
        let mut buf = [0; 32];
        file.read(&mut buf);
        // convert to repr
        let mut repr = F::Repr::default();
        repr.as_mut().copy_from_slice(&buf);
        // new field
        let coeff = F::from_repr(repr).unwrap();
        coeffs.push(coeff);
    }
    coeffs
}

#[allow(unused_must_use)]
pub(crate) fn read_poly_from_file<F: PrimeField, B: Basis>(
    file_name: &str,
    num_coeffs: u64,
) -> Polynomial<F, B> {
    // open file
    let mut file = File::open(file_name).unwrap();

    // assert len
    let file_len = fs::metadata(file_name).unwrap().len();
    assert!(
        file_len >= (num_coeffs * 32),
        "file don't have enough bytes"
    );

    let mut coeffs: Vec<F> = vec![];

    for _ in 0..num_coeffs {
        // read bytes
        let mut buf = [0; 32];
        file.read(&mut buf);
        // convert to repr
        let mut repr = F::Repr::default();
        repr.as_mut().copy_from_slice(&buf);
        // new field
        let coeff = F::from_repr(repr).unwrap();
        coeffs.push(coeff);
    }
    Polynomial {
        values: coeffs,
        _marker: PhantomData,
    }
}

/// Read a Polynomialinto file.(without montgomery)
#[allow(unused_must_use)]
pub fn read_poly_vec_from_file<F: PrimeField, B: Basis>(
    file_name: &str,
    polys_num: u64,
    num_coeffs: u64,
) -> Vec<Polynomial<F, B>> {
    // open file
    let mut file = File::open(file_name).unwrap();

    // assert len
    let file_len = fs::metadata(file_name).unwrap().len();
    assert!(
        file_len >= (polys_num * num_coeffs * 32),
        "file don't have enough bytes"
    );

    let mut results = vec![];
    for _ in 0..polys_num {
        let mut coeffs: Vec<F> = vec![];

        for _ in 0..num_coeffs {
            // read bytes
            let mut buf = [0; 32];
            file.read(&mut buf);
            // convert to repr
            let mut repr = F::Repr::default();
            repr.as_mut().copy_from_slice(&buf);
            // new field
            let coeff = F::from_repr(repr).unwrap();
            coeffs.push(coeff);
        }
        let poly = Polynomial {
            values: coeffs,
            _marker: PhantomData,
        };

        results.push(poly);
    }
    results
}
