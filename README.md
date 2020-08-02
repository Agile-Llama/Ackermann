# Ackermann

Python with no optimzations unable to compute 4,1 without segmentation fault. 
(4,1) With optimzations (not really a fair comparison) brings it down to under 1 second.

Initial function. Seg faulting above is optimzation. 
```
def ack(m, n):
    if m == 0:
        ans = n+1
    elif n == 0:
        ans = ack(m-1, 1)
    else:
        ans = ack(m-1, ack(m, n-1))
    return ans
'''

Haskell compiled with GHC. Run with 'time ./ackermann' 
(4,1) 266.16s user 2.42s system 99% cpu 4:30.76 total

Julia with no optimizations 
(4,1) I get 5.522631 seconds compiling with 'julia ackermann.jl'
(4,2) Stackoverflow error. 

