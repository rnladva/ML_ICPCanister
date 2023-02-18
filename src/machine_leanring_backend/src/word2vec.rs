use crate::cbow::*;
use crate::sampler::*;
use crate::params::*;
use crate::rate::*;
use crate::document_stream::*;
use crate::dictionary::*;

use std::{collections::HashMap};
use serde::{Serialize, Deserialize};
use candid::CandidType;
use std::cell::RefCell;

#[derive(Serialize, Deserialize, CandidType, Debug, Clone)]
pub struct Word2Vec {
    pub model: CBOW,
    unigrams: Sampler,
    dict: Dictionary,
}

impl Word2Vec {
    pub fn new(unigrams: Sampler, dict: Dictionary, n_rows: usize, n_hidden: usize) -> Word2Vec {
        Word2Vec { 
            model: CBOW { 
                embed: ParameterStore::seeded(n_rows, n_hidden),
                predict: ParameterStore::zeros(n_rows, n_hidden),
             }, 
            unigrams, 
            dict
        }
    }

    pub fn optimize(&mut self, file: Vec<String>, win: usize, n_sample: usize, r: LearningRate, epochs: usize) {
        let mut adjusted_rate = r.clone();
        for epoch in 0 .. epochs {
            let stream = DocumentStream::new(file.clone(), &self.dict).doc(&self.dict);
            let mut total_error = 0.0f32;
            let mut n_windows = 0i32;
            
            for document in stream {
                
                for win_idx in 0 .. document.words.len() {
                    if let Some(window) = document.window(win_idx, win) {
                        total_error += self.model.negative_sampling(&window, adjusted_rate.rate, n_sample, &self.unigrams);
                        n_windows += 1;
                        
                        if n_windows % 10000 == 0 {
                            ic_cdk::println!("\t- EPOCH {} ERROR {} RATE {} WINDOWS {}", epoch, total_error, adjusted_rate.rate, n_windows);
                            total_error = 0.0f32;
                        }
                    }
                }
            }
            adjusted_rate.update();
        }

    }
}