use std::ops::{Mul, Add, Div};
use num::{One, Zero, Float};
use std::slice::{Iter, IterMut};
use std::vec::IntoIter;
use std::fmt::{self, write};
use crate::mat_utils;

use super::Vector;

impl<T> Vector<T> {
    pub fn new<U: Into<Vec<T>>>(data: U) -> Vector<T> {
        let our_data = data.into();
        let size = our_data.len();

        Vector {
            size: size,
            data: our_data,
        }
    }

    pub fn from_fn<F>(size: usize, mut f: F) -> Vector<T>
    where F: FnMut(usize) -> T 
    {
        let data: Vec<T> = (0..size).into_iter().map(|x| f(x)).collect();
        
        Vector { 
            size: size, 
            data: data, 
        }
    }

    ///return the size of vector
    pub fn size(&self) -> usize {
        self.size
    }

    ///return a non-mutable reference to the underlying data
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    ///returna a mutable slice of the underling data
    pub fn mut_data(&mut self) -> &mut [T] {
        &mut self.data
    }

    ///consumes the vector and returns the vec of data
    pub fn into_vec(self) -> Vec<T> {
        self.data 
    }

    ///returns an iterator over the vector's data
    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    ///returns an iterator over mutable references to the vector data
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.mut_data().iter_mut()
    }

    ///returns a pointer to the element at the given index, without doing
    /// bounds checking
    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        self.data.get_unchecked(index)
    }

    ///returns an unsafe mutable pointer to the element at the given index
    /// without doing bounds checking
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        self.data.get_unchecked_mut(index)
    }

}

impl<T> Into<Vec<T>> for Vector<T> {
    fn into(self) -> Vec<T> {
        self.data
    }
}

impl<T> IntoIterator for Vector<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Vector<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> FromIterator<T> for Vector<T> {
    fn from_iter<I>(iter: I) -> Self where I: IntoIterator<Item=T> {
        let values: Vec<T> = iter.into_iter().collect();
        Vector::new(values)
    }
}

// impl<T: fmt::Display> fmt::Display for Vector<T>{
//     ///display the vector
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         try!(write!(f, "["));
//         for (i, datum) in self.data.iter().enumerate() {
//             match f.precision() {
//                 Some(places) => {
//                     try!(write(f, "{:.*", places, datum));
//                 }
//                 None => {
//                     try!(write(f, " {}", datum));
//                 }
//             }
//             if i < self.data.len() - 1 {
//                 try!(write(f, ","));
//             }
//         }
//         write(f, "]")
//     }
// }

impl<T: Clone> Clone for Vector<T> {
    ///clones the vector
    fn clone(&self) -> Vector<T> {
        Vector { 
            size: self.size, 
            data: self.data.clone(),
         }
    }
}

impl<T: Copy> Vector<T> {
    ///applies a function to each element in the vector
    pub fn apply(mut self, f: &dyn Fn(T) -> T) -> Vector<T> {
        for val in &mut self.data {
            *val = f(*val);
        }
        self
    }
}

impl<T: Copy + PartialOrd> Vector<T> {
    ///find the argmax of the vector
    /// 
    /// returns the index of the largest value in the vector
    /// 
    pub fn argmax(&self) -> (usize, T) {
        mat_utils::argmax(&self.data)
    }

    ///find the argmin of the vector
    /// 
    /// returns the index of the smallest value in the vector
    pub fn argmin(&self) -> (usize, T) {
        mat_utils::argmin(&self.data)
    }


    // ///select elements from the vector and form a new vector from them
    // pub fn select(&self, idxs: &[usize]) -> Vector<T> {
    //     let mut new_data = Vec::with_capacity(idxs.len());

    //     for idx in idxs.into_iter() {
    //         new_data.push(self[*idx]);
    //     }

    //     Vector::new(new_data)
    // }
}

impl<T: Clone + Zero> Vector<T> {
    ///constructs vector of all zeros
    /// 
    /// requires the size of the vector
    pub fn zeros(size: usize) -> Vector<T> {
        Vector { 
            size: size, 
            data: vec![T::zero(); size],
        }
    }
}

impl<T: Clone + One> Vector<T> {
    ///constructs vector of all ones.
    /// 
    /// requires the size of the vector
    pub fn ones(size: usize) -> Vector<T> {
        Vector { 
            size: size, 
            data: vec![T::one(); size], 
        }
    }
}

impl<T: Copy + Zero + Mul<T, Output = T> + Add<T, Output = T>> Vector<T> {
    ///compute dot product with specified vector
    pub fn dot(&self, v: &Vector<T>) -> T {
        mat_utils::dot(&self.data, &v.data)
    }
}

impl<T: Copy + Zero + Add<T, Output = T>> Vector<T> {
    /// the sum of the vector
    /// 
    /// returns the sum of all elememts in the vector
    pub fn sum(&self) -> T {
        mat_utils::unrolled_sum(&self.data[..])
    }

}

impl<T: Copy + Mul<T, Output = T>> Vector<T> {
    ///the elementwise product of two vectors
    /// 
    pub fn elemul(&self, v: &Vector<T>) -> Vector<T> {
        assert_eq!(self.size, v.size);
        Vector::new(mat_utils::ele_mul(&self.data, &v.data))
    }
}

impl<T: Copy + Div<T, Output = T>> Vector<T>  {
    /// the elementwise division of two vectors
    pub fn elediv(&self, v: &Vector<T>) -> Vector<T> {
        assert_eq!(self.size, v.size);
        Vector::new(mat_utils::ele_div(&self.data, &v.data))
    }
}

// impl<T: Float> Vector<T> {
//     ///compute vecror norm for vector
    
    
// }