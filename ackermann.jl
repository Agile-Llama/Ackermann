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

# For benchmarking.
@time begin
    ack(4, 1)
    end
