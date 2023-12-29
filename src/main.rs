fn main() {

    let proceed: bool = true;

    if proceed {
        println!("proceeding");
    } else {
        println!("not proceeding");
    }

    let height: i32 = 190;
    if height > 180 {
        println!("tall");
    } else if height > 170 {
        println!("average");
    } else {
        println!("short");
    }

}
