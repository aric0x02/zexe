use super::CommitmentScheme;
use crate::Error;
use blake2::Blake2s256 as b2s;
use digest::Digest;
use rand::Rng;

pub struct Blake2sCommitment;

#[cfg(feature = "r1cs")]
pub mod constraints;

impl CommitmentScheme for Blake2sCommitment {
    type Parameters = ();
    type Randomness = [u8; 32];
    type Output = [u8; 32];

    fn setup<R: Rng>(_: &mut R) -> Result<Self::Parameters, Error> {
        Ok(())
    }

    fn commit(
        _: &Self::Parameters,
        input: &[u8],
        randomness: &Self::Randomness,
    ) -> Result<Self::Output, Error> {
        let mut h = b2s::new();
        h.update(input);
        h.update(randomness.as_ref());
        let result = h.finalize();
        Ok(result[..].try_into().unwrap())
    }
}
