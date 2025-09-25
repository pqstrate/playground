use p3_blake3::Blake3;
use p3_challenger::{DuplexChallenger, HashChallenger, SerializingChallenger64};
use p3_commit::ExtensionMmcs;
use p3_dft::Radix2DitParallel;
use p3_field::extension::BinomialExtensionField;
use p3_fri::TwoAdicFriPcs;
use p3_goldilocks::{Goldilocks, Poseidon2Goldilocks};
use p3_keccak::{Keccak256Hash, KeccakF};
use p3_merkle_tree::MerkleTreeMmcs;
use p3_symmetric::{
    CompressionFunctionFromHasher, PaddingFreeSponge, SerializingHasher, TruncatedPermutation,
};
use p3_uni_stark::StarkConfig;

pub type Val = Goldilocks;
pub type Challenge = BinomialExtensionField<Val, 2>;

// Keccak-based type definitions
pub type KeccakByteHash = Keccak256Hash;
pub type KeccakU64Hash = PaddingFreeSponge<KeccakF, 25, 17, 4>;
pub type KeccakFieldHash = SerializingHasher<KeccakU64Hash>;
pub type KeccakCompress = CompressionFunctionFromHasher<KeccakU64Hash, 2, 4>;
pub type KeccakValMmcs = MerkleTreeMmcs<
    [Val; p3_keccak::VECTOR_LEN],
    [u64; p3_keccak::VECTOR_LEN],
    KeccakFieldHash,
    KeccakCompress,
    4,
>;
pub type KeccakChallengeMmcs = ExtensionMmcs<Val, Challenge, KeccakValMmcs>;
pub type KeccakChallenger = SerializingChallenger64<Val, HashChallenger<u8, KeccakByteHash, 32>>;
pub type KeccakPcs = TwoAdicFriPcs<Val, Radix2DitParallel<Val>, KeccakValMmcs, KeccakChallengeMmcs>;
pub type KeccakConfig = StarkConfig<KeccakPcs, Challenge, KeccakChallenger>;

// Poseidon2-based type definitions
pub type Poseidon2Perm = Poseidon2Goldilocks<16>;
pub type Poseidon2Hash = PaddingFreeSponge<Poseidon2Perm, 16, 8, 8>;
pub type Poseidon2Compress = TruncatedPermutation<Poseidon2Perm, 2, 8, 16>;
pub type Poseidon2ValMmcs = MerkleTreeMmcs<
    <Val as p3_field::Field>::Packing,
    <Val as p3_field::Field>::Packing,
    Poseidon2Hash,
    Poseidon2Compress,
    8,
>;
pub type Poseidon2ChallengeMmcs = ExtensionMmcs<Val, Challenge, Poseidon2ValMmcs>;
pub type Poseidon2Challenger = DuplexChallenger<Val, Poseidon2Perm, 16, 8>;
pub type Poseidon2Pcs =
    TwoAdicFriPcs<Val, Radix2DitParallel<Val>, Poseidon2ValMmcs, Poseidon2ChallengeMmcs>;
pub type Poseidon2Config = StarkConfig<Poseidon2Pcs, Challenge, Poseidon2Challenger>;

// Blake3-based type definitions (following merkle-tree benchmark pattern)
pub type Blake3ByteHash = Blake3;
pub type Blake3FieldHash = SerializingHasher<Blake3>;
pub type Blake3Compress = CompressionFunctionFromHasher<Blake3, 2, 32>;
pub type Blake3ValMmcs = MerkleTreeMmcs<Val, u8, Blake3FieldHash, Blake3Compress, 32>;
pub type Blake3ChallengeMmcs = ExtensionMmcs<Val, Challenge, Blake3ValMmcs>;
pub type Blake3Challenger = SerializingChallenger64<Val, HashChallenger<u8, Blake3ByteHash, 32>>;
pub type Blake3Pcs = TwoAdicFriPcs<Val, Radix2DitParallel<Val>, Blake3ValMmcs, Blake3ChallengeMmcs>;
pub type Blake3Config = StarkConfig<Blake3Pcs, Challenge, Blake3Challenger>;
