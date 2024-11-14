#[cfg(test)]
mod tests {
    use crate::llvm::tests::generate_result;

    #[test]
    fn check_main_func() {
        let contents = r#"
        func main() u32 {
            let u32 a = 6 * 3 - 1
            return a
        }
        "#;

        assert_eq!(6 * 3 - 1, generate_result(contents).unwrap());
    }

    #[test]
    fn check_mult_func() {
        let contents = r#"
        func add(a u32, b u32) u32 {
            return a + b
        }

        func main() u32 {
            let u32 a = add(2, 3)
            return a
        }
        "#;

        assert_eq!(5, generate_result(contents).unwrap());
    }

    #[test]
    fn check_mut() {
        let contents = r#"
        func main() u32 {
            let u32! a = 2
            if 5 < 6 {
                a = 4
            }
            return a
        }
        "#;

        assert_eq!(4, generate_result(contents).unwrap());
    }

    #[test]
    fn check_array() {
        let contents = r#"
        func main() u32 {
            let u32[] a = [1, 2, 3]
            return a[0]
        }
        "#;

        assert_eq!(1, generate_result(contents).unwrap());
    }

    #[test]
    fn check_array_assign() {
        let contents = r#"
        func main() u32 {
            let u32[]! a = [1, 2, 3]
            a[0] = 4
            return a[0]
        }
        "#;

        assert_eq!(4, generate_result(contents).unwrap());
    }

    #[test]
    fn check_struct() {
        let contents = r#"
        struct Point {
            x u32
            y u32
        }

        func main() u32 {
            let Point p = [ 1, 2 ]
            return p.x
        }
        "#;

        assert_eq!(1, generate_result(contents).unwrap());
    }
}
