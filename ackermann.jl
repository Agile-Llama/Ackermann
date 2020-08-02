import Pkg; 
Pkg.add("Memoize")

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

using Memoize
@memoize ack3(m::Integer, n::Integer) = m == 0 ? n + 1 : n == 0 ? ack3(m - 1, 1) : ack3(m - 1, ack3(m, n - 1))

# For benchmarking.
@time begin
    println("ack results: ")
    ack(4, 1)
end

@time begin
    println("ack3 results: ")
    ack3(4, 2)
end
