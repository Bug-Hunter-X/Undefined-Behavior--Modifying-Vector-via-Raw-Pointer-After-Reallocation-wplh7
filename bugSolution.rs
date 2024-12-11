fn main() {
    let mut v = vec![1, 2, 3];
    //Instead of using raw pointers, use safe methods
    v[0] = 10; 
    println!("{:?}", v);
}