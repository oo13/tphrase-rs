use tphrase::*;

pub struct ZeroNG {}
impl RandomGenerator for ZeroNG {
    fn new() -> Self {
        Self {}
    }
    fn next(self: &mut Self) -> f64 {
        0.0
    }
}

pub struct Point9NG {}
impl RandomGenerator for Point9NG {
    fn new() -> Self {
        Self {}
    }
    fn next(self: &mut Self) -> f64 {
        0.9
    }
}

pub trait LinearNGParam {
    fn new() -> Self;
    fn max() -> f64;
}

#[allow(dead_code)]
pub struct LinearNG<P: LinearNGParam> {
    n: P,
    i: f64,
}
impl<P: LinearNGParam> RandomGenerator for LinearNG<P> {
    fn new() -> Self {
        Self {
            n: P::new(),
            i: 0.0,
        }
    }
    fn next(self: &mut Self) -> f64 {
        if self.i >= P::max() {
            0.0
        } else {
            self.i += 1.0;
            (self.i - 0.5) / P::max()
        }
    }
}

pub struct LinearNGParam3 {}
impl LinearNGParam for LinearNGParam3 {
    fn new() -> Self {
        Self {}
    }
    fn max() -> f64 {
        3.0
    }
}

pub struct LinearNGParam6 {}
impl LinearNGParam for LinearNGParam6 {
    fn new() -> Self {
        Self {}
    }
    fn max() -> f64 {
        6.0
    }
}

#[allow(dead_code)]
pub type LinearNG3 = LinearNG<LinearNGParam3>;
#[allow(dead_code)]
pub type LinearNG6 = LinearNG<LinearNGParam6>;

#[allow(dead_code)]
pub type TextDistribution = std::collections::HashMap<String, f64>;

#[allow(dead_code)]
pub fn check_distribution(
    ph: &mut Generator,
    num: usize,
    dist: &TextDistribution,
    allowance: f64,
) -> bool {
    let mut count = std::collections::HashMap::<String, usize>::new();
    for _ in 0..num {
        let s = ph.generate();
        match dist.get(&s) {
            Some(_) => *count.entry(s).or_insert(1) += 1,
            None => {
                println!("The result \"{}\" is not expected.", s);
                return false;
            }
        }
    }

    let mut is_match = true;
    for (s, probability) in dist.iter() {
        let mut op: f64 = 0.0;
        if let Some(x) = count.get(s) {
            op = (*x as f64) / (num as f64);
        }
        if (op - probability).abs() > allowance {
            println!(
                "The probability ({}) of the result \"{}\" is not match the expected value {}.",
                op, s, probability
            );
            is_match = false;
        }
    }
    return is_match;
}
