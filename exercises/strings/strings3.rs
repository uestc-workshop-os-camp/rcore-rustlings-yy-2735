// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

// ~I AM NOT DONE

fn trim_me(input: &str) -> String {
    let trimmed_str = input.trim();
    trimmed_str.to_string()
}

fn compose_me(input: &str) -> String {
    let composed_str = input.to_string() + " world!";
    composed_str
}

fn replace_me(input: &str) -> String {
    let replaced_str = input.replace("cars", "balloons");
    replaced_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
