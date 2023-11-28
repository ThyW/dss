#[cfg(test)]
mod tests {
    use crate::data_structures::gap_buffer::GapBuffer;

    #[test]
    fn gap_buffer_show() {
        let gb = GapBuffer::default();

        assert_eq!(gb.to_string(), "")
    }

    #[test]
    fn gb_inser_one() {
        let mut gb = GapBuffer::default();

        gb.insert_byte(b'h');
        gb.insert_byte(b'e');
        gb.insert_byte(b'l');
        gb.insert_byte(b'l');
        gb.insert_byte(b'o');
        gb.insert_byte(b' ');
        gb.insert_char('w');
        gb.insert_char('o');
        gb.insert_char('r');
        gb.insert_char('l');
        gb.insert_char('d');

        assert_eq!(&gb.buffer()[0..11], b"hello world");
        assert_eq!(&gb.buffer()[12..], &[0; 20]);
        assert_eq!(gb.to_string(), "hello world");
    }

    #[test]
    fn gb_insert_many() {
        let mut gb = GapBuffer::default();

        gb.insert(b"hello world");

        assert_eq!(&gb.buffer()[0..11], b"hello world");
        assert_eq!(&gb.buffer()[12..], &[0; 20]);
        assert_eq!(gb.to_string(), "hello world");
    }

    #[test]
    fn gb_grow() {
        let mut gb = GapBuffer::default();

        gb.insert(b"hello world welcome to another day here");

        assert_eq!(
            &gb.buffer()[0..39],
            b"hello world welcome to another day here"
        );
        assert_eq!(&gb.buffer()[41..], &[0; 23]);
        assert_eq!(gb.to_string(), "hello world welcome to another day here");
    }

    #[test]
    fn gb_left() {
        let mut gb = GapBuffer::default();

        gb.insert(b"hello world welcome to another day here");
        assert_eq!(gb.capacity, 64);
        gb.left_by(10);

        let (l, r) = gb.gap();

        assert_eq!(&gb.buffer()[0..29], b"hello world welcome to anothe");
        assert_eq!(&gb.buffer()[r + 1..], b"r day here");
        assert_eq!(&gb.buffer()[l..r], [0; 24]);
        assert_eq!(gb.to_string(), "hello world welcome to another day here");

        gb.left_by(2);

        let (l, r) = gb.gap();

        assert_eq!(&gb.buffer()[0..27], b"hello world welcome to anot");
        assert_eq!(&gb.buffer()[r + 1..], b"her day here");
        assert_eq!(&gb.buffer()[l..r], [0; 24]);
        assert_eq!(gb.to_string(), "hello world welcome to another day here");

        gb.left_by(2);
        let (l, r) = gb.gap();

        assert_eq!(&gb.buffer()[0..25], b"hello world welcome to an");
        assert_eq!(&gb.buffer()[r + 1..], b"other day here");
        assert_eq!(&gb.buffer()[l..r], [0; 24]);
        assert_eq!(gb.to_string(), "hello world welcome to another day here");

        gb.left_by(2);
        let (l, r) = gb.gap();

        assert_eq!(&gb.buffer()[0..23], b"hello world welcome to ");
        assert_eq!(&gb.buffer()[r + 1..], b"another day here");
        assert_eq!(&gb.buffer()[l..r], [0; 24]);
        assert_eq!(gb.to_string(), "hello world welcome to another day here");

        gb.left_by(32);
        let (l, r) = gb.gap();
        assert_eq!(&gb.buffer()[l..r], [0; 24]);
        assert_eq!(
            &gb.buffer()[25..],
            b"hello world welcome to another day here"
        );

        assert_eq!(gb.to_string(), "hello world welcome to another day here");
    }

    #[test]
    fn gb_right() {
        let mut gb = GapBuffer::default();

        gb.insert(b"hello world welcome to another day here");
        assert_eq!(gb.capacity, 64);

        gb.left_by(15);
        let (l, r) = gb.gap();

        assert_eq!(&gb.buffer()[0..24], b"hello world welcome to a");
        assert_eq!(&gb.buffer()[r + 1..], b"nother day here");
        assert_eq!(&gb.buffer()[l..r], [0; 24]);
        assert_eq!(gb.to_string(), "hello world welcome to another day here");

        gb.right_by(5);
        let (l, r) = gb.gap();

        assert_eq!(&gb.buffer()[0..29], b"hello world welcome to anothe");
        assert_eq!(&gb.buffer()[r + 1..], b"r day here");
        assert_eq!(&gb.buffer()[l..r], [0; 24]);
        assert_eq!(gb.to_string(), "hello world welcome to another day here");

        gb.right_by(10);
        let (l, r) = gb.gap();

        assert_eq!(
            &gb.buffer()[0..39],
            b"hello world welcome to another day here"
        );
        assert_eq!(&gb.buffer()[r + 1..], b"");
        assert_eq!(&gb.buffer()[l..r], [0; 24]);
        assert_eq!(gb.to_string(), "hello world welcome to another day here");

        gb.right_by(10);
        let (l, r) = gb.gap();

        assert_eq!(
            &gb.buffer()[0..39],
            b"hello world welcome to another day here"
        );
        assert_eq!(&gb.buffer()[r + 1..], b"");
        assert_eq!(&gb.buffer()[l..r], [0; 24]);
        assert_eq!(gb.to_string(), "hello world welcome to another day here");
    }

    #[test]
    fn gb_delete() {
        let mut gb = GapBuffer::default();

        gb.insert(b"hello world string");

        assert_eq!(gb.to_string(), "hello world string");

        gb.delete_left(7);

        assert_eq!(gb.to_string(), "hello world");

        gb.left_by(5);

        assert_eq!(gb.to_string(), "hello world");

        gb.delete_left(20);

        assert_eq!(gb.to_string(), "world");

        gb.insert_str("hahahahahahahahhaahahhahahaahahhahahahahaahha");

        assert_eq!(
            gb.to_string(),
            "hahahahahahahahhaahahhahahaahahhahahahahaahhaworld"
        );

        gb.delete_right(10);

        assert_eq!(
            gb.to_string(),
            "hahahahahahahahhaahahhahahaahahhahahahahaahha"
        );

        assert_eq!(gb.capacity, 64);

        gb.delete_left(32);

        assert_eq!(gb.to_string(), "hahahahahahah");
        assert_eq!(gb.capacity, 64);
    }
}
