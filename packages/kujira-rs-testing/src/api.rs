use bech32::{encode, primitives::decode::CheckedHrpstring, Bech32, Hrp};
use cosmwasm_std::{
    testing::MockApi, Addr, Api, CanonicalAddr, RecoverPubkeyError, StdError, StdResult,
    VerificationError,
};

use sha2::{Digest, Sha256};

pub struct MockApiBech32 {
    prefix: &'static str,
    inner: MockApi,
}

impl MockApiBech32 {
    pub fn new(prefix: &'static str) -> Self {
        Self {
            prefix,
            inner: MockApi::default(),
        }
    }
}

impl Api for MockApiBech32 {
    fn addr_validate(&self, input: &str) -> StdResult<Addr> {
        let canonical = self.addr_canonicalize(input)?;
        let normalized = self.addr_humanize(&canonical)?;
        if input != normalized.to_string() {
            Err(StdError::generic_err(
                "Invalid input: address not normalized",
            ))
        } else {
            Ok(Addr::unchecked(input))
        }
    }

    fn addr_canonicalize(&self, input: &str) -> StdResult<CanonicalAddr> {
        if let Ok(decoded) = CheckedHrpstring::new::<Bech32>(input) {
            if decoded.hrp().as_str() == self.prefix {
                return Ok(decoded.byte_iter().collect::<Vec<u8>>().into());
            }
        }
        Err(StdError::generic_err("Invalid input"))
    }

    fn addr_humanize(&self, canonical: &CanonicalAddr) -> StdResult<Addr> {
        bech32::encode::<Bech32>(Hrp::parse_unchecked(self.prefix), canonical.as_slice())
            .map_or_else(
                |_| Err(StdError::generic_err("Invalid canonical address")),
                |encoded| Ok(Addr::unchecked(encoded)),
            )
    }

    fn secp256k1_verify(
        &self,
        message_hash: &[u8],
        signature: &[u8],
        public_key: &[u8],
    ) -> Result<bool, VerificationError> {
        self.inner
            .secp256k1_verify(message_hash, signature, public_key)
    }

    fn secp256k1_recover_pubkey(
        &self,
        message_hash: &[u8],
        signature: &[u8],
        recovery_param: u8,
    ) -> Result<Vec<u8>, RecoverPubkeyError> {
        self.inner
            .secp256k1_recover_pubkey(message_hash, signature, recovery_param)
    }

    fn ed25519_verify(
        &self,
        message: &[u8],
        signature: &[u8],
        public_key: &[u8],
    ) -> Result<bool, VerificationError> {
        self.inner.ed25519_verify(message, signature, public_key)
    }

    fn ed25519_batch_verify(
        &self,
        messages: &[&[u8]],
        signatures: &[&[u8]],
        public_keys: &[&[u8]],
    ) -> Result<bool, VerificationError> {
        self.inner
            .ed25519_batch_verify(messages, signatures, public_keys)
    }

    fn debug(&self, message: &str) {
        self.inner.debug(message);
    }
}

impl MockApiBech32 {
    pub fn addr_make(&self, input: &str) -> Addr {
        let digest = Sha256::digest(input).to_vec();
        match encode::<Bech32>(Hrp::parse_unchecked(self.prefix), digest.as_slice()) {
            Ok(address) => Addr::unchecked(address),
            Err(reason) => panic!("Generating address failed with reason: {}", reason),
        }
    }
}
