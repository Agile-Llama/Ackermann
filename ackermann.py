import sys
import time

# Python has a stack limit. By default its quite concervative. In order to do 4,1 in Ackermann need to increase the limit.
sys.setrecursionlimit(30000)

memo = {}

def ack(m, n):
    if not (m, n) in memo:
        result = (n + 1) if m == 0 else (
            ack(m-1, 1) if n == 0 else ack(m-1, ack(m, n-1)))
        memo[(m, n)] = result
    return memo[(m, n)]

start = time.time()
answer = ack(4,1)
end = time.time()
print('Answer: %d Time: %d' % (answer, end-start))

