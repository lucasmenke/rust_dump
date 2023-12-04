fn main() {
    // STACK
    // stack only data gets copied
    let x: u8 = 5;
    let y: u8 = x;
    println!("x: {} {:p}\ny: {} {:p}", x, &x, y, &y);

    // HEAP
    // data on the heap only gets referenced via pointers
    let s1 = String::from("test"); // each heap data only has one owner -> s1
    println!("s1: {} {:p}", &s1, &s1);
    let s2 = s1; // ownership moves from s1 to s2 -> s1 gets dropped
    // rust doesn't do shallow copies -> avoid dangling pointers
    println!("s2: {} {:p}", s2, &s2); // moving values creates new pointer -> prevent double freeing memory 
    let s3 = s2.clone(); // data on heap gets cloned -> s2 & s3 point to different memory addresses
    println!("s3: {} {:p}", s3, &s3);
}
