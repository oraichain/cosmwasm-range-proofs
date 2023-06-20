use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
use curve25519_dalek::ristretto::CompressedRistretto;
use curve25519_dalek::scalar::Scalar;
use merlin::Transcript;
use node_bindgen::core::{buffer::ArrayBuffer, buffer::JSArrayBuffer, NjError};
use node_bindgen::derive::node_bindgen;
use rand_chacha::{rand_core::SeedableRng, ChaChaRng};

const TRANSCRIPT_BYTES: &[u8] = b"Oraichain.RangeProof.Age";

#[node_bindgen]
fn gen_proof(secret: u32, n: u32) -> Result<Vec<ArrayBuffer>, NjError> {
    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(64, 8);

    // Use a deterministic RNG for proving, so the test vectors can be
    // generated reproducibly.

    let mut seeds = [0u8; 32];
    getrandom::getrandom(&mut seeds).map_err(|err| NjError::Other(err.to_string()))?;
    let mut test_rng = ChaChaRng::from_seed(seeds);
    let blinding = Scalar::random(&mut test_rng);

    let mut transcript = Transcript::new(TRANSCRIPT_BYTES);
    let (proof, value_commitment) = RangeProof::prove_single(
        &bp_gens,
        &pc_gens,
        &mut transcript,
        secret as u64,
        &blinding,
        n as usize,
    )
    .map_err(|err| NjError::Other(err.to_string()))?;

    Ok(vec![
        ArrayBuffer::new(proof.to_bytes()),
        ArrayBuffer::new(value_commitment.as_bytes().to_vec()),
    ])
}

#[node_bindgen]
fn verify(proof: JSArrayBuffer, commitment: JSArrayBuffer, n: u32) -> Result<bool, NjError> {
    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(64, 8);
    let range_proof =
        RangeProof::from_bytes(proof.as_bytes()).map_err(|err| NjError::Other(err.to_string()))?;
    let value_commitment = CompressedRistretto::from_slice(commitment.as_bytes());

    // this one to verify prover
    let mut transcript = Transcript::new(TRANSCRIPT_BYTES);
    let result = range_proof
        .verify_single(
            &bp_gens,
            &pc_gens,
            &mut transcript,
            &value_commitment,
            n as usize,
        )
        .is_ok();

    Ok(result)
}
