//! file: rnd.rs
//! author: Jacob Xie
//! date: 2024/02/04 09:39:07 Sunday
//! brief:

use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Alphanumeric, Distribution, Standard};
use rand::rngs::StdRng;
use rand::{thread_rng, Rng, SeedableRng};

// ================================================================================================
// const
// ================================================================================================

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~";

const DOMAIN: &[&str] = &["com", "org", "rs"];

// ================================================================================================
// fn
// ================================================================================================

pub fn rand_by_range<T>(start: T, end: T) -> T
where
    T: SampleUniform + std::cmp::PartialOrd,
{
    thread_rng().gen_range(start..end)
}

pub fn rand_fixed_string(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn rand_range_string(start: usize, end: usize) -> String {
    let len = rand_by_range(start, end);
    rand_fixed_string(len)
}

pub fn rand_password(len: usize) -> String {
    (0..len)
        .map(|_| {
            let idx = rand_by_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn rand_email() -> String {
    let prefix = rand_range_string(5, 10).to_lowercase();
    let suffix = rand_range_string(3, 9).to_lowercase();
    let domain = DOMAIN[rand_by_range(0, DOMAIN.len())];

    format!("{prefix}@{suffix}.{domain}")
}

// ================================================================================================
// test
// ================================================================================================

#[test]
fn rand_email_success() {
    let a1 = rand_email();
    let a2 = rand_email();
    let a3 = rand_email();

    println!("{:?}", a1);
    println!("{:?}", a2);
    println!("{:?}", a3);
}

#[allow(dead_code)]
#[derive(Debug)]
struct User {
    id: u64,
    name: String,
    email: String,
}

impl Distribution<User> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> User {
        User {
            id: rng.gen(),
            name: rand_range_string(3, 8),
            email: rand_email(),
        }
    }
}

impl User {
    fn rand_sample() -> User {
        StdRng::from_entropy().sample(Standard)
    }
}

#[test]
fn rand_user_success() {
    let u1 = User::rand_sample();
    let u2 = User::rand_sample();
    let u3 = User::rand_sample();

    println!("{:?}", u1);
    println!("{:?}", u2);
    println!("{:?}", u3);
}
