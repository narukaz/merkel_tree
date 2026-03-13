# merkel_tree

A minimal Rust implementation of a Merkle Tree supporting **Merkle root generation, proof creation, and proof verification**.

This project explores the core data structures used in blockchains and distributed systems for efficient data integrity verification.

## Overview

A Merkle Tree is a binary hash tree where:

- Leaf nodes contain the hash of data blocks.
- Internal nodes contain the hash of their child hashes.
- The root hash uniquely represents the entire dataset.

This structure allows verifying that a piece of data exists in a dataset **without revealing the entire dataset**, using a Merkle Proof.

Merkle Trees are widely used in systems such as:

- Bitcoin and Ethereum
- Distributed storage
- Version control systems (Git)
- Peer-to-peer networks

## Features

- SHA-256 based hashing
- Deterministic Merkle root generation
- Merkle proof generation
- Efficient `O(log n)` proof verification
- Handles odd numbers of leaves by duplicating the last node
- Rust-native unit tests for correctness and tamper detection

## How It Works

Given a list of data blocks:

```
A  B  C  D
```

Hashes are computed bottom-up:

```
H(A)  H(B)  H(C)  H(D)

H(H(A) || H(B))     H(H(C) || H(D))

           ROOT
```

To prove that `C` exists in the dataset, a **Merkle proof** contains the sibling hashes along the path to the root.

Verification recomputes the root using only:

- the data hash
- the proof hashes

This allows membership verification in **O(log n)** time.

## Installation

Add the crate to your project:

```
cargo add merkel_tree
```

Or include it manually in `Cargo.toml`:

```toml
[dependencies]
merkel_tree = { path = "./merkel_tree" }
```

## Example

```rust
use merkel_tree::MerkelTree;

let data = vec![
    b"A".as_ref(),
    b"B".as_ref(),
    b"C".as_ref(),
];

let tree = MerkelTree::from_data(&data);

let proof = tree.proof(1).unwrap();

let root = tree.root.unwrap().hash;

assert!(MerkelTree::verify(root, data[1], &proof));
```

## Testing

The project includes unit tests written using Rust’s built-in testing framework.

Run tests with:

```
cargo test
```

Tests cover:

- correct root generation
- proof creation
- verification logic
- tamper detection
- edge cases such as odd leaf counts

## Project Structure

```
src/
 ├─ lib.rs
 ├─ merkle_tree.rs
 └─ proof.rs

tests/
 └─ integration tests
```

## Learning Goals

This project was built to explore:

- Cryptographic hash structures
- Data integrity verification
- Blockchain infrastructure primitives
- Rust systems programming

## Future Work

Possible improvements include:

- memory-efficient tree traversal
- persistent tree storage
- parallel hashing
- benchmarking and performance optimizations

## License

MIT