fn main() {
    f1();
}

fn f1() {
    f2();
}
fn f2() {
    let locked = std::sync::RwLock::new(());
    let one = locked.write().unwrap();
    let two = locked.read().unwrap();
    println!("locked twice: {:?}, {:?}", one, two);
}