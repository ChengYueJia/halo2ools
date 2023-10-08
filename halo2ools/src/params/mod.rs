use halo2_proofs::halo2curves::ff::PrimeField;
use halo2_proofs::halo2curves::pairing::Engine;
use halo2_proofs::poly::commitment::Params;
use halo2_proofs::poly::kzg::commitment::ParamsKZG;

use halo2_proofs::halo2curves::bn256::Bn256;
use rand::rngs::OsRng;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;

type ProverParams = ParamsKZG<Bn256>;

/// This utility supports parameter generation.
/// Can be invoked with: gen_params <degree> <path to file>
pub fn gen_param(degree: u32) -> ProverParams {
    let general_params = ProverParams::setup(degree, OsRng);
    println!("Generated params with degree: {degree}");
    general_params
}

pub fn gen_param_by_downsize(target_degree: u32, src_param: &ProverParams) -> ProverParams {
    let mut target = src_param.clone();
    target.downsize(target_degree);
    target
}

pub fn load_param(param_file: &String) -> ProverParams {
    let file = File::open(&param_file).expect("open exist param successfully");
    let params =
        ProverParams::read(&mut std::io::BufReader::new(file)).expect("Failed to read params");
    params
}

pub fn save_param(param_file: &String, param: &ProverParams) {
    let mut file = File::create(param_file).expect("Failed to create file");
    let mut buf = Vec::new();
    param.write(&mut buf).expect("Failed to write params");
    file.write_all(&buf[..])
        .expect("Failed to write params to file");
    println!("Success to write param: {param_file}");
}
