use std::cmp;
use num::Zero;
use std::ops::{Add, Mul, Sub, Div};

///computes dot product of two slices
pub fn dot<T: Copy + Zero + Add<T, Output = T> + Mul<T, Output = T>>(u: &[T], v: &[T]) -> T {
    let len = cmp::min(u.len(), v.len());
    let xs = &u[..len];
    let ys = &v[..len];
    
    let mut s = T::zero();
    let (mut p0, mut p1, mut p2, mut p3, mut p4, mut p5, mut p6, mut p7) =
    (T::zero(), T::zero(), T::zero(), T::zero(), T::zero(), T::zero(), T::zero(), T::zero());

    while xs.len() >= 8 {
        p0 = p0 + xs[0] * ys[0];
        p1 = p1 + xs[1] * ys[1];
        p2 = p2 + xs[2] * ys[2];
        p3 = p3 + xs[3] * ys[3];
        p4 = p4 + xs[4] * ys[4];
        p5 = p5 + xs[5] * ys[5];
        p6 = p6 + xs[6] * ys[6];
        p7 = p7 + xs[7] * ys[7];

        // xs = xs[8..];
        // ys = ys[8..];
    }

    s = s + p0 + p4;
    s = s + p1 + p5;
    s = s + p2 + p6;
    s = s + p3 + p7;

    for i in 0..xs.len() {
        s = s + xs[i] * ys[i];
    }
    s
}

///unrolled sum
/// computes the sum over the slice consuming it in the process
/// 
/// given graciously by bluss from ndarray!
pub fn unrolled_sum<T>(mut xs: &[T]) ->T
    where T: Clone + Add<Output = T> + Zero
{
    ///eightfold unrolled so that floating point can be vectorized
    /// (even with struct floating point accuracy sematics)
    let mut sum = T::zero();
    let (mut p0, mut p1, mut p2, mut p3, mut p4, mut p5, mut p6, mut p7) =
    (T::zero(), T::zero(), T::zero(), T::zero(), T::zero(), T::zero(),T::zero(), T::zero());
    while xs.len() >= 8 {
        p0 = p0 + xs[0].clone();
        p1 = p1 + xs[1].clone();
        p2 = p2 + xs[2].clone();
        p3 = p3 + xs[3].clone();
        p4 = p4 + xs[4].clone();
        p5 = p5 + xs[5].clone();
        p6 = p6 + xs[6].clone();
        p7 = p7 + xs[7].clone();
        
        xs = &xs[8..];
    }

    sum = sum.clone() + (p0 + p4);
    sum = sum.clone() + (p1 + p5);
    sum = sum.clone() + (p2 + p6);
    sum = sum.clone() + (p3 + p7);

    for elt in xs {
        sum = sum.clone() + elt.clone();
    }
    sum
}

///vectorized binary operation applied to two slices
/// the first argument should be a mutable slice which will
/// be modified in place to prevent new memory allocation
pub fn in_place_vec_bin_op<F, T>(mut u: &mut [T], v: &[T], mut f: F)
where F: FnMut(&mut T, &T),
        T: Copy
{
    debug_assert_eq!(u.len(), v.len());
    let len = cmp::min(u.len(), v.len());

    let ys = &v[..len];
    let xs = &mut u[..len];
    for i in 0..len {
        f(&mut xs[i], &ys[i])
    }
}

///vectorized binary operation applied to two slices
pub fn vec_bin_op<F, T>(u: &[T], v: &[T], f: F) -> Vec<T>
where F: Fn(T, T) -> T,
    T: Copy
{
    debug_assert_eq!(u.len(), v.len());
    let len = cmp::min(u.len(), v.len());

    let xs = &u[..len];
    let ys = &v[..len];

    let mut out_vec = Vec::with_capacity(len);
    unsafe {
        out_vec.set_len(len);
    }

    {
        let out_slice = &mut out_vec[..len];
        for i in 0..len {
            out_slice[i] = f(xs[i], ys[i]);
        }
    }
    out_vec
}

///compute vector sum of two slices
pub fn vec_sum<T: Copy + Add<T, Output = T>>(u: &[T], v: &[T]) -> Vec<T> {
    vec_bin_op(u, v, |x, y| x + y)
}

///compute vector difference two slices
pub fn vec_sub<T: Copy + Sub<T, Output = T>>(u: &[T], v: &[T]) ->Vec<T> {
    vec_bin_op(u, v, |x, y| x - y)
}

///compute elementwise multiplication
pub fn ele_mul<T: Copy + Mul<T, Output = T>>(u: &[T], v: &[T]) -> Vec<T> {
    vec_bin_op(u, v, |x, y| x * y)
}

///compute elementwise division
pub fn ele_div<T: Copy + Div<T, Output = T>>(u: &[T], v: &[T]) -> Vec<T> {
    vec_bin_op(u, v, |x, y| x / y)
}

///find argmax of slice
/// 
/// returns index of first occuring maximum
pub fn argmax<T>(u: &[T]) -> (usize, T)
    where T: Copy + PartialOrd
{
    assert!(u.len() != 0);

    let mut max_index = 0;
    let mut max = u[max_index];

    for (i, v) in u.iter().enumerate().skip(1) {
        if max < *v {
            max_index = i;
            max = *v;
        }
    }
    (max_index, max)
}

///find argmin of slice
/// returns index of first occuring minimum
pub fn argmin<T>(u: &[T]) -> (usize, T) 
    where T: Copy + PartialOrd
{
    assert!(u.len() != 0);

    let mut min_index = 0;
    let mut min = u[min_index];

    for (i, v) in u.iter().enumerate().skip(1) {
        if min > *v {
            min_index = i;
            min = *v;
        }
    }
    (min_index, min)
}

///find index of value in slice
/// returns index of first occuring value
pub fn find<T>(p: &[T], u: T) -> usize 
    where T: PartialOrd
{
    for (i, v) in p.iter().enumerate() {
        if *v == u {
            return i;
        }
    }
    panic!("value not found")
}