fn ack(m: isize, n: isize) -> isize {
    if m == 0 {
        n + 1
    } else if n == 0 {
        ack(m - 1, 1)
    } else {
        ack(m - 1, ack(m, n - 1))
    }
}
 
fn main() {
    use std::time::Instant;

    let before = Instant::now();
    let a = ack(4, 2);
    println!("Elapsed time: {:.2?}", before.elapsed());
    println!("Answer: {}", a); 


}