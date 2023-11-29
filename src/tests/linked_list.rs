#[cfg(test)]
mod test {

    use crate::data_structures::linked_list::*;
    #[test]
    fn ll_push_back() {
        let mut ll = LinkedList::<usize>::new();

        ll.push_back(1);
        ll.push_back(2);
        ll.push_back(3);

        assert_eq!(ll.len(), 3);

        for i in 0..3 {
            let item = ll.get(i);
            assert!(item.is_some());
            assert_eq!(item.unwrap(), &(i + 1))
        }

        ll.push_back(4);
        ll.push_back(5);
        ll.push_back(6);

        assert_eq!(ll.len(), 6);

        for i in 4..6 {
            let item = ll.get(i);
            assert!(item.is_some());
            assert_eq!(item.unwrap(), &(i + 1))
        }
    }

    #[test]
    fn ll_push_front() {
        let mut ll = LinkedList::<usize>::new();

        ll.push_front(0);
        ll.push_front(1);
        ll.push_front(2);

        assert_eq!(ll.len(), 3);

        for i in 0..3 {
            let item = ll.get(i);
            assert!(item.is_some());
            assert_eq!(item.unwrap(), &(2 - i))
        }

        ll.push_front(3);
        ll.push_front(4);
        ll.push_front(5);

        assert_eq!(ll.len(), 6);

        for i in 0..6 {
            let item = ll.get(i);
            assert!(item.is_some());
            assert_eq!(item.unwrap(), &(5 - i))
        }
    }

    #[test]
    fn ll_modify() {
        let mut ll = LinkedList::<usize>::new();

        for i in 0..10 {
            ll.push_front(10 - i);
        }

        assert_eq!(ll.len(), 10);

        for i in 0..10 {
            let x = ll.get_mut(i);
            assert!(x.is_some());
            *(x.unwrap()) *= 10;
        }

        assert_eq!(ll.len(), 10);

        for i in 0..10 {
            let x = ll.get(i);
            assert!(x.is_some());
            assert_eq!(x.unwrap(), &((i + 1) * 10));
        }
    }

    #[test]
    fn ll_pop() {
        let mut ll = LinkedList::<usize>::new();

        for i in 0..10 {
            ll.push_front(i);
        }

        assert_eq!(ll.len(), 10);

        for _ in 0..5 {
            let old = ll.pop_head();
            assert!(old.is_some());
        }

        assert_eq!(ll.len(), 5);

        for i in 0..5 {
            let item = ll.get(i);
            assert!(item.is_some());

            assert_eq!(item.unwrap(), &(4 - i));
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct TestType(i32);

    #[test]
    fn ll_pop_no_default() {
        let mut ll = LinkedList::<TestType>::new();

        for i in 0..10 {
            ll.push_front(TestType(i));
        }

        assert_eq!(ll.len(), 10);

        for _ in 0..5 {
            let old = ll.pop_head();
            assert!(old.is_some());
        }

        assert_eq!(ll.len(), 5);

        for i in 0..5 {
            let item = ll.get(i);
            assert!(item.is_some());

            assert_eq!(item.unwrap().0, (4 - i as i32));
        }
    }

    #[test]
    fn ll_pop_back() {
        let mut ll = LinkedList::<usize>::new();

        for i in 0..10 {
            ll.push_front(i);
        }

        assert_eq!(ll.len(), 10);

        for _ in 0..5 {
            let old = ll.pop_back();
            assert!(old.is_some());
        }

        assert_eq!(ll.len(), 5);

        for i in 0..5 {
            let item = ll.get(i);
            assert!(item.is_some());

            assert_eq!(item.unwrap(), &(9 - i));
        }
    }
}
