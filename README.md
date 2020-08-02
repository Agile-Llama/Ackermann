# Ackermann
An interesting property of the Ackermann function is that the maximum stack depth needed to evaluate it (in levels of calls) is the same as the answer to the function. This means that there will be severe limits on the actual calculation that can be done, imposed by the limits of the virtual memory of your hardware.
Generally most standard(Naive) implentations of the algorithm won't make it past 4,1 without some form of seg fault or stack issue. Optimzations make it possible to do higher(deeper?).

A(4, 2) cannot possibly be computed by simple recursive application of the Ackermann function in any tractable
amount of time. Instead, shortcut formulas such as A(3, n) = 8×2n−3 are used as an optimization to complete some of the recursive calls.
A practical method of computing functions similar to Ackermann's is to use memoization of intermediate results

Python with no optimzations unable to compute 4,1 without segmentation fault. 
(4,1) With optimzations (not really a fair comparison) brings it down to under 1 second.

Initial function. Seg faulting at (4,2)
```Python
def ack(m, n):
    if m == 0:
        ans = n+1
    elif n == 0:
        ans = ack(m-1, 1)
    else:
        ans = ack(m-1, ack(m, n-1))
    return ans
```

Haskell compiled with GHC. Run with 'time ./ackermann' 
(4,1) 266.16s user 2.42s system 99% cpu 4:30.76 total
(4,2) Not fully run it, but no segfault or stack overflow occurs when running for 30mins.

```Haskell
main = print (ack 4 1)
-- Ackermann function
ack 0 n = n + 1
ack m 0 = ack (m-1) 1
ack m n = ack (m-1) (ack m (n - 1))
```

Julia Naive
(4,1) I get 5.522631 seconds compiling with 'julia ackermann.jl'
(4,2) Stackoverflow error. 
```Julia
function ack(m, n)
    if m == 0
        ans = n + 1
    elseif n == 0
        ans = ack(m-1, 1)
    else
        ans = ack(m-1, ack(m, n-1))
    end
    return ans
end
```

Julia with optimzations (using memorization)
(4,1)  0.108952 seconds (555.83 k allocations: 19.818 MiB)

Rust compiled with rustc ackermann.rs and ./ackermann
(4,1) Naive Elapsed time: 13.89s, Answer: 65533
```Rust
fn ack(m: isize, n: isize) -> isize {
    if m == 0 {
        n + 1
    } else if n == 0 {
        ack(m - 1, 1)
    } else {
        ack(m - 1, ack(m, n - 1))
    }
}
```
(4,1) Optimzed Elapsed time: 136.73ms, Answer: 65533


