# Bitcoin Clone - Learning Roadmap

## Overview

This roadmap will guide you through building a Bitcoin-like blockchain from scratch, learning everything from fundamentals to advanced implementation.

**Bitcoin Core Language**: Bitcoin Core (reference implementation) is written in **C++** for performance-critical operations, with Python used for scripting and testing.

---

## Phase 1: Fundamentals (Week 1-2)
**Goal**: Understand core concepts

### What to Learn:
- What is a blockchain? (linked list of blocks)
- Cryptographic hashing (SHA-256)
- Merkle trees (concept)
- Proof of Work vs Proof of Stake
- Public-key cryptography basics

### What to Build:
- Simple block structure (index, hash, previous_hash, data, timestamp)
- Hash calculation function
- Link blocks together

---

## Phase 2: Basic Blockchain (Week 2-3)
**Goal**: Working chain with validation

### What to Learn:
- Genesis block concept
- Chain validation
- Tamper detection
- Difficulty adjustment basics

### What to Build:
- Blockchain class (add blocks, validate chain)
- Genesis block creation
- Chain integrity checks
- Simple proof of work (find hash with N leading zeros)

---

## Phase 3: Transactions (Week 3-4)
**Goal**: Value transfer mechanism

### What to Learn:
- Transaction structure (inputs/outputs)
- Transaction IDs
- Transaction validation
- Double-spend problem

### What to Build:
- Transaction class (inputs, outputs, amounts)
- Transaction signing (start with simple validation)
- Transaction verification
- Store transactions in blocks

---

## Phase 4: Cryptography & Wallets (Week 4-5)
**Goal**: Secure transactions

### What to Learn:
- ECDSA (Elliptic Curve Digital Signature Algorithm)
- Public/private key pairs
- Address generation (Base58 encoding)
- Digital signatures

### What to Build:
- Key pair generation
- Transaction signing with private key
- Signature verification with public key
- Address generation from public key
- Wallet class (create, sign transactions)

---

## Phase 5: UTXO Model (Week 5-6)
**Goal**: Bitcoin's accounting system

### What to Learn:
- UTXO (Unspent Transaction Output) concept
- How Bitcoin tracks balances
- Transaction inputs reference UTXOs
- Change outputs

### What to Build:
- UTXO tracking system
- Balance calculation
- Transaction input/output matching
- UTXO validation

---

## Phase 6: Mining & Consensus (Week 6-7)
**Goal**: Block creation and network agreement

### What to Learn:
- Mining process in detail
- Block rewards
- Coinbase transactions
- Difficulty adjustment algorithms
- Longest chain rule

### What to Build:
- Mining loop
- Block reward mechanism
- Difficulty adjustment
- Chain selection (longest valid chain)

---

## Phase 7: Merkle Trees (Week 7-8)
**Goal**: Efficient transaction verification

### What to Learn:
- Merkle tree structure
- Merkle root calculation
- SPV (Simplified Payment Verification)
- Why Merkle trees matter

### What to Build:
- Merkle tree implementation
- Merkle root in block header
- Transaction inclusion proofs

---

## Phase 8: Network Layer (Week 8-9)
**Goal**: Distributed system basics

### What to Learn:
- P2P networking
- Node discovery
- Block propagation
- Transaction propagation
- Network protocols

### What to Build:
- Simple node class
- Block broadcasting (simulation)
- Transaction broadcasting
- Chain synchronization

---

## Phase 9: Advanced Features (Week 9-10)
**Goal**: Production-like features

### What to Learn:
- Persistence (database)
- Mempool (unconfirmed transactions)
- Fee calculation
- Block size limits
- Script interpreter basics

### What to Build:
- Database integration (SQLite/JSON files)
- Mempool management
- Transaction fees
- Block size validation

---

## Phase 10: Testing & Optimization (Week 10+)
**Goal**: Robust, tested system

### What to Learn:
- Unit testing
- Integration testing
- Performance optimization
- Security best practices

### What to Build:
- Comprehensive test suite
- Performance benchmarks
- CLI interface
- Documentation

---

## Recommended Learning Resources

1. **Bitcoin Whitepaper**: https://bitcoin.org/bitcoin.pdf
2. **Mastering Bitcoin** (book by Andreas Antonopoulos)
3. **Bitcoin Developer Guide**: https://bitcoin.org/en/developer-guide
4. **Bitcoin Core Source Code**: https://github.com/bitcoin/bitcoin

---

## Language Choice for Your Project

- **Python**: Easier to learn, faster to prototype, good for understanding concepts ✅ **Recommended for learning**
- **C++**: Closer to Bitcoin Core, better performance, more complex
- **JavaScript/TypeScript**: Good for web-based demos
- **Go/Rust**: Modern alternatives with good performance

**Recommendation**: Start with Python to learn concepts, then optionally rewrite in C++ if you want performance.

---

## Key Milestones

- [ ] ✅ Working blockchain with mining
- [ ] ✅ Transactions with signatures
- [ ] ✅ Wallet that can send/receive
- [ ] ✅ UTXO tracking
- [ ] ✅ Multiple nodes (simulated)
- [ ] ✅ Persistent storage
- [ ] ✅ Full test suite

---

## Current Phase: Phase 1 - Fundamentals

**Status**: Starting now!

