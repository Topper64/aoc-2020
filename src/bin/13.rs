use std::io::{self, BufRead, BufReader};
use std::fs::File;
use num::Integer;

fn euclid<T: Integer + Copy>(a: &T, b: &T) -> (T, T, T) {
    let mut r = (*a, *b);
    let mut s = (T::one(), T::zero());
    let mut t = (T::zero(), T::one());
    while r.1 != T::zero() {
        let q = r.0.div_floor(&r.1);
        r = (r.1, r.0 - r.1 * q);
        s = (s.1, s.0 - s.1 * q);
        t = (t.1, t.0 - t.1 * q);
    }
    (r.0, s.0, t.0)
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/13.txt")?);

    // Read file
    let mut lines = reader.lines().map(|line| line.unwrap());
    let start: i128 = lines.next().unwrap().parse().unwrap();
    let line = lines.next().unwrap();
    let frequencies = line.split(',').map(|f| f.parse().ok());
    let frequencies: Vec<_> = frequencies.collect();

    // Find next bus
    let mut soonest = None;
    for freq in frequencies.iter() {
        if let Some(freq) = *freq {
            let wait = freq - start.rem_euclid(freq);
            soonest = match soonest {
                Some((w, _)) if w < wait => soonest,
                _ => Some((wait, freq)),
            }
        }
    }

    // Solve some simultaneous congruences:
    // If the ith bus is every f_i minutes, we want (minimal) x such that
    // x + i = 0 (f_i)
    // are all satisfied. The f_i are all coprime (in fact prime), so CRT will give a solution,
    // unique mod n := prod f_i
    let mut cong: Option<(i128, i128)> = None;
    for (i, freq) in frequencies.iter().enumerate() {
        let i = i as i128;
        if let Some(f) = *freq {
            let b = f - i;
            cong = match cong {
                // Simultaneously solving x = a (mod n) and x = b (mod f)
                Some((a, n)) => {
                    // Get r, s, t such that r = s n + t f = gcd(n, f)
                    let (r, s, t) = euclid(&n, &f);
                    // If the gcd is 1, the congruences can be reduced to one
                    // Specifically, x = b s n + a t f (mod n f)
                    assert_eq!(r, 1);
                    let a = b*s*n + a*t*f;
                    let n = n * f;
                    Some((a.rem_euclid(n), n))
                },

                // First congruence: nothing to do
                _ => Some((b, f)),
            }
        }
    }

    let (wait, freq) = soonest.unwrap();
    println!("Part 1: {}", wait * freq);
    let (t, _) = cong.unwrap();
    println!("Part 2: {}", t);

    Ok(())
}
