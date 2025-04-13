#[cfg(test)]
mod tests {
    use crate::generate_result;

    #[test]
    fn check_if_else_cond() {
        let contents = r#"
        func main() u32 {
            if 5 > 2 {
                return 1
            } else {
                return 0
            }
        }
        "#;

        assert_eq!(1, generate_result(contents).unwrap());
    }

    #[test]
    fn check_mult_if_else_cond() {
        let contents = r#"
        func main() u32 {
            let u32 a = 2
            if a == 0 {
                return 1
            } else if a == 1 {
                return 2
            } else if a == 2 {
                return 3
            } else {
                return 0
            }
        }
        "#;

        assert_eq!(3, generate_result(contents).unwrap());
    }

    #[test]
    fn check_if_cond() {
        let contents = r#"
        func main() u32 {
            if 5 > 2 {
                return 1
            }
            return 0
        }
        "#;

        assert_eq!(1, generate_result(contents).unwrap());
    }

    #[test]
    fn check_if_else_if_cond() {
        let contents = r#"
        func main() u32 {
            if 0 > 2 {
                return 1
            } else if 2 > 0 {
                return 2
            }
            return 0
        }
        "#;

        assert_eq!(2, generate_result(contents).unwrap());
    }

    #[test]
    fn check_if_false_cond() {
        let contents = r#"
        func main() u32 {
            if 0 > 2 {
                return 1
            }  
            return 0
        }
        "#;

        assert_eq!(0, generate_result(contents).unwrap());
    }

    #[test]
    fn check_mult_if_cond() {
        let contents = r#"
        func main() u32 {
            if 0 > 2 {
                return 1
            }  

            if 0 < 2 {
                return 2
            }  
            return 0
        }
        "#;

        assert_eq!(2, generate_result(contents).unwrap());
    }
}
