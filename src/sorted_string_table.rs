pub struct SortedStringTable {
    elements: Vec<String>,
}

impl SortedStringTable {
    fn push_copy(&mut self, element: &str) {
        self.elements.push(element.to_string());
    }

    fn push_move(&mut self, element: String) {
        self.elements.push(element);
    }
    fn pop(&mut self) -> Option<String> {
        self.elements.pop()
    }
    fn find(&self, element: &str) -> Option<String> {
        Some(element.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop_element_to_sst() {
        let mut sst = SortedStringTable { elements: vec!["azer".to_string()] };
        sst.push_copy("Hello !");
        assert_eq!(sst.pop(), Some("Hello !".to_string()));
    }
}
