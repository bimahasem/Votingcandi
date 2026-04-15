# Soroban Candidate Vote

## Description
Soroban Candidate Vote is a simple decentralized voting smart contract built on Stellar Soroban. This application allows users to add candidates, view the list of candidates, and vote for a candidate on the blockchain.

## Features
- Add a new candidate
- View all candidates
- Vote for a candidate
- Store voting data on Stellar Soroban smart contract

## Smart Contract Functions
- `add_candidate(name)` → Adds a new candidate
- `get_candidates()` → Returns all candidates
- `vote_candidate(id)` → Votes for a candidate by ID

## Testnet Smart Contract ID
`CDQN2CNLBWYB26PRVONQE7ONVVPOZCYVVKN7OENWWCDVUC2GYYOLLYAS

## Testnet Screenshot
Add your testnet screenshot below.

![Testnet Screenshot](screenshots/testnet-contract.png)

## Example Usage

### Add Candidate
```bash
stellar contract invoke --id YOUR_CONTRACT_ID --source-account YOUR_ACCOUNT --network testnet -- add_candidate --name "Alice"
