use std::cmp;
use std::env;
use std::fs::File;
use std::io::Read;

fn slurp_to_result(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(&filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn slurp(input: Option<&String>) -> Result<String, std::io::Error> {
    let filename = input.expect("missing required input filename");

    let rv = slurp_to_result(&filename[..]);
    if let Err(e) = rv {
        eprintln!("Error reading {}: {}", filename, e);
        Err(e)
    } else {
        rv
    }
}

fn day0() -> i32 {
    println!("{}", "In the beginning was the NULL, and it was without form, and void*.");
    0
}

fn day1(input: Option<&String>) -> i32 {
    let data = slurp(input);
    if let Err(_e) = data {
        return 1;
    }
    let s: String = data.unwrap();

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
    let mut left = s.trim().chars();
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

fn day2(input: Option<&String>) -> i32 {
    let data = slurp(input);
    if let Err(_e) = data {
        eprintln!("Expected 1 spreadsheet input, e.g. 'i j k, l m n' (2r x 3c)");
        eprintln!("(Yes, one line; it used to be a CLI arg.)");
        return 1;
    }
    let s: String = data.unwrap();

    // input: a spreadsheet (in a file), such as "5 1 9 5, 7 5 3, 2 4 6 8"
    // checksum: sum of a value per row
    // row value: difference between largest and smallest cells
    // cell data: ???; 1-digit integers are provided in the lone example.
    let bad_input = "Spreadsheet data MUST be numeric";

    // the answer
    let mut checksum: i64 = 0;

    // We can do this in-place, in one pass.
    // for each line, "get the row value"...
    // ...which is the difference between min and max cell values...
    // ...so we look at each cell and keep a running min + max.
    // And then use that min/max to add to the checksum.
    let lines = s.split(',');
    for l in lines {
        let mut cells = l.trim().split_whitespace();
        // We can't put min/max into the if-let because checksum uses them, too
        let mut min = 0;
        let mut max = 0;

        // Pull the first value off of cells, then iterate over the rest
        // (Can we have a blank row? It is unspecified.  We will use 0 for it.)
        if let Some(value) = cells.next() {
            // once again, non-numeric input seems to be undefined.
            // maybe the site never gives us invalid input?
            // in any case, let's crash fast and hard on unexpected errors.
            let mut v: i64 = value.parse().expect(bad_input);
            min = v;
            max = v;
            for cell in cells {
                v = cell.parse().expect(bad_input);
                min = cmp::min(min, v);
                max = cmp::max(max, v);
            }
        }

        // All (0 to N) cells processed, add to checksum
        checksum += max - min;
    }

    // display and return
    println!("{}", checksum);
    0
}

fn day3(input: Option<&String>) -> i32 {
    // Gather input.
    let s = input.expect("Starting position in memory required");
    let start: i32 = s.parse().expect("Starting position in memory MUST be numeric");
    if start == 1 {
        println!("0, you joker");
        return 0;
    }

    // Not going to lie, getting this algorithm took a lot of thinking.
    // Find the ring number by comparing squares, iteratively.
    let mut order: i32 = 3;
    while (order*order) < start {
        order += 2;
    }
    //println!("order is {}", order);

    // Find the nearest corners to our position.
    let far_corner = order*order;
    let prev_order = order - 2;
    let size = far_corner - prev_order*prev_order;
    let quad = size / 4;
    let mut corner = far_corner;
    // specifically, find the corner immediately before our position.
    while start < corner {
        corner -= quad;
    }

    // Moves = ABS(position, AVERAGE(nearest 2 corners)) + (k-1)/2
    let center_of_edge = (corner + corner + quad) / 2;
    let in_ring_moves = start - center_of_edge;
    let cross_ring_moves = (order - 1) / 2;
    //println!("{} moves in-ring, {} moves centerward", in_ring_moves.abs(), cross_ring_moves);
    let moves = in_ring_moves.abs() + cross_ring_moves;

    println!("{}", moves);
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
        2 => day2(args.get(2)),
        3 => day3(args.get(2)),
        4...24 => no_day(day),
        25 => christmas_day(),
        _ => never_day(),
    }
}

fn main() {
    let status = real_main();
    std::process::exit(status);
}
