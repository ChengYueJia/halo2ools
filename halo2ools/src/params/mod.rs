use halo2_proofs::{
    halo2curves::bn256::Bn256,
    poly::{commitment::Params, kzg::commitment::ParamsKZG},
};
use rand::rngs::OsRng;

use halo2_proofs::halo2curves::pairing::{Engine, MultiMillerLoop};
use std::fs::File;
use std::io::Write;
use std::path::Path;

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
    let file = File::open(param_file).expect("open exist param successfully");

    ProverParams::read(&mut std::io::BufReader::new(file)).expect("Failed to read params")
}

pub fn save_param(param_file: &String, param: &ProverParams) {
    let mut file = File::create(param_file).expect("Failed to create file");
    let mut buf = Vec::new();
    param.write(&mut buf).expect("Failed to write params");
    file.write_all(&buf[..])
        .expect("Failed to write params to file");
    println!("Success to write param: {param_file}");
}

pub fn load_or_build_unsafe_params<E: Engine>(
    k: u32,
    cache_file_opt: Option<&Path>,
) -> ParamsKZG<E> {
    if let Some(cache_file) = &cache_file_opt {
        if Path::exists(&cache_file) {
            println!("read params K={} from {:?}", k, cache_file);
            let mut fd = std::fs::File::open(&cache_file).unwrap();
            return ParamsKZG::<E::G1Affine>::read(&mut fd).unwrap();
        }
    }

    let params = ParamsKZG::<E::G1Affine>::unsafe_setup::<E>(k);

    if let Some(cache_file) = &cache_file_opt {
        println!("write params K={} to {:?}", k, cache_file);
        let mut fd = std::fs::File::create(&cache_file).unwrap();
        params.write(&mut fd).unwrap();
    };

    params
}
