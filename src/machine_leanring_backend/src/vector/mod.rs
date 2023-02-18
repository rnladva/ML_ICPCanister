pub mod impl_vec;
pub mod impl_ops;
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Vector<T> {
    size: usize,
    data: Vec<T>,
}