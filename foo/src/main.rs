use rand::Rng;

fn main() {
    println!("Hello, world!");
    // get random int from 1 to 50
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..51);
    println!("The secret number is: {}", secret_number);
}
