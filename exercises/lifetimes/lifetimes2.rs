// lifetimes2.rs
// Make this code compile by annotating take_two_return_one with appropriate lifetimes.
// Execute `rustlings hint lifetimes2` for hints :)

// I AM NOT DONE

fn take_two_return_one(a: &u32, b: &u32) -> &u32 {
    b
}

fn main() {
    let long_live = 12;
    
    let z: &u32 = {
        let short_live = 42;
        take_two_return_one(&short_live, &long_live)
    };
}