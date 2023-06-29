use rand::{distributions::Standard, prelude::Distribution};

pub mod cuddle;
pub mod pet;

/// Allows heather to choose a way to react
enum Reaction {
    /// She liked what you did
    Happy,
    /// She doesn't care much about what you did
    Ignore,
    /// She hated what you did
    Anger,
}
impl Distribution<Reaction> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Reaction {
        match rng.gen_range(0..=64) {
            0..=3 => Reaction::Anger,
            4..=6 => Reaction::Ignore,
            _ => Reaction::Happy,
        }
    }
}
