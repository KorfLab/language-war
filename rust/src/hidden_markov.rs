use crate::collections::{IndexedVec2, Vec2};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HiddenMarkovParseError<S, O> {
    #[error("state {0:?} is duplicated")]
    DuplicateState(S),
    #[error("observation {0:?} is duplicated")]
    DuplicateObservation(O),
    #[error("missing initial probability of state {0:?}")]
    InitialProbabilityMissing(S),
    #[error("missing emission probabilities of state {0:?}")]
    EmissionProbabilitiesMissing(S),
    #[error("observation {0:?} not found in observation list")]
    ObservationNotFound(O),
}

pub struct HiddenMarkov<O, S>
where
    O: Eq + Hash,
    S: Eq + Hash,
{
    states: Vec<S>,
    state_indices: HashMap<S, usize>,
    observations: Vec<O>,
    obs_indices: HashMap<O, usize>,
    initial_probabilities: Vec<f64>,
    transition_matrix: Vec2<f64>,
    emission_matrix: Vec2<f64>,
}

impl<O, S> HiddenMarkov<O, S>
where
    O: Eq + Hash + Clone + Debug,
    S: Eq + Hash + Clone + Debug,
{
    pub fn new(
        states: Vec<S>,
        observations: Vec<O>,
        initial_probabilities: HashMap<S, f64>,
        transition_matrix: HashMap<S, HashMap<S, f64>>,
        emission_matrix: HashMap<S, HashMap<O, f64>>,
    ) -> Result<Self, HiddenMarkovParseError<S, O>> {
        let mut state_indices = HashMap::new();
        for (idx, state) in states.iter().enumerate() {
            if state_indices.insert(state.clone(), idx).is_some() {
                return Err(HiddenMarkovParseError::DuplicateState(state.clone()));
            }
        }

        let mut obs_indices = HashMap::new();
        for (idx, obs) in observations.iter().enumerate() {
            if obs_indices.insert(obs.clone(), idx).is_some() {
                return Err(HiddenMarkovParseError::DuplicateObservation(obs.clone()));
            }
        }

        for state in &states {
            if !initial_probabilities.contains_key(state) {
                return Err(HiddenMarkovParseError::InitialProbabilityMissing(
                    state.clone(),
                ));
            }
            if !emission_matrix.contains_key(state) {
                return Err(HiddenMarkovParseError::EmissionProbabilitiesMissing(
                    state.clone(),
                ));
            }
        }

        let n_states = states.len();
        let n_observations = observations.len();
        let mut initial_data = Vec::with_capacity(n_states);
        let mut transition_data = Vec::with_capacity(n_states * n_states);
        for from in &states {
            initial_data.push(
                *initial_probabilities.get(from).ok_or_else(|| {
                    HiddenMarkovParseError::InitialProbabilityMissing(from.clone())
                })?,
            );
            let row = transition_matrix.get(from);
            for to in &states {
                transition_data.push(row.and_then(|r| r.get(to)).copied().unwrap_or(0.0));
            }
        }

        let mut emission_data = vec![0.0f64; n_states * n_observations];
        for (from_idx, from_state) in states.iter().enumerate() {
            let row = emission_matrix.get(from_state).ok_or_else(|| {
                HiddenMarkovParseError::EmissionProbabilitiesMissing(from_state.clone())
            })?;
            for (obs, &prob) in row {
                let obs_idx = *obs_indices
                    .get(obs)
                    .ok_or_else(|| HiddenMarkovParseError::ObservationNotFound(obs.clone()))?;
                emission_data[from_idx * n_observations + obs_idx] = prob;
            }
        }

        Ok(Self {
            states,
            state_indices,
            observations,
            obs_indices,
            initial_probabilities: initial_data,
            transition_matrix: Vec2::from_flat(transition_data, n_states, n_states),
            emission_matrix: Vec2::from_flat(emission_data, n_states, n_observations),
        })
    }
}

impl<O, S> HiddenMarkov<O, S>
where
    O: Eq + Hash + Clone,
    S: Eq + Hash + Clone,
{
    pub fn states(&self) -> &[S] {
        &self.states
    }

    pub fn observations(&self) -> &[O] {
        &self.observations
    }

    pub fn state_index(&self, state: &S) -> Option<usize> {
        self.state_indices.get(state).copied()
    }

    pub fn observation_index(&self, obs: &O) -> Option<usize> {
        self.obs_indices.get(obs).copied()
    }

    pub fn state_at(&self, idx: usize) -> Option<&S> {
        self.states.get(idx)
    }

    pub fn initial_probabilities(&self) -> &[f64] {
        &self.initial_probabilities
    }

    pub fn transition_matrix(&self) -> &Vec2<f64> {
        &self.transition_matrix
    }

    pub fn emission_matrix(&self) -> &Vec2<f64> {
        &self.emission_matrix
    }

    pub fn transition(&self) -> IndexedVec2<'_, S, S, f64> {
        IndexedVec2::new(
            &self.transition_matrix,
            &self.state_indices,
            &self.state_indices,
        )
    }

    pub fn emissions(&self) -> IndexedVec2<'_, S, O, f64> {
        IndexedVec2::new(
            &self.emission_matrix,
            &self.state_indices,
            &self.obs_indices,
        )
    }

    pub fn initial_probability(&self, state: &S) -> Option<f64> {
        self.state_indices
            .get(state)
            .map(|&i| self.initial_probabilities[i])
    }

    pub fn transition_probability(&self, from: &S, to: &S) -> Option<f64> {
        self.transition().get(from, to).copied()
    }

    pub fn emission_probability(&self, state: &S, observation: &O) -> f64 {
        self.state_indices
            .get(state)
            .and_then(|&si| {
                self.obs_indices
                    .get(observation)
                    .map(|&oi| *self.emission_matrix.get(si, oi))
            })
            .unwrap_or(0.0)
    }

    pub fn calculate(&self, observations: Vec<O>) -> Vec<S> {
        let n_states = self.states.len();
        let n_obs = observations.len();
        if n_obs == 0 {
            return vec![];
        }

        let mut dp = Vec2::with_default(n_states, n_obs, f64::NEG_INFINITY);
        let mut trace = Vec2::with_default(n_states, n_obs, 0usize);

        for i in 0..n_states {
            let state = &self.states[i];
            let init_prob = self.initial_probability(state).unwrap_or(0.0);
            if init_prob == 0.0 {
                continue;
            }
            let emission_prob = self.emission_probability(state, &observations[0]);
            if emission_prob == 0.0 {
                continue;
            }
            dp.set(i, 0, init_prob.ln() + emission_prob.ln());
        }

        for (i, observation) in observations.iter().enumerate().skip(1) {
            for j in 0..n_states {
                let state = &self.states[j];
                let emission_prob = self.emission_probability(state, observation);
                if emission_prob == 0.0 {
                    continue;
                }
                let log_emission = emission_prob.ln();

                let mut max_prob = f64::NEG_INFINITY;
                let mut best_state = 0;

                for prob_idx in 0..n_states {
                    let trans_prob = self.transition_matrix.get(prob_idx, j);
                    if *trans_prob == 0.0 {
                        continue;
                    }
                    let prob = *dp.get(prob_idx, i - 1) + (*trans_prob).ln();

                    if prob > max_prob {
                        max_prob = prob;
                        best_state = prob_idx;
                    }
                }
                dp.set(j, i, max_prob + log_emission);
                trace.set(j, i, best_state);
            }
        }

        let mut best_path: Vec<usize> = vec![0; n_obs];
        best_path[n_obs - 1] = (0..n_states)
            .max_by(|&a, &b| dp.get(a, n_obs - 1).partial_cmp(dp.get(b, n_obs - 1)).unwrap())
            .unwrap();

        for i in (0..n_obs - 1).rev() {
            best_path[i] = *trace.get(best_path[i + 1], i + 1);
        }

        best_path.iter().map(|&idx| self.states[idx].clone()).collect()
    }
}
