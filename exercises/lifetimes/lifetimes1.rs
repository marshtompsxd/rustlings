// lifetimes1.rs
// Remember our ColorClassicStruct? 
// Instead of using the static lifetime, try to give `name` and `hex` some non-static lifetimes!
// Execute `rustlings hint lifetimes1` for hints :)

// I AM NOT DONE

struct ColorClassicStruct {
    // TODO: use a lifetime that is not 'static here
    name: &str,
    // TODO: use a lifetime that is not 'static here
    hex: &str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        let green = ColorClassicStruct { name: "green", hex: "#00FF00" };

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }
}
