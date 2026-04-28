use clap::Parser;
use korflab_language_war::hidden_markov::HiddenMarkov;
use rand::prelude::*;
use rand::rngs::Xoshiro256PlusPlus;
use std::collections::HashMap;

#[derive(Debug, Parser)]
struct Args {
    #[arg(default_value_t = 20)]
    num_observations: usize,
}

fn main() {
    let args = Args::parse();

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    enum State {
        Fair,
        Loaded,
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    enum Observation {
        One,
        Two,
        Three,
        Four,
        Five,
        Six,
    }

    let states = vec![State::Fair, State::Loaded];
    let observation_space = vec![
        Observation::One,
        Observation::Two,
        Observation::Three,
        Observation::Four,
        Observation::Five,
        Observation::Six,
    ];

    let mut initial_probabilities = HashMap::new();
    initial_probabilities.insert(State::Fair, 0.5);
    initial_probabilities.insert(State::Loaded, 0.5);

    let mut transition_matrix = HashMap::new();
    transition_matrix.insert(
        State::Fair,
        HashMap::from([(State::Fair, 0.8), (State::Loaded, 0.2)]),
    );
    transition_matrix.insert(
        State::Loaded,
        HashMap::from([(State::Fair, 0.5), (State::Loaded, 0.5)]),
    );

    let mut emission_matrix = HashMap::new();
    emission_matrix.insert(
        State::Fair,
        HashMap::from([
            (Observation::One, 1.0 / 6.0),
            (Observation::Two, 1.0 / 6.0),
            (Observation::Three, 1.0 / 6.0),
            (Observation::Four, 1.0 / 6.0),
            (Observation::Five, 1.0 / 6.0),
            (Observation::Six, 1.0 / 6.0),
        ]),
    );
    emission_matrix.insert(
        State::Loaded,
        HashMap::from([
            (Observation::One, 0.1),
            (Observation::Two, 0.1),
            (Observation::Three, 0.1),
            (Observation::Four, 0.1),
            (Observation::Five, 0.1),
            (Observation::Six, 0.5),
        ]),
    );

    let hmm = HiddenMarkov::new(states.clone(), observation_space, initial_probabilities, transition_matrix, emission_matrix)
        .unwrap();

    let mut rng: Xoshiro256PlusPlus = rand::make_rng();
    let mut observations: Vec<Observation> = Vec::new();
    let mut true_states: Vec<State> = Vec::new();

    let mut current_state = if rng.random::<f64>() < 0.5 {
        State::Fair
    } else {
        State::Loaded
    };

    for _ in 0..args.num_observations {
        true_states.push(current_state.clone());

        let emissions = if current_state == State::Fair {
            &[1.0 / 6.0; 6]
        } else {
            &[0.1, 0.1, 0.1, 0.1, 0.1, 0.5]
        };

        let roll = rng.random::<f64>();
        let mut cumulative = 0.0;
        let mut obs = Observation::One;
        for (i, &p) in emissions.iter().enumerate() {
            cumulative += p;
            if roll < cumulative {
                obs = match i {
                    0 => Observation::One,
                    1 => Observation::Two,
                    2 => Observation::Three,
                    3 => Observation::Four,
                    4 => Observation::Five,
                    _ => Observation::Six,
                };
                break;
            }
        }
        observations.push(obs);

        let next_state_prob = if current_state == State::Fair {
            0.1
        } else {
            0.2
        };
        current_state = if rng.random::<f64>() < next_state_prob {
            if current_state == State::Fair { State::Loaded } else { State::Fair }
        } else {
            current_state
        };
    }

    let best_path = hmm.calculate(observations.clone());

    let obs_to_str = |o: &Observation| match o {
        Observation::One => "1",
        Observation::Two => "2",
        Observation::Three => "3",
        Observation::Four => "4",
        Observation::Five => "5",
        Observation::Six => "6",
    };
    let state_to_str = |s: &State| match s {
        State::Fair => "F",
        State::Loaded => "L",
    };

    println!("Observations:  {}", observations.iter().map(obs_to_str).collect::<Vec<_>>().join(" "));
    println!("True states:   {}", true_states.iter().map(state_to_str).collect::<Vec<_>>().join(" "));
    println!("Viterbi path:  {}", best_path.iter().map(state_to_str).collect::<Vec<_>>().join(" "));

    //let correct = true_states.iter().zip(best_path.iter()).filter(|(a, b)| a == b).count();
    //println!("Accuracy: {}/{}", correct, true_states.len());
}
