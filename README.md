# advent2017-rs

The [advent of code, 2017.](https://adventofcode.com/2017)

In Rust, because I am oxidizing.

# A Small Rant

I noticed the AoC site has a working HTTPS certificate, but does not use HSTS,
and the OAuth redirect is to the insecure site.

This potentially leaks my GitHub identity to my ISP, AoC’s ISP, and any
state-level actor who may have tapped either one of them.
