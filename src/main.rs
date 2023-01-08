fn main() {
    let mut user_id = [0, 1, 2, 3, 4, 5, 6];

    let borrow_user_id = &mut user_id[..];

    for id in &mut borrow_user_id[..] {
        *id *= 2;
    }

    println!("borrow_user_id: {:?}", user_id);
}
