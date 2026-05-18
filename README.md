# Stellar Voting DApp

A simple decentralized voting application built on **Stellar Soroban**.  
This project allows users to vote for candidates using their Stellar wallet address as their identity.

## Smart Contract

**Network:** Stellar Testnet  
**Contract ID:**

```text
CC7VUYKPHXMVZRO5MTJDI57P57OQTEKLXO2ETVOHVHRMWPUE47U32OPK
```

## Project Description

Stellar Voting DApp is a simple Web3 voting application where an admin can create candidates and users can vote for one candidate.  
Each wallet address can only vote once, making the voting process more transparent and preventing duplicate votes from the same wallet.

The voting data is stored on-chain using a Soroban smart contract, so the result can be checked publicly on the Stellar testnet.

## Main Features

- Initialize the voting contract with an admin address
- Add candidates to the voting list
- Vote for a candidate using a Stellar wallet
- Prevent the same wallet from voting more than once
- View all candidates and their vote counts
- Check whether a wallet has already voted
- Open or close the voting process by admin
- View the current voting status
- View the admin address

## Contract Functions

### `initialize(admin: Address)`

Initializes the contract and sets the admin address.

### `add_candidate(admin: Address, name: String)`

Adds a new candidate to the voting list.  
Only the admin can call this function.

### `vote(voter: Address, candidate_id: u32)`

Allows a voter to vote for a candidate.  
Each voter address can only vote once.

### `get_candidates()`

Returns the list of all candidates and their vote counts.

### `get_candidate(candidate_id: u32)`

Returns a specific candidate by candidate ID.

### `has_voted(voter: Address)`

Checks whether a wallet address has already voted.

### `set_voting_open(admin: Address, is_open: bool)`

Opens or closes the voting process.  
Only the admin can call this function.

### `is_voting_open()`

Returns the current voting status.

### `get_admin()`

Returns the admin address of the contract.

## How It Works

1. The admin initializes the contract.
2. The admin adds candidates.
3. A user connects their Stellar wallet.
4. The user selects a candidate.
5. The user submits a vote transaction.
6. The smart contract checks whether:
   - Voting is still open
   - The voter has not voted before
   - The selected candidate exists
7. The vote count is updated on-chain.

## Example Usage

### Initialize Contract

```text
initialize(admin)
```

### Add Candidates

```text
add_candidate(admin, "Alice")
add_candidate(admin, "Bob")
add_candidate(admin, "Charlie")
```

### Vote

```text
vote(voter, 0)
```

In this example, `0` is the candidate ID for the first candidate.

### Get Voting Result

```text
get_candidates()
```

## Technology Used

- Stellar Soroban
- Rust
- Soroban SDK
- Stellar Testnet
- Stellar Lab
- Soroban Studio

## Folder Structure

```text
contracts/
└── hello-world/
    ├── src/
    │   ├── lib.rs
    │   └── test.rs
    └── Cargo.toml
```

## Smart Contract Storage

The contract stores several types of data:

- `Admin` - stores the admin wallet address
- `VotingOpen` - stores whether voting is open or closed
- `Candidates` - stores all candidates and their vote counts
- `Voted(Address)` - stores whether a wallet has already voted

## Security Notes

- Only the admin can add candidates.
- Only the admin can open or close voting.
- A voter must authorize the transaction using their wallet.
- A wallet address can only vote once.
- Votes are stored on-chain and can be verified publicly.

## Deployment

The contract has been deployed to Stellar Testnet.

**Contract ID:**

```text
CC7VUYKPHXMVZRO5MTJDI57P57OQTEKLXO2ETVOHVHRMWPUE47U32OPK
```

You can inspect and invoke the contract using Stellar Lab on the testnet network.

## Future Improvements

- Build a frontend interface for easier voting
- Add wallet connection support
- Display live voting results
- Add a candidate image or description
- Add event name and voting deadline
- Add better admin dashboard

## Author

Created as a simple Web3 project submission using Stellar Soroban.
