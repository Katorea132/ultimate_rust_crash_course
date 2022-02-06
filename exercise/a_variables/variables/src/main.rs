const STARTING_MISSILES: i32 = 8;

const READY_AMOUNT: i32 = 2;

fn main() {

    // let (bunnies, carrots) = (8, 50); Multiple assignment pattern with tuples.

    // Variables are inmutable by default in Rust.
    // let mut bunnies: i32 = 2; // Rust can determine the type of variable automatically.

    // The constant is even immutable-r, they go all in upper case and snake case (screaming snake case)
    // the keyword const is used instead of let
    // the type annotation is required
    // the value must be a constant expression determinable at compile time.
    // const WARP_FACTOR: f64 = 9.9;
    // These can be placed outside a function scope in a module scope and used anywhere I want.
    // Really fast.

    // let mut missiles: i32 = STARTING_MISSILES;
    // let ready:  = READY_AMOUNT;
    let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);

    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;

    println!("{} missiles left", missiles - ready);

}
