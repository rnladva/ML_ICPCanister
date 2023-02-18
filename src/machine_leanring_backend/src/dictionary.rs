use std::{collections::HashMap};
use serde::{Serialize, Deserialize};
use candid::CandidType;
use std::cell::RefCell;


thread_local! {
    static ID2PROP :RefCell<Dictionary> = RefCell::new(Dictionary::default()); 
    static DATA: RefCell<Data> = RefCell::new(Vec::new());
}
pub type Data = Vec<String>;

#[derive(Serialize, Deserialize, CandidType, Debug, Clone)]
pub struct Dictionary {
    pub word2id: HashMap<String, usize>,
    pub id2words: HashMap<usize, String>,
    pub n_ids: usize,
}

// impl Dictionary {

//     pub fn init_ids(mut self) -> Self {
//         self.n_ids = 0;
//         self
//     }

//     pub fn inc(&mut self) {
//         self.n_ids += 1;
//     }
// }

pub fn input_data(s: String) {
    DATA.with(|data|
        data.borrow_mut().push(s)
    );
}

pub fn get_data() -> Vec<String> {
    DATA.with(|data|
        data.borrow().clone()
    )
}
impl Default for Dictionary {
    fn default() -> Self {
        Dictionary { 
            word2id: HashMap::new(), 
            id2words: HashMap::new(), 
            n_ids: 0
         }
    }
}
impl Dictionary {

    pub fn estimate(mut input: Vec<String>)  {
        let dict = Dictionary::default();
        let current_id = 0;
        for line in input.iter_mut() {
            let components: Vec<&str> = line.trim().split_terminator(' ').collect();

            let tokens: Vec<&str> = components[1].trim().split_terminator(' ').collect();
            let ids: Vec<&str> = components[0].trim().split_terminator(' ').collect();

            for token in tokens.iter().chain(ids.iter()) {
                if !dict.word2id.contains_key(&token.to_string()) {
                    ID2PROP.with(|dict|
                        {
                            dict.borrow_mut().word2id.insert(token.to_string(), current_id);
                            dict.borrow_mut().id2words.insert(current_id, token.to_string());
                            dict.borrow_mut().n_ids += 1;
                        }
                    );
                    // dict.word2id.insert(token.to_string(), current_id);
                    // dict.id2words.insert(current_id, token.to_string());
                    // ic_cdk::prinstln!("current id {:?}", &current_id);
                    // current_id += 1;
                }
            }
            line.clear();
        }
        // dict.n_ids = current_id;
        
    }
}

pub fn get_dictionary() -> Dictionary {
    ID2PROP.with(|dict| 
        dict.borrow().clone()
    )
}