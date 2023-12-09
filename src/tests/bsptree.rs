#[cfg(test)]
mod test {
    use crate::data_structures::bsptree::*;

    #[test]
    fn bs() {
        let mut tree = BSPTree::new(Rectangle::new(0, 0, 64, 64));

        tree.insert(2);
        tree.insert(3);
        tree.insert(4);

        tree.print(0);
    }
}
