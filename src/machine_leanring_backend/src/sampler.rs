use ic_rand::utils::{Rand32, Rng};
use serde::{Serialize, Deserialize};
use candid::CandidType;
use std::cell::RefCell;
use ic_cdk::api::time;
#[derive(Serialize, Deserialize, CandidType, Debug, Clone)]
pub struct Sampler {
    counts: Vec<f32>,
    max: f32,
}

thread_local! {
    static SAMPLER :RefCell<Sampler> = RefCell::new(Sampler::default()); 
}

impl Default for Sampler {
    fn default() -> Self {
        Sampler {
             counts: Vec::new(), 
             max: 0f32 
            }
    }
}

impl Sampler {
    pub fn new(counts: Vec<f32>) -> Sampler {
        let mut max = 0.0;
        for c in &counts {
            if *c > max {
                max = *c;
            }
        }

        Sampler { counts, max }
    }

    pub fn unigram(words: Vec<usize>, rows: usize) -> Sampler {
        let mut counts = vec![0_f32; rows];
        ic_cdk::println!("words {:?}", &words);
        ic_cdk::println!("rows {:?}", &rows);
        for word in words.iter() {
            counts[*word] += 1.0;
        }

        for c in &mut counts {
            *c = (*c).powf(0.75);
        }

        let mut so_far = 0.0;
        for c in &mut counts {
            so_far += *c;
            *c = so_far;
        }

        let sampler_var = Sampler::new(counts);
        SAMPLER.with(|sampler|
            *sampler.borrow_mut() = sampler_var.clone()
        );
        sampler_var
    }

    fn bisect(&self, search: f32, lo: usize, hi: usize) -> usize {
        let center = ((hi - lo) / 2) + lo;
        if center == 0 || center == hi || search <= self.counts[center] && search > self.counts[center - 1] {
            usize::min(center, hi - 1)
        } else  if search > self.counts[center] {
            self.bisect(search, center, hi)
        } else {
            self.bisect(search, lo, center)
        }
    }

    pub fn multinomial(&self) -> usize {
        let n = self.counts.len();
        let seed = time();
        let mut rng = Rand32::new(seed);
        let uniform = rng.gen_range(0.0, self.max);
        self.bisect(uniform, 0, n)
    }
}

pub fn get_sample() -> Sampler {
    SAMPLER.with(|sampler|
        sampler.borrow().clone()
    )
}