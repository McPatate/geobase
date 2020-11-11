pub fn hello_geobase(s: &str) -> String {
    let prefix: String = "Prefixed ".to_string();
    prefix + s
}

#[cfg(test)]
mod tests {
    use crate::hello_geobase;

    #[test]
    fn it_works() {
        assert_eq!(hello_geobase("Hello !"), "Prefixed Hello !");
    }
}
