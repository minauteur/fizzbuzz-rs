use std::fmt;

enum FizzBuzzNum {
    IsFizz,
    IsBuzz,
    IsFizzBuzz,
    IsNum(i32),
}

impl fmt::Display for FizzBuzzNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FizzBuzzNum::IsFizz => f.write_str("Fizz"),
            FizzBuzzNum::IsBuzz => f.write_str("Buzz"),
            FizzBuzzNum::IsFizzBuzz => f.write_str("FizzBuzz"),
            FizzBuzzNum::IsNum(num) => write!(f, "{}", num),
        }
    }
}

impl fmt::Debug for FizzBuzzNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

fn fizzbuzz(i: i32) -> FizzBuzzNum {
    if i % 15 == 0 {
        FizzBuzzNum::IsFizzBuzz
    } else if i % 5 == 0 {
        FizzBuzzNum::IsBuzz
    } else if i % 3 == 0 {
        FizzBuzzNum::IsFizz
    } else {
        FizzBuzzNum::IsNum(i)
    }
}

fn main() {
    for i in 1..100 {
        println!("{}", fizzbuzz(i));
    }
}
