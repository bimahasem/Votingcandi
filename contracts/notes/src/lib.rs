#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Candidate {
    id: u64,
    name: String,
    vote_count: u32,
}

const CANDIDATE_DATA: Symbol = symbol_short!("CANDDATA");

#[contract]
pub struct CandidateVoteContract;

#[contractimpl]
impl CandidateVoteContract {
    pub fn get_candidates(env: Env) -> Vec<Candidate> {
        env.storage().instance().get(&CANDIDATE_DATA).unwrap_or(Vec::new(&env))
    }

    pub fn add_candidate(env: Env, name: String) -> String {
        let mut candidates: Vec<Candidate> = env.storage().instance().get(&CANDIDATE_DATA).unwrap_or(Vec::new(&env));

        let candidate = Candidate {
            id: env.prng().gen::<u64>(),
            name,
            vote_count: 0,
        };

        candidates.push_back(candidate);
        env.storage().instance().set(&CANDIDATE_DATA, &candidates);

        String::from_str(&env, "Candidate added successfully")
    }

    pub fn vote_candidate(env: Env, id: u64) -> String {
        let mut candidates: Vec<Candidate> = env.storage().instance().get(&CANDIDATE_DATA).unwrap_or(Vec::new(&env));

        for i in 0..candidates.len() {
            let mut candidate = candidates.get(i).unwrap();
            if candidate.id == id {
                candidate.vote_count += 1;
                candidates.set(i, candidate);
                env.storage().instance().set(&CANDIDATE_DATA, &candidates);
                return String::from_str(&env, "Vote submitted successfully");
            }
        }

        String::from_str(&env, "Candidate not found")
    }
}