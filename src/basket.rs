pub struct Basket {
    item: Option<String>,
}

impl Basket {
    fn get(&mut self) -> Option<String> {
        self.item.take()
    }

    fn put(&mut self, item: String) {
        self.item = Some(item);
    }

    fn is_empty(&self) -> bool {
        self.item.is_none()
    }
}
