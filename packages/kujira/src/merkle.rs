use cosmwasm_std::StdError;
use hex::FromHexError;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::convert::TryInto;
use thiserror::Error;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Merkle {
    root: String,
}

pub type Proof = Vec<String>;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Hex(#[from] FromHexError),

    #[error("Wrong length")]
    WrongLength {},

    #[error("Verification failed")]
    VerificationFailed {},
}

impl Merkle {
    pub fn new(root: String) -> Result<Self, Error> {
        let mut root_buf: [u8; 32] = [0; 32];
        hex::decode_to_slice(&root, &mut root_buf)?;

        Ok(Self { root })
    }

    pub fn verify(&self, proof: Proof, input: String) -> Result<(), Error> {
        let hash = sha2::Sha256::digest(input.as_bytes())
            .as_slice()
            .try_into()
            .map_err(|_| Error::WrongLength {})?;

        let hash = proof.into_iter().try_fold(hash, |hash, p| {
            let mut proof_buf = [0; 32];
            hex::decode_to_slice(p, &mut proof_buf)?;
            let mut hashes = [hash, proof_buf];
            hashes.sort_unstable();
            sha2::Sha256::digest(hashes.concat())
                .as_slice()
                .try_into()
                .map_err(|_| Error::WrongLength {})
        })?;

        let mut root_buf: [u8; 32] = [0; 32];
        hex::decode_to_slice(self.root.clone(), &mut root_buf)?;
        if root_buf != hash {
            return Err(Error::VerificationFailed {});
        }
        Ok(())
    }
}
