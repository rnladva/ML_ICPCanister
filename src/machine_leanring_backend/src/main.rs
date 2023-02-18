
use candid::candid_method;
use ic_cdk_macros::{query, update};


use machine_leanring_backend::document_stream::DocumentStream;
use machine_leanring_backend::{sampler, document_stream};
use machine_leanring_backend::{ dictionary::Dictionary};
// use ngram::{get_ngram, get_ngram_with_padding};
use machine_leanring_backend::vector::Vector;
use machine_leanring_backend::word2vec::*;
use machine_leanring_backend::params::*;
use machine_leanring_backend::rate;
use machine_leanring_backend::dictionary::{
    input_data,
     get_data,
     get_dictionary
    };

pub static PROP_STR: &str =r###"
{
    what is the step by step guide to invest in share market in india,
    what is the step by step guide to invest in share market,
    what is the story of kohinoor koh noor diamond,
    what would happen if the indian government stole the kohinoor koh noor diamond back,
    how can increase the speed of my internet connection while using vpn,
    how can internet speed be increased by hacking through dns,
}
    "###;


#[update]
#[candid_method(update)]
fn input_dict_data(s: String) {
    input_data(s);
}

#[query]
#[candid_method(query)]
fn query_dict_data() -> Vec<String> {
    get_data()
}

#[update]
#[candid_method(update)]
fn gen_dict() {
    // let prop = serde_json::from_str(PROP_STR).unwrap();
    // ic_cdk::println!("{:?} ",prop);
    let data = get_data();
    Dictionary::estimate(data);

}

#[query]
#[candid_method(query)]
fn gen_vec() {
    let vector = Vector::new(vec![1.0, 2.0, 3.0 ,4.0]);
    ic_cdk::println!("{:?}", vector);
}



#[update]
#[candid_method(update)]
fn gen_unigram() -> ParameterStore{
    let data = get_data();
    let dim = 32;
    let win = 5;
    let start_rate = 0.025;
    let min_rate = 0.0001;
    let step_rate = 0.002;
    let n_samples = 8;
    let epochs = 5;
    let rate_var = rate::LearningRate {
        rate: start_rate,
        min: min_rate,
        step: step_rate,
    };
    ic_cdk::println!("data {:?}", &data);
    Dictionary::estimate(data.clone());

    let dict_data = get_dictionary();
    ic_cdk::println!("dict data {:?}", &dict_data);
    let mut doc_stream = DocumentStream::new(data.clone(), &dict_data);
    let vec_doc = doc_stream.doc(&dict_data)
    .iter()
    .map(|document| document.words.clone())
    .flatten()
    .collect();
    let unigram = sampler::Sampler::unigram(
        vec_doc
        , dict_data.n_ids);
    ic_cdk::println!("unigram {:?}", &unigram);
    let mut w2v = Word2Vec::new(unigram, dict_data.clone(), dict_data.n_ids, dim);
    ic_cdk::println!("\t with param size: {} {}",
            w2v.model.embed.rows(),
            w2v.model.embed.cols
            
        );
    w2v.optimize(data.clone(), win, n_samples, rate_var, epochs);
    ic_cdk::println!("get embeddings");
    get_emb()
}

#[query]
#[candid_method(query)]
fn get_sample() -> sampler::Sampler {
    machine_leanring_backend::sampler::get_sample()
}

#[query]
#[candid_method(query)]
fn get_all_dict() -> Dictionary {
    get_dictionary()
}

pub fn read_u32_into(src: &[u8], dst: &mut [u32]) {
    assert!(src.len() > 4 * dst.len());
    for (out, chunk) in dst.iter_mut().zip(src.chunks_exact(4)) {
        *out = u32::from_le_bytes(chunk.try_into().unwrap());
    }
}

pub fn read_u64_into(src: &[u8], dst: &mut [u64]) {
    assert!(src.len() >= 8 * dst.len());

    for (out, chunk) in dst.iter_mut().zip(src.chunks_exact(8)) {
        *out = u64::from_le_bytes(chunk.try_into().unwrap());
    }
}

#[query]
#[candid_method(query)]
fn test_read() {
    let bytes = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    let mut buf = [0u32; 4];
    read_u32_into(&bytes, &mut buf);
    assert_eq!(buf[0], 0x04030201);
    assert_eq!(buf[3], 0x100F0E0D);

    let mut buf = [0u32; 3];
    read_u32_into(&bytes[1..13], &mut buf); // unaligned
    assert_eq!(buf[0], 0x05040302);
    assert_eq!(buf[2], 0x0D0C0B0A);

    let mut buf = [0u64; 2];
    read_u64_into(&bytes, &mut buf);
    assert_eq!(buf[0], 0x0807060504030201);
    assert_eq!(buf[1], 0x100F0E0D0C0B0A09);

    let mut buf = [0u64; 1];
    read_u64_into(&bytes[7..15], &mut buf); // unaligned
    assert_eq!(buf[0], 0x0F0E0D0C0B0A0908);
}

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    ic_cdk::export::candid::export_service!();
    __export_service()
}
#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    std::print!("{}", export_candid());
}
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}