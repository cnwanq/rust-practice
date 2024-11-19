use std::process;

fn main() {
    println!("{}", get_process_id());
}

fn get_process_id() -> u32 {
    process::id()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_process_id() {
        let id = get_process_id();
        assert!(id > 0);
    }
}
