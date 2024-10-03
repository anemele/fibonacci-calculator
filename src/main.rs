use std::io::{self, Write};

use num::BigUint;

fn calc_fib(n: u64) -> BigUint {
    if n == 0 {
        eprintln!("the index should begin with 1");
        return 0u64.into();
    }
    let mut a = 0u64.into();
    let mut b = 1u64.into();
    for _ in 1..n {
        let c = &a + &b;
        a = b;
        b = c;
    }
    b
}

#[test]
fn test_calc_fib() {
    assert_eq!(calc_fib(0), 0u64.into());
    assert_eq!(calc_fib(1), 1u64.into());
    assert_eq!(calc_fib(2), 1u64.into());
    assert_eq!(calc_fib(3), 2u64.into());
    assert_eq!(calc_fib(4), 3u64.into());
    assert_eq!(calc_fib(5), 5u64.into());
    assert_eq!(calc_fib(6), 8u64.into());
    assert_eq!(calc_fib(7), 13u64.into());
    assert_eq!(calc_fib(8), 21u64.into());
    assert_eq!(calc_fib(9), 34u64.into());
    assert_eq!(calc_fib(10), 55u64.into());
}

fn calc_fib_series(n: u64) -> impl Iterator<Item = BigUint> {
    if n == 0 {
        eprintln!("the index should begin with 1");
    }

    let mut a: BigUint = 0u64.into();
    let mut b: BigUint = 1u64.into();

    (0..n).map(move |_| {
        let c = &a + &b;
        a = b.clone();
        b = c.clone();
        a.clone()
    })
}

#[test]
fn test_calc_fib_series() {
    let mut fib_series = calc_fib_series(10);
    assert_eq!(fib_series.next().unwrap(), 1u64.into());
    assert_eq!(fib_series.next().unwrap(), 1u64.into());
    assert_eq!(fib_series.next().unwrap(), 2u64.into());
    assert_eq!(fib_series.next().unwrap(), 3u64.into());
    assert_eq!(fib_series.next().unwrap(), 5u64.into());
    assert_eq!(fib_series.next().unwrap(), 8u64.into());
    assert_eq!(fib_series.next().unwrap(), 13u64.into());
    assert_eq!(fib_series.next().unwrap(), 21u64.into());
    assert_eq!(fib_series.next().unwrap(), 34u64.into());
    assert_eq!(fib_series.next().unwrap(), 55u64.into());
    assert_eq!(fib_series.next(), None);
}

#[test]
fn test_calc_fib_series_invalid_index() {
    let mut fib_series = calc_fib_series(0);
    assert_eq!(fib_series.next(), None);
}

fn repl() -> anyhow::Result<()> {
    println!("Fibonacci calculator");
    const EXIT_STR: &str = "exit";
    println!("interactive mode (type `{}` to exit)", EXIT_STR);

    let mut s = String::new();
    let mut stdout = io::stdout();
    loop {
        print!("> ");
        stdout.flush()?;

        s.clear();
        io::stdin().read_line(&mut s)?;

        if s.trim() == EXIT_STR {
            break;
        }

        let Ok(num) = s.trim().parse::<u64>() else {
            eprintln!("invalid number, a non-negative integer is required");
            continue;
        };

        println!("{}", calc_fib(num));
    }

    Ok(())
}

fn calc_one(args: &[String]) -> anyhow::Result<()> {
    for num in args {
        let Ok(num) = num.parse::<u64>() else {
            eprintln!("invalid number, a non-negative integer is required");
            continue;
        };
        println!("fib({})={}", num, calc_fib(num));
    }
    Ok(())
}

fn calc_series(num: &str) -> anyhow::Result<()> {
    let num = num.parse::<u64>()?;
    for num in calc_fib_series(num) {
        println!("{}", num);
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        repl()?;
    } else if args[1] == "series" {
        if args.len() != 3 {
            eprintln!("a non-negative integer is required");
        } else {
            calc_series(&args[2])?;
        }
    } else {
        calc_one(&args[1..])?;
    }

    Ok(())
}
