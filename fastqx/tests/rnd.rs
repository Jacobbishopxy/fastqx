//! file: rnd.rs
//! author: Jacob Xie
//! date: 2024/02/04 09:39:07 Sunday
//! brief:

use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};
use fastqx::utils::ChronoHelper;
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

pub fn rand_letter() -> char {
    thread_rng().gen_range('a'..'z')
}

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
    let mut prefix = rand_range_string(2, 10).to_lowercase();
    prefix.insert(0, rand_letter());
    let suffix = rand_range_string(3, 9).to_lowercase();
    let domain = DOMAIN[rand_by_range(0, DOMAIN.len())];

    format!("{prefix}@{suffix}.{domain}")
}

enum DuType {
    Da,
    Sc,
}

fn rand_t<T>(start: T, end: T, t: DuType) -> T
where
    T: Copy,
    T: std::ops::Sub<Output = Duration>,
    T: std::ops::Add<Duration, Output = T>,
{
    let num = end - start;
    let num = match t {
        DuType::Da => Duration::days(rand_by_range(0, num.num_days())),
        DuType::Sc => Duration::seconds(rand_by_range(0, num.num_seconds())),
    };
    start + num
}

pub fn rand_local_datetime_by_str(start: &str, end: &str) -> DateTime<Local> {
    rand_t(
        ChronoHelper.gen_local_datetime_unchecked(start),
        ChronoHelper.gen_local_datetime_unchecked(end),
        DuType::Sc,
    )
}

pub fn rand_naive_datetime_by_str(start: &str, end: &str) -> NaiveDateTime {
    rand_t(
        ChronoHelper.gen_naive_datetime_unchecked(start),
        ChronoHelper.gen_naive_datetime_unchecked(end),
        DuType::Sc,
    )
}

pub fn rand_naive_date_by_str(start: &str, end: &str) -> NaiveDate {
    rand_t(
        ChronoHelper.gen_naive_date_unchecked(start),
        ChronoHelper.gen_naive_date_unchecked(end),
        DuType::Da,
    )
}

pub fn rand_naive_time_by_str(start: &str, end: &str) -> NaiveTime {
    rand_t(
        ChronoHelper.gen_naive_time_unchecked(start),
        ChronoHelper.gen_naive_time_unchecked(end),
        DuType::Sc,
    )
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
    registry_time: DateTime<Local>,
    expired_time: NaiveDateTime,
    birthday: NaiveDate,
    cron: NaiveTime,
}

impl Distribution<User> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> User {
        User {
            id: rng.gen(),
            name: rand_range_string(3, 8),
            email: rand_email(),
            registry_time: rand_local_datetime_by_str("19900101.000000", "20200101.000000"),
            expired_time: rand_naive_datetime_by_str("20300101.000000", "20500101.000000"),
            birthday: rand_naive_date_by_str("19900101", "20200101"),
            cron: rand_naive_time_by_str("000000", "235959"),
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
