pub struct SortedStringTable {
    elements: Vec<String>,
    
}

impl SortedStringTable {
    fn push(&mut self, element: &str) {
        let copy = element.to_string();
        self.elements.push(copy);
    }

    fn pop(&mut self) -> Option<String> {
        self.elements.pop()
    }

    fn find(&self, element: &str) -> Option<String> {
        Some(element.to_string())
    }
}

#[cfg(test)]
mod sst_tests {
    use super::*;

    #[test]
    fn push_pop_element_to_sst() {
        let mut sst = SortedStringTable { elements: vec!["azer".to_string()] };
        sst.push_copy("Hello !");
        assert_eq!(sst.pop(), Some("Hello !".to_string()));
    }
}
