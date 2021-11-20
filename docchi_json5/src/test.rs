
#[cfg(test)]
mod tests {


    #[test]
    fn it_works() {
        let s = r#"{ a : 100, b : "abc" }"#.to_string();

        crate::de::from_str(&s).unwrap();
    }
}
