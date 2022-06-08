// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

#![forbid(unsafe_code)]
#![allow(clippy::too_many_arguments)]

#[macro_use]
extern crate lazy_static;

pub use snarkvm_console_network_environment as environment;
pub use snarkvm_console_network_environment::*;

pub mod testnet3;
pub use testnet3::*;

pub mod prelude {
    pub use crate::environment::prelude::*;

    pub use crate::Network;
}

use snarkvm_console_algorithms::{Poseidon2, Poseidon4, BHP1024, BHP512};
use snarkvm_console_collections::merkle_tree::MerkleTree;
use snarkvm_curves::{AffineCurve, MontgomeryParameters, ProjectiveCurve, TwistedEdwardsParameters};
use snarkvm_fields::traits::*;
use snarkvm_utilities::BigInteger;

use anyhow::Result;
use core::{fmt, hash};

pub trait Network: Copy + Clone + fmt::Debug + Eq + PartialEq + hash::Hash {
    type Affine: AffineCurve<
        Projective = Self::Projective,
        BaseField = Self::Field,
        ScalarField = Self::Scalar,
        Coordinates = (Self::Field, Self::Field),
    >;
    type AffineParameters: MontgomeryParameters<BaseField = Self::Field>
        + TwistedEdwardsParameters<BaseField = Self::Field>;
    type Projective: ProjectiveCurve<Affine = Self::Affine, BaseField = Self::Field, ScalarField = Self::Scalar>;
    type Field: PrimeField<BigInteger = Self::BigInteger> + SquareRootField + Copy;
    type Scalar: PrimeField<BigInteger = Self::BigInteger> + Copy;
    type BigInteger: BigInteger;

    /// The network ID.
    const ID: u16;

    /// The maximum recursive depth of a value and/or entry.
    /// Note: This value must be strictly less than u8::MAX.
    const MAX_DATA_DEPTH: usize = 32;
    /// The maximum number of values and/or entries in data.
    const MAX_DATA_ENTRIES: usize = 32;

    /// The maximum number of inputs per transition.
    const MAX_INPUTS: usize = 8;
    /// The maximum number of outputs per transition.
    const MAX_OUTPUTS: usize = 8;
    /// The maximum number of transitions per transaction.
    const MAX_TRANSITIONS: usize = 16;
    /// The maximum number of transactions per block.
    const MAX_TRANSACTIONS: usize = 65536;

    /// The maximum number of bits in data (must not exceed u16::MAX).
    const MAX_DATA_SIZE_IN_FIELDS: u32 = (128 * 1024 * 8) / <Self::Field as PrimeField>::Parameters::CAPACITY;

    /// The maximum number of bytes allowed in a string.
    const MAX_STRING_BYTES: u32 = u8::MAX as u32;

    /// TODO (howardwu): Refactor Fp256 and Fp384 and deprecate this method.
    /// A helper method to recover a field element from **little-endian** bits.
    fn field_from_bits_le(bits: &[bool]) -> Result<Self::Field>;

    /// TODO (howardwu): Refactor Fp256 and Fp384 and deprecate this method.
    /// A helper method to recover a field element from **big-endian** bits.
    fn field_from_bits_be(bits: &[bool]) -> Result<Self::Field>;

    /// Returns the balance commitment domain as a constant field element.
    fn bcm_domain() -> Self::Field;

    /// Returns the encryption domain as a constant field element.
    fn encryption_domain() -> Self::Field;

    /// Returns the MAC domain as a constant field element.
    fn mac_domain() -> Self::Field;

    /// Returns the randomizer domain as a constant field element.
    fn randomizer_domain() -> Self::Field;

    /// Returns the balance commitment randomizer domain as a constant field element.
    fn r_bcm_domain() -> Self::Field;

    /// Returns the serial number domain as a constant field element.
    fn serial_number_domain() -> Self::Field;

    /// Returns the powers of G.
    fn g_powers() -> &'static Vec<Self::Projective>;

    /// Returns the scalar multiplication on the group bases.
    fn g_scalar_multiply(scalar: &Self::Scalar) -> Self::Projective;

    /// Returns a BHP commitment with an input hasher of 256-bits.
    fn commit_bhp256(input: &[bool], randomizer: &Self::Scalar) -> Result<Self::Field>;

    /// Returns a BHP commitment with an input hasher of 512-bits.
    fn commit_bhp512(input: &[bool], randomizer: &Self::Scalar) -> Result<Self::Field>;

    /// Returns a BHP commitment with an input hasher of 768-bits.
    fn commit_bhp768(input: &[bool], randomizer: &Self::Scalar) -> Result<Self::Field>;

    /// Returns a BHP commitment with an input hasher of 1024-bits.
    fn commit_bhp1024(input: &[bool], randomizer: &Self::Scalar) -> Result<Self::Field>;

    /// Returns a Pedersen commitment for the given (up to) 64-bit input and randomizer.
    fn commit_ped64(input: &[bool], randomizer: &Self::Scalar) -> Result<Self::Affine>;

    /// Returns a Pedersen commitment for the given (up to) 128-bit input and randomizer.
    fn commit_ped128(input: &[bool], randomizer: &Self::Scalar) -> Result<Self::Affine>;

    /// Returns the BHP hash with an input hasher of 256-bits.
    fn hash_bhp256(input: &[bool]) -> Result<Self::Field>;

    /// Returns the BHP hash with an input hasher of 512-bits.
    fn hash_bhp512(input: &[bool]) -> Result<Self::Field>;

    /// Returns the BHP hash with an input hasher of 768-bits.
    fn hash_bhp768(input: &[bool]) -> Result<Self::Field>;

    /// Returns the BHP hash with an input hasher of 1024-bits.
    fn hash_bhp1024(input: &[bool]) -> Result<Self::Field>;

    /// Returns the Pedersen hash for a given (up to) 64-bit input.
    fn hash_ped64(input: &[bool]) -> Result<Self::Field>;

    /// Returns the Pedersen hash for a given (up to) 128-bit input.
    fn hash_ped128(input: &[bool]) -> Result<Self::Field>;

    /// Returns the Poseidon hash with an input rate of 2.
    fn hash_psd2(input: &[Self::Field]) -> Result<Self::Field>;

    /// Returns the Poseidon hash with an input rate of 4.
    fn hash_psd4(input: &[Self::Field]) -> Result<Self::Field>;

    /// Returns the Poseidon hash with an input rate of 8.
    fn hash_psd8(input: &[Self::Field]) -> Result<Self::Field>;

    /// Returns the extended Poseidon hash with an input rate of 2.
    fn hash_many_psd2(input: &[Self::Field], num_outputs: u16) -> Vec<Self::Field>;

    /// Returns the extended Poseidon hash with an input rate of 4.
    fn hash_many_psd4(input: &[Self::Field], num_outputs: u16) -> Vec<Self::Field>;

    /// Returns the extended Poseidon hash with an input rate of 8.
    fn hash_many_psd8(input: &[Self::Field], num_outputs: u16) -> Vec<Self::Field>;

    /// Returns the Poseidon hash with an input rate of 2 on the affine curve.
    fn hash_to_group_psd2(input: &[Self::Field]) -> Result<Self::Affine>;

    /// Returns the Poseidon hash with an input rate of 4 on the affine curve.
    fn hash_to_group_psd4(input: &[Self::Field]) -> Result<Self::Affine>;

    /// Returns the Poseidon hash with an input rate of 8 on the affine curve.
    fn hash_to_group_psd8(input: &[Self::Field]) -> Result<Self::Affine>;

    /// Returns the Poseidon hash with an input rate of 2 on the scalar field.
    fn hash_to_scalar_psd2(input: &[Self::Field]) -> Result<Self::Scalar>;

    /// Returns the Poseidon hash with an input rate of 4 on the scalar field.
    fn hash_to_scalar_psd4(input: &[Self::Field]) -> Result<Self::Scalar>;

    /// Returns the Poseidon hash with an input rate of 8 on the scalar field.
    fn hash_to_scalar_psd8(input: &[Self::Field]) -> Result<Self::Scalar>;

    /// Returns a Merkle tree with a BHP leaf hasher of 1024-bits and a BHP path hasher of 512-bits.
    #[allow(clippy::type_complexity)]
    fn merkle_tree_bhp<const DEPTH: u8>(
        leaves: &[Vec<bool>],
    ) -> Result<MerkleTree<BHP1024<Self::Affine>, BHP512<Self::Affine>, DEPTH>>;

    /// Returns a Merkle tree with a Poseidon leaf hasher with input rate of 4 and a Poseidon path hasher with input rate of 2.
    #[allow(clippy::type_complexity)]
    fn merkle_tree_psd<const DEPTH: u8>(
        leaves: &[Vec<Self::Field>],
    ) -> Result<MerkleTree<Poseidon4<Self::Field>, Poseidon2<Self::Field>, DEPTH>>;

    /// Returns the Poseidon PRF with an input rate of 2.
    fn prf_psd2(seed: &Self::Field, input: &[Self::Field]) -> Result<Self::Field>;

    /// Returns the Poseidon PRF with an input rate of 4.
    fn prf_psd4(seed: &Self::Field, input: &[Self::Field]) -> Result<Self::Field>;

    /// Returns the Poseidon PRF with an input rate of 8.
    fn prf_psd8(seed: &Self::Field, input: &[Self::Field]) -> Result<Self::Field>;

    /// Returns the Poseidon PRF on the **scalar** field with an input rate of 2.
    fn prf_psd2s(seed: &Self::Scalar, input: &[Self::Scalar]) -> Result<Self::Scalar>;

    /// Halts the program from further synthesis, evaluation, and execution in the current environment.
    fn halt<S: Into<String>, T>(message: S) -> T {
        panic!("{}", message.into())
    }
}
