import sys
import time

def ack(m, n):
    if m == 0:
        ans = n+1
    elif n == 0:
        ans = ack(m-1, 1)
    else:
        ans = ack(m-1, ack(m, n-1))
    return ans

# Python has a stack limit. By default its quite concervative. In order to do 4,1 in Ackermann need to increase the limit.
sys.setrecursionlimit(50000)

start = time.time()
answer = ack(4,1)
end = time.time()
print('Answer: %d Time: %d' % (answer, end-start))

