fn main() {
    let vec = vec![vec![1,2,3,4],vec![4,5,6]];
    let flat:Vec<_> = vec.into_iter().flatten().collect();
    println!("{:?}", flat);

} 