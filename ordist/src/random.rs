use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::traits::OrDistElement;

pub trait Shuffle: Sized {
    fn shuffle(&self) -> Self {
        self.shuffle_seed(gen_seed_u32())
    }
    fn shuffle_into(&mut self) -> Self {
        self.shuffle_seed_into(gen_seed_u32())
    }
    fn shuffle_inplace(&mut self) {
        self.shuffle_seed_inplace(gen_seed_u32())
    }
    fn shuffle_seed(&self, random_seed: u32) -> Self;
    fn shuffle_seed_into(&mut self, random_seed: u32) -> Self;
    fn shuffle_seed_inplace(&mut self, random_seed: u32);
}

impl<T: OrDistElement> Shuffle for Vec<T> {
    fn shuffle_seed(&self, random_seed: u32) -> Self {
        let mut v = self.clone();
        v.shuffle_seed_inplace(random_seed);
        v
    }

    fn shuffle_seed_into(&mut self, random_seed: u32) -> Self {
        self.shuffle_seed_inplace(random_seed);
        self.clone()
    }

    fn shuffle_seed_inplace(&mut self, random_seed: u32) {
        shuffle(self, random_seed);
    }
}

pub fn shuffle<T: OrDistElement>(v: &mut [T], random_seed: u32) {
    let mut v2 = v.to_owned();
    let n = v2.len();
    let mut rng = StdRng::from_seed(seed_from_u32(random_seed));
    for i in 0..n {
        let ud = Uniform::new(0, n - i);
        let e = v2.remove(ud.sample(&mut rng));
        v[i] = e;
    }
}

pub fn gen_seed_u32() -> u32 {
    let ud = Uniform::new(u32::MIN, u32::MAX);
    ud.sample(&mut rand::thread_rng())
}
pub fn seed_from_u32(seed: u32) -> [u8; 32] {
    let mut v = [0; 32];
    for i in 0..32 {
        v[i] = (seed >> i & 1) as u8;
    }
    v
}
