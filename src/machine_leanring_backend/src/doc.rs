use crate::window::*;
use crate::numerics::*;
use crate::dictionary::*;

use serde::{Serialize, Deserialize};
use candid::CandidType;
use std::cell::RefCell;

#[derive(Serialize, Deserialize, CandidType, Debug, Clone)]
pub struct  Document {
    pub words: Vec<usize>,
    pub document_ids: Vec<usize>,
}

thread_local! {
    static DOCUMENT :RefCell<Vec<Document>> = RefCell::new(Vec::new()); 
}

impl Default for Document {
    fn default() -> Self {
        Self { 
            words: Vec::new(), 
            document_ids: Vec::new() 
        }
    }
}

pub fn insert_doc(document: Document) {
    DOCUMENT.with(|docu|
        docu.borrow_mut().push(document)
    );
}

impl Document {
    pub fn new(document_ids: Vec<String>, words: Vec<String>, dict: &Dictionary) -> Document {
        Document { 
            words: words.iter().map(|word| dict.word2id[word]).collect(), 
            document_ids: document_ids.iter().map(|id| dict.word2id[id]).collect(), }
    }

    pub fn window(&self, pos: usize, size: usize) -> Option<Window> {
        let start = sub(pos, size);
        let stop = usize::min(pos + size, self.words.len() - 1);
        if stop - start == 2 * size {
            Some(Window { 
                ids: &self.document_ids, 
                words: &&self.words[start as usize .. stop as usize], 
                predict_pos: size 
            })
        } else {
            None
        }
    }
}