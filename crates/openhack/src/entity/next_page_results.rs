#[derive(Debug, Clone)]
pub struct NextPageResults<T> {
    pub data: Vec<T>,
    pub has_next_page: bool,
}

mod impls {
    use super::*;

    impl<T> NextPageResults<T> {
        pub fn new(data: Vec<T>, has_next_page: bool) -> Self {
            Self {
                data,
                has_next_page,
            }
        }

        pub fn iter(&self) -> impl Iterator<Item = &T> {
            self.data.iter()
        }
    }
}
