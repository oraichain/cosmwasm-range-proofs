mod util;

mod errors;
mod generators;
mod inner_product_proof;
mod range_proof;
mod transcript;

pub use crate::{
	errors::ProofError,
	generators::{BulletproofGens, BulletproofGensShare, PedersenGens},
	range_proof::RangeProof,
};

pub mod range_proof_mpc {
	pub use crate::{
		errors::MPCError,
		range_proof::{dealer, messages, party},
	};
}
