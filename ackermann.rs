use std::collections::HashMap;
use std::cmp::max;

// Naive approach
fn ack(m: isize, n: isize) -> isize {
    if m == 0 {
        n + 1
    } else if n == 0 {
        ack(m - 1, 1)
    } else {
        ack(m - 1, ack(m, n - 1))
    }
}

// More optimzed approach using a cache
fn ack_with_cache(m: u64, n: u64) -> u64 {
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();

    _ack_with_cache(&mut cache, m, n)
}

fn _ack_with_cache(cache: &mut HashMap<(u64, u64), u64>, m: u64, n: u64) -> u64 {
    match (m, n) {
        (0, n) => n + 1,
        (1, n) => n + 2,
        (m, 0) => _ack_with_cache(cache, m - 1, 1),
        (m, 1) => {
            let n = _ack_with_cache(cache, m - 1, 1);
            _ack_with_cache(cache, m - 1, n)
        }
        (m, n) => {
            if cache.contains_key(&(m, n)) {
                *cache.get(&(m, n)).unwrap()
            } else {
                let s = _ack_with_cache(cache, m, n - 2);
                let t = _ack_with_cache(cache, m, n - 1);

                let res = (s..(t + 1)).fold(0, |acc, x| _ack_comparator(cache, m, acc, x));
                cache.insert((m, n), res);
                res
            }
        }
    }
}

fn _ack_comparator(cache: &mut HashMap<(u64, u64), u64>, m: u64, acc: u64, x: u64) -> u64 {
    let c = _ack_with_cache(cache, m - 1, x);
    max(acc, c)
}

fn main() {
    use std::time::Instant;

    //let before_naive = Instant::now();
    //let a = ack(4, 1);
    //println!("Naive Elapsed time: {:.2?}", before_naive.elapsed());
    //println!("Answer: {}", a); 

    let before = Instant::now();
    let b = ack_with_cache(4, 2);
    println!("Optimzed Elapsed time: {:.2?}", before.elapsed());
    println!("Answer: {}", b); 
}