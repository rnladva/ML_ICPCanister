use crate::dictionary::*;
use crate::doc::*;
#[derive(Clone)]
pub struct DocumentStream<'a> {
    pub reader: Vec<String>,
    pub dict: &'a Dictionary,
}

// impl<'a> Iterator for DocumentStream<'a> {
//     type Item = Document;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.next_document()
//     }
// }

impl<'a> DocumentStream<'a> {
    pub fn new(file: Vec<String>, dict: &Dictionary) -> DocumentStream {
        DocumentStream { reader: file, dict: dict }
    }

    // pub fn next_document(&mut self) -> Option<Document> {

    //     for line in self.reader.iter() {
    //         Some(DocumentStream::doc(self.reader.clone(), &self.dict))
    //     }
            
    // }

    pub fn doc(&mut self, dict: &Dictionary) -> Vec<Document> {
        let mut vec_doc = Vec::new();
        for line in self.reader.iter() {
            let components: Vec<&str> = line.trim().split_terminator(' ').collect();
            let tokens: Vec<&str> = components[1].trim().split_terminator(' ').collect();
            let ids: Vec<&str> = components[0].trim().split_terminator(' ').collect();
            let mut words: Vec<String> = Vec::new();
            let mut id_tokens: Vec<String> = Vec::new();
            for token in tokens {
                words.push(String::from(token));
            }
            for id in ids {
                id_tokens.push(String::from(id));
                
            }
        
            // ic_cdk::println!("id tokens {:?}", &id_tokens);
            // ic_cdk::println!("words {:?}", &words);
            vec_doc.push(Document::new(id_tokens, words, dict));
            
        }
        ic_cdk::println!("{:?}", &vec_doc);
        vec_doc
                    
        
    }
}