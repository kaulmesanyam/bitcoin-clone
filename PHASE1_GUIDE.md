# Phase 1: Fundamentals - Learning Guide

## What You'll Learn

### 1. What is a Blockchain?

A blockchain is essentially a **linked list of blocks**, where each block contains:
- Data (transactions, in Bitcoin's case)
- A hash of its own contents
- A hash of the previous block

This creates an **immutable chain**: if you change any block, its hash changes, which breaks the link to the next block.

**Key Insight**: The "chain" is created by each block storing the hash of the previous block.

### 2. Cryptographic Hashing (SHA-256)

**Hash Function**: Takes any input and produces a fixed-size output (256 bits = 64 hex characters for SHA-256).

**Properties**:
- **Deterministic**: Same input → same output
- **One-way**: Can't reverse a hash to get the original data
- **Avalanche effect**: Tiny change in input → completely different hash
- **Collision resistant**: Very hard to find two inputs with same hash

**Why it matters**: 
- Detects any tampering (change data → hash changes)
- Creates the "chain" (previous block hash links blocks)
- Used in mining (finding hashes with specific patterns)

### 3. Block Structure

A block needs:
- **Index**: Position in the chain (0 for first block)
- **Previous Hash**: Hash of the previous block (creates the link)
- **Timestamp**: When the block was created
- **Data**: The actual content (transactions later, simple data now)
- **Hash**: This block's hash (calculated from all above fields)

### 4. Merkle Trees (Concept Only)

**What**: A tree structure where:
- Leaves are transactions
- Each node is the hash of its children
- Root hash represents all transactions

**Why**: Efficiently verify if a transaction is in a block without downloading all transactions.

**For now**: Just understand the concept. We'll implement it in Phase 7.

### 5. Proof of Work (Concept)

**What**: Finding a value (nonce) such that the block's hash meets certain criteria (e.g., starts with N zeros).

**Why**: Makes creating blocks expensive, securing the network.

**For now**: Just understand that mining = finding a special hash.

---

## What to Build

### Step 1: Hash Function
Create a function that takes data and returns SHA-256 hash.

**Python hint**: Use `hashlib.sha256()`

### Step 2: Block Structure
Create a Block class with:
- Fields: index, previous_hash, timestamp, data
- Method to calculate its own hash
- Method to convert to string/dict (for display)

### Step 3: Link Blocks
Create multiple blocks where each block's `previous_hash` is the hash of the previous block.

### Step 4: Test Immutability
- Create a chain of blocks
- Change data in an earlier block
- Show that the hash changes and breaks the chain

---

## Questions to Answer

1. Why does storing the previous block's hash create security?
2. What happens if you change data in block 2 of a 5-block chain?
3. How does SHA-256 ensure tamper detection?

---

## Next Steps

Once you've built:
- ✅ Block structure
- ✅ Hash calculation
- ✅ Linked blocks

You're ready for **Phase 2: Basic Blockchain** where we'll add validation and mining!

