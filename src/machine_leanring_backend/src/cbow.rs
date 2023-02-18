use crate::sampler::*;
use crate::params::*;
use crate::window::*;
use crate::numerics::*;

use serde::{Serialize, Deserialize};
use candid::CandidType;
#[derive(Serialize, Deserialize, CandidType, Debug, Clone)]
pub struct  CBOW {
    pub embed: ParameterStore,
    pub predict: ParameterStore,
}

impl CBOW {
    fn ids(&self, win: &Window) -> Vec<usize> {
        let mut ids = Vec::new();
        for i in 0 .. win.words.len() {
            if i != win.predict_pos {
                ids.push(win.words[i])
            }
        }

        for id in win.ids.iter() {
            ids.push(*id)
        }
        ids
    }

    fn embedding(&self, ids: Vec<usize>) -> Vec<f32> {
        let mut result = vec![0.0; self.embed.cols];
        self.embed.avg(&mut result, ids);
        result
    }

    fn gradient(&self, label: usize, h: &[f32], truth: f32, rate: f32) -> (f32, f32) {
        let w = self.predict.at(label);
        let a = sigmoid(dot(&h, &w));
        let d = (truth - a) * rate;
        let e = -f32::ln(if (truth - 1f32).abs() < f32::EPSILON{a} else {1f32 - a});
        (d, e)
    }

    pub fn negative_sampling(&mut self, window: &Window, rate: f32, n_sample: usize, unigram: &Sampler) -> f32 {
        let mut gradient_embed = vec![0.0; self.embed.cols];
        let h = self.embedding(self.ids(window));
        let (pos, pos_err) = self.gradient(window.label(), &h, 1.0, rate);
        let mut error = pos_err;
        let mut gradient_pos_predict = vec![0.0; self.predict.cols];
        for i in 0 .. self.embed.cols {
            gradient_embed[i] += pos * self.predict.at(window.label())[i];
            gradient_pos_predict[i] += pos * h[i];
        }

        self.predict.update(window.label(), &gradient_pos_predict);
        for _sample in 0 .. n_sample {
            let token = unigram.multinomial();
            let (neg, neg_error) = self.gradient(token, &h, 0.0, rate);
            error += neg_error;
            let mut gradient_neg_predict = vec![0.0; self.predict.cols];
            for i in 0 .. self.embed.cols {
                gradient_embed[i] += neg * self.predict.at(token)[i];
                gradient_neg_predict[i] += neg * h[i];
            }
            self.predict.update(token, &gradient_neg_predict);

        }

        for i in self.ids(window).iter() {
            self.embed.update(*i, &gradient_embed);
        }
        add_emb(h.clone());
        error
    }
}