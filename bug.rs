fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 6; 
    let z = &mut x; // This will cause a compile-time error
    *z = 7; 
}