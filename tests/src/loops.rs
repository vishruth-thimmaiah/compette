#[cfg(test)]
mod tests {
    use crate::generate_result;

    #[test]
    fn test_loop() {
        let content = r#"
        func main() u32 {
            let u32! a = 0
            loop a < 10 {
                a = a + 1
            }
            return a
        }
        "#;

        assert_eq!(10, generate_result(content).unwrap());
    }

    #[test]
    fn test_loop_return() {
        let content = r#"
        func main() u32 {
            let u32! a = 0
            loop {
                a = a + 1
                return a
            }
            return 0
        }
        "#;

        assert_eq!(1, generate_result(content).unwrap());
    }

    #[test]
    fn test_loop_nested() {
        let content = r#"
        func main() u32 {
            let u32! a = 0
            loop {
                a = a + 1
                if a > 10 {
                    break
                }
            }
            return a
        }
        "#;

        assert_eq!(11, generate_result(content).unwrap());
    }
}
