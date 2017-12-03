use std::env;

fn day0() -> i32 {
    println!("{}", "In the beginning was the NULL, and it was without form, and void*.");
    0
}

fn day1(input: Option<&String>) -> i32 {
    let s = input.expect("needed 1 symbol string of input");
    // problem is undefined for strings of these lengths
    if s.len() <= 1 {
        return 0;
    }

    // here's our inhuman answer.
    let mut sum: u32 = 0;

    // although the problem is undefined for other values, I'm...
    // (•_•)
    // ( •_•)>⌐■-■
    // (⌐■_■)
    // covering all my bases.
    let base = 10;

    // Okay, for good reason, we can't index into strings.
    // Let's have two iterators, one positioned immediately after the other.
    // We start by bumping 'right' forward 1,
    // to save char 0 for comparison with the final char.
    // 'left' starts at 0 and runs to the penultimate char.
    // 'right' starts at 1 and runs to the final char in the `for` loop.
    // then, we compare the last char, still waiting in 'left', to char 0.
    let mut left = s.chars();
    let mut right = left.clone();
    let zero_char = right.next().unwrap(); // right=[1], save s[0]

    for cur in right { // iterate 1..end
        let prev = left.next().unwrap(); // get corresponding 0..(end-1)
        // parse all our input, so nobody can sneak in "1234oo7".
        let cur_int = cur.to_digit(base).expect("input MUST be numeric");
        if cur == prev {
            sum += cur_int; // add to sum if match seen
        }
    }

    // compare char 0 to final char
    if zero_char == left.next().unwrap() {
        let zero_int = zero_char.to_digit(base).expect("input MUST be numeric");
        sum += zero_int;
    }

    // show result and return success
    println!("{}", sum);
    0
}

fn no_day(day: u8) -> i32 {
    eprintln!("Still loading day {} from the future.", day);
    1
}

fn never_day() -> i32 {
    // just a self-indulgent Discworld reference
    eprintln!("+++ OUT OF CHEESE ERROR +++");
    2
}

fn christmas_day() -> i32 {
    let lyrics: [&str; 10] = [
        "Hark! The herald angels sing",
        "'Glory to the newborn king!",
        "Peace on Earth and mercy mild",
        "God and sinners reconciled!'",
        "Joyful, all ye nations rise",
        "Join the triumph of the skies",
        "With the angelic host proclaim:",
        "'Christ is born in Bethlehem'",
        "Hark! The herald angels sing",
        "'Glory to the newborn king!'",
    ];
    for i in lyrics.iter() {
        println!("\t{}", i);
    }
    0
}

fn real_main() -> i32 {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("{}", "Usage: advent2017-rs <day> [<day's inputs>]");
        return 2;
    }

    let day: u8 = args[1].trim().parse()
        .expect("Day needs to be a day number (0-24)");

    match day {
        0 => day0(),
        1 => day1(args.get(2)),
        2...24 => no_day(day),
        25 => christmas_day(),
        _ => never_day(),
    }
}

fn main() {
    let status = real_main();
    std::process::exit(status);
}
