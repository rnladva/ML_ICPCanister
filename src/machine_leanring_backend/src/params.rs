// use crate::util::Rand32;
use ic_rand::utils::{Rand32, Rng};
use serde::{Serialize, Deserialize};
use candid::CandidType;

use std::{cell::RefCell, f32::consts::E};



#[derive(Serialize, Deserialize, CandidType, Debug, Clone)]
pub struct ParameterStore {
    pub cols: usize,
    pub weights: Vec<f32>
}

thread_local! {
    static EMB: RefCell<ParameterStore> = RefCell::new(ParameterStore::default());
}

pub fn add_emb(weight: Vec<f32>) {
    EMB.with(|emb| 
        {for w in weight.iter() {
            emb.borrow_mut().weights.push(w.to_owned());
        }
        }
    );
}

pub fn get_emb() -> ParameterStore {
    EMB.with(|emb|
        emb.borrow().clone()
    )
}

impl Default for ParameterStore {
    fn default() -> Self {
        Self { 
            cols: 0usize, 
            weights: Vec::new(),
         }
    }
}

impl ParameterStore {
    pub fn seeded(rows: usize, cols: usize) -> ParameterStore {
        let seed = ic_cdk::api::time();
        let mut rng = Rand32::new(seed);
        let mut weights = Vec::new();
        for _i in 0..(rows * cols) {
            let uniform = (rng.gen_range(0.0, 1.0) - 0.5f32) / cols as f32;
            weights.push(uniform);
        }
        ParameterStore { cols, weights }
    }

    pub fn zeros(rows: usize, cols: usize) -> ParameterStore {
        ParameterStore { cols, weights: vec![0.0; rows * cols] }
    }

    pub fn rows(&self) -> usize {
        self.weights.len() / self.cols
    }

    pub fn at(&self, i: usize) -> &[f32] {
        let from = i * self.cols;
        let to = (i + 1) * self.cols;
        &self.weights[from .. to]
    }

    pub fn update(&mut self, i: usize, v: &[f32]) {
        let from = i * self.cols;
        for (i, x) in v.iter().enumerate().take(self.cols) {
            self.weights[i + from] += x;
        }
    }

    pub fn avg(&self, result: &mut Vec<f32>, indices: Vec<usize>) {
        for (col, x) in result.iter_mut().enumerate().take(self.cols) {
            *x = 0.0;
            for row in indices.iter() {
                let from = row * self.cols;
                *x += self.weights[col + from];
            }
            *x /= indices.len() as f32;
        }
    }
}