pub trait VecWrapper<T> {
    fn get_vec(&self) -> &Vec<T>;

    fn len(&self) -> usize {
        self.get_vec().len()
    }
}
