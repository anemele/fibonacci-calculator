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
            eprintln!("invalid number, a none-negative integer is required");
            continue;
        };

        println!("{}", calc_fib(num));
    }

    Ok(())
}

fn run(args: &[String]) -> anyhow::Result<()> {
    for num in args {
        let Ok(num) = num.parse::<u64>() else {
            eprintln!("invalid number, a none-negative integer is required");
            continue;
        };
        println!("fib({})={}", num, calc_fib(num));
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        repl()?;
    } else {
        run(&args[1..])?;
    }

    Ok(())
}
