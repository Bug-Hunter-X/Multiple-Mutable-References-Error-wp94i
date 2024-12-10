fn main() {
    let mut x = 5;
    { // Limit scope of mutable references
        let y = &mut x;
        *y = 6; 
    }
    let z = &mut x; 
    *z = 7; 
}
