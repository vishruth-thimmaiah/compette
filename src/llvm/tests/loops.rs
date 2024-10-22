#[cfg(test)]
mod tests {
    use crate::llvm::tests::general::generate_result;

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
}
