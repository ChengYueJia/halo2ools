use halo2_proofs::arithmetic::*;
use halo2ools::params::*;

// cargo test test_save_and_load  -- --nocapture
// DEGREE=6 cargo test --test debug_msm test_save_and_load --features msm_cuda -- --nocapture
#[test]
fn test_save_and_load() {
    let param_10_file = "param_11.srs".to_string();

    let param10 = gen_param(10);

    save_param(&param_10_file, &param10);

    let actual = load_param(&param_10_file);
    assert_eq!(param10.g2(), actual.g2());
    assert_eq!(param10.s_g2(), actual.s_g2());
}
