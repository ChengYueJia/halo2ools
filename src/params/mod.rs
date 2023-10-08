use halo2_proofs::poly::commitment::Params;
use prover::ProverParams;
use rand::rngs::OsRng;
use std::env;
use std::fs::File;
use std::io::Write;


pub type ProverParams = ParamsKZG<Bn256>;
pub type ProverCommitmentScheme = KZGCommitmentScheme<Bn256>;
pub type ProverKey = ProvingKey<G1Affine>;


/// This utility supports parameter generation.
/// Can be invoked with: gen_params <degree> <path to file>
fn generate_param(degree: u32) -> ProverParams{
    let general_params = ProverParams::setup(degree, OsRng);
    println!("Generated params with degree: {degree}");
}

fn gen_param_by_downsize(target_degree:u32, src_param:&ProverParams  ) -> ProverParams{
    assert!( src_param.k() < target_degree , "source param'k is less than target degree");
    if src_param.k() == target_degree {
        src_param.clone()
    }else {
        src_param.downsize(target_degree).clone()
    }

}

fn load_param(param_file:&String) -> ProverParams {

    let file = File::open(&param_file).expect("open exist param successfully");
    let params = ProverParams::read(&mut std::io::BufReader::new(file)).expect("Failed to read params"),
    params
}




fn save_param(param_file:&String, param: &ProverParams)  -> Result(()){
    let mut file = File::create(param_file).expect("Failed to create file");
    let mut buf = Vec::new();
    param
        .write(&mut buf)
        .expect("Failed to write params");
    file.write_all(&buf[..])
        .expect("Failed to write params to file");
    println!("Success to write param: {param_file}");
}