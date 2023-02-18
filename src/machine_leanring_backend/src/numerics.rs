pub fn euclidean(x: &[f32], y: &[f32]) -> f32 {
    let mut distance = 0.0;
    for i in 0 .. x.len() {
        distance += (x[i] - y[i]).powf(2.0)
    }
    f32::sqrt(distance)
}

pub fn sub(x: usize, y: usize) -> usize {
    if x >= y {
        x - y
    } else {
        0
    }
}

pub fn dot(x: &[f32], y: &[f32]) -> f32 {
    let mut result = 0.0;
    for i in 0 .. x.len() {
        result += x[i] * y[i];
    }
    result
}

pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + f32::exp(-x))
}