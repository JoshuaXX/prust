#[derive(Debug, Clone, Copy)]
pub enum Ordering {
    Less = 100,
    Equal = 200,
    Greater = 300,
}

pub fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Ordering::Less
    } else if n > m {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}
pub enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

fn main() {
    let _rt = RoughTime::InTheFuture(TimeUnit::Seconds, 2400);
}
