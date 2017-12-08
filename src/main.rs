extern crate regex;

use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::error::Error; // why ISN'T this in the std::prelude, anyway?
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::Iterator;
use std::iter::FromIterator;

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

fn day1_part1(s: &str) -> u32 {
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

    // return answer
    sum
}

fn day1_part2(s: &str) -> u32 {
    let mut left = s.trim().chars();
    let half = left.clone().count() / 2;
    let right = left.clone().skip(half);

    let mut sum: u32 = 0;
    let base = 10;

    for cur in right {
        let prev = left.next().unwrap();
        if prev == cur {
            // because it's halfway around, each side will match itself.
            // e.g. 1212 will add 1 both times it sees 1.
            // we're only doing half a pass, so count both sides at once.
            sum += 2 * cur.to_digit(base).unwrap();
        }
    }

    sum
}

fn day1(input: Option<&String>) -> i32 {
    let data = slurp(input);
    if let Err(_e) = data {
        return 1;
    }
    let s: String = data.unwrap();

    println!("part 1: {}", day1_part1(&s));
    println!("part 2: {}", day1_part2(&s));
    0
}


fn day2_part1(s: &str) -> i64 {
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

    checksum
}

fn day2_part2(s: &str) -> i64 {
    let mut checksum: i64 = 0;

    for l in s.split(',') {
        let mut cells: Vec<i64> = l.trim().split_whitespace()
            .map(|cell| cell.parse().unwrap())
            .collect();
        cells.sort_unstable();

        // good news: the puzzle input was all positive integers
        'line: for i in 0..cells.len() {
            let x = cells[i];
            for j in i..cells.len() {
                let y = cells[j];
                if y > x && y % x == 0 {
                    checksum += y/x;
                    break 'line;
                }
            }
        }
    }

    checksum
}

fn day2(input: Option<&String>) -> i32 {
    let data = slurp(input);
    if let Err(_e) = data {
        eprintln!("Expected 1 spreadsheet input, e.g. 'i j k, l m n' (2r x 3c)");
        eprintln!("(Yes, one line; it used to be a CLI arg.)");
        return 1;
    }
    let s: String = data.unwrap();

    // display and return
    println!("part 1: {}", day2_part1(&s));
    println!("part 2: {}", day2_part2(&s));
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

fn sort_chars(w: &str) -> String {
    let mut cv: Vec<_> = w.chars().collect();
    cv.sort_unstable();
    String::from_iter(cv)
}

fn day4(input: Option<&String>) -> i32 {
    let filename = input.expect("input filename required");

    // Today, we want to iterate over lines, so no slurp().
    let file = match File::open(&filename) {
        Ok(f) => f,
        Err(why) => {
            eprintln!("Error opening {}: {}", filename, why.description());
            return 1;
        }
    };

    let reader = BufReader::new(file);
    let mut valid = 0;
    let mut valid_part2 = 0;
    'passphrase: for line in reader.lines().map(|l| l.unwrap()) {
        // PART ONE
        // Split the line into words.  Use a HashSet to see if we've previously
        // seen it.  If not, add it to the HashSet.  Otherwise, this is an
        // invalid passphrase, and we can skip ahead to the next one.
        let mut seen = HashSet::new();
        for word in line.split(' ') {
            if seen.contains(word) {
                continue 'passphrase; // Invalid! Continue on next line.
            }
            seen.insert(word);
        }

        valid += 1; // Processed entire line, and saw no identical words.

        // PART TWO
        // for all the words we've seen, make a copy with their chars sorted.
        // then, do the same HashSet dance.  "sorted" is the vector with
        // individual words; "anagrams" is the HashSet of seen anagrams.  The
        // latter points into the former, so must be declared later, as it will
        // be dropped first (drops occur in LIFO order.)
        let mut sorted: Vec<String> = Vec::new();
        let mut anagrams = HashSet::new();
        for word in &seen {
            // sort_chars gives us ownership of a String that's copied+sorted.
            sorted.push(sort_chars(word));
        }
        seen.clear(); // Free a bit of RAM early.
        for w in &sorted {
            if anagrams.contains(w) {
                continue 'passphrase;
            }
            anagrams.insert(w);
        }

        valid_part2 += 1; // Found no identical anagrams.
    }

    println!("part1: {} valid", valid);
    println!("part2: {} valid", valid_part2);
    0
}

fn day5_read_file(filename: &str) -> Vec<i32> {
    let file = match File::open(&filename) {
        Ok(f) => f,
        Err(why) => {
            eprintln!("Error opening {}: {}", filename, why.description());
            return Vec::new();
        }
    };

    let reader = BufReader::new(file);
    reader.lines()
        .map(|l| l.unwrap().parse().expect("all offsets MUST be numeric"))
        .collect()
}

fn day5_move (offset: i32, head: usize, max_head: usize) -> Option<usize> {
    let magnitude = offset.abs() as usize;

    if offset < 0 && magnitude > head {
        None // exited left of tape
    } else if offset > 0 && (magnitude > max_head || (max_head - magnitude) < head) {
        None // exited right of tape
    } else if offset >= 0 {
        Some(head + magnitude)
    } else {
        Some(head - magnitude)
    }
}

fn day5_part1(mut tape: Vec<i32>) {
    let mut head: usize = 0;
    let mut hops: usize = 0;
    let end = tape.len() - 1;
    loop {
        let offset = tape[head]; // save offset

        tape[head] += 1; // modify cell
        hops += 1; // count the move

        match day5_move(offset, head, end) {
            None => break, // exited tape
            Some(x) => head = x, // moved on tape
        }
    }

    println!("Part 1: {}", hops);
}

fn day5_part2(mut tape: Vec<i32>) {
    let mut head: usize = 0;
    let mut hops: usize = 0;
    let end = tape.len() - 1;
    loop {
        let offset = tape[head];

        if offset >= 3 {
            tape[head] -= 1;
        } else {
            tape[head] += 1;
        }
        hops += 1;

        match day5_move(offset, head, end) {
            None => break,
            Some(x) => head = x,
        }
    }

    println!("Part 2: {}", hops);
}

fn day5(input: Option<&String>) -> i32 {
    let filename = input.expect("input filename required (tape input, 1 offset per line)");

    let tape = day5_read_file(&filename);
    if tape.len() == 0 {
        eprintln!("No tape");
        return 1;
    }

    day5_part1(tape);

    let tape = day5_read_file(&filename);
    day5_part2(tape);

    0
}

fn day6_hash_state (v: &Vec<u32>) -> String {
    // create a hashable value from a vec.
    // first, format it uniquely
    // then, join all the formats into a string
    v.iter().map(|n| format!("{}~", n)).collect()
}

fn day6_redistribute(banks: &mut Vec<u32>) {
    let len = banks.len();
    if len > std::u32::MAX as usize {
        panic!("len {} > u32::MAX", len);
    }
    let max = *banks.iter().max().unwrap();
    // find the first bank with the max number of items
    let mut at = banks.iter().enumerate()
        .find(|e| *e.1 == max).unwrap().0;

    // redistribute items in that bank
    let mut max_moves = len + 3; // runaway loop breaker
    let mut rest = banks[at];
    let segment = (f64::from(rest) / f64::from(len as u32)).ceil() as u32;
    banks[at] = 0; // empty full bank, keep our place...
    while rest > 0 {
        at = (at + 1) % len; // move to next bank

        // distribute a chunk into this bank
        let moved = cmp::min(segment, rest);
        banks[at] += moved;
        rest -= moved;

        max_moves -= 1;
        if max_moves == 0 {
            panic!("exhausted max moves, moving {} by {}", max, segment);
        }
    }
}

fn day6(input: Option<&String>) -> i32 {
    let data = slurp(input);
    if let Err(_e) = data {
        eprintln!("Expected 1 line of memory bank input, e.g. '2 3 7 1'");
        return 1;
    }
    let s: String = data.unwrap();

    let mut banks: Vec<u32> = s.split_whitespace()
        .map(|n| n.parse().expect("cell sizes MUST be u32 values"))
        .collect();
    let mut states: Vec<String> = Vec::new();

    // save initial state
    states.push(day6_hash_state(&banks));

    loop {
        day6_redistribute(&mut banks);

        let next_state = day6_hash_state(&banks);
        if let Some(i) = states.iter().enumerate().find(|e| *e.1 == next_state) {
            println!("part 2: {}", states.len() - i.0);
            break;
        }
        states.push(next_state); // push new state

        // Infinite loop breaker.  Lucky: picked arbitrarily, and never hit.
        if states.len() > 10240 {
            panic!("self destruct button pressed");
        }
    }

    // There's no "+1" needed because we pushed the initial state in.  If the
    // first redistribution yielded a duplicate state, we exited the loop with
    // states.len() == 1, and the duplicate state unpushed.
    println!("part 1: {}", states.len());
    0
}

fn day7(input: Option<&String>) -> i32 {
    // today, we're going to slurp.  owning the entire file data is going to
    // simplify things, at the price of some RAM.
    let data = slurp(input);
    if let Err(_e) = data {
        eprintln!("expected a file in 'w (42) -> x, y, z' format");
        return 1;
    };
    let s = data.unwrap();

    // as usual, nobody specified the limits on program names. let's accept
    // "unicode non-whitespace" even if that probably means other control chars.
    let line_re = Regex::new(r"(?m)^(\S+) \(\d+\)(?: -> (.*))?$").unwrap();
    let support_re = Regex::new(r",\s*").unwrap();
    let mut programs: HashMap<&str, bool> = HashMap::new();
    for line in line_re.captures_iter(&s) {
        // insert this program if it's not there.
        // can't use line[1], because "program" must outlive "line".
        let program = line.get(1).unwrap().as_str();
        if ! programs.contains_key(program) {
            programs.insert(program, false);
        }

        // mark all supported programs as seen and supported
        line.get(2)
            .and_then(|supported| {
                support_re.split(supported.as_str().trim())
                    .for_each(|program| {
                        programs.insert(program, true);
                    });
                // must return this result to be thrown away
                // because Option doesn't have for_each
                Some(supported)
            });
    }

    // find the unsupported program
    for (program, supported) in &programs {
        if ! supported {
            println!("part 1: {}", program);
        }
    }

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
    // I see there's a 25th day's puzzle.
    // We'll have to delete this eventually, but for now, <3
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
        4 => day4(args.get(2)),
        5 => day5(args.get(2)),
        6 => day6(args.get(2)),
        7 => day7(args.get(2)),
        8...24 => no_day(day),
        25 => christmas_day(),
        _ => never_day(),
    }
}

fn main() {
    let status = real_main();
    std::process::exit(status);
}
