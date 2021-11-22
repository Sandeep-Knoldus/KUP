#[cfg(test)]
mod tests {
    pub use crate::recursive_op::nth_element::nth_number;
    pub use crate::recursive_op::nth_element::List::{Const, Space};
    pub use crate::recursive_op::odd_number::first_odd_number;
    pub use crate::recursive_op::odd_number::List::{Constructor, Nothing};
    pub use crate::recursive_op::repeat_number::first_repeat;
    pub use crate::recursive_op::repeat_number::List::{Cons, Nil};
    pub use crate::recursive_op::second_repeat_numbers::second_repeat_element;
    pub use crate::recursive_op::second_repeat_numbers::List::{None, Sans};

    #[test]
    fn first_repeated_searching_success() {
        env_logger::init();
        let given_elements = Cons(
            1,
            Box::new(Cons(
                22,
                Box::new(Cons(
                    21,
                    Box::new(Cons(4, Box::new(Cons(5, Box::new(Cons(5, Box::new(Nil))))))),
                )),
            )),
        );
        assert_eq!(first_repeat(-1, given_elements), 5);
    }
    #[test]
    fn first_repeated_searching_successfully() {
        let given_elements = Cons(
            1,
            Box::new(Cons(
                1,
                Box::new(Cons(
                    2,
                    Box::new(Cons(3, Box::new(Cons(5, Box::new(Cons(5, Box::new(Nil))))))),
                )),
            )),
        );
        assert_eq!(first_repeat(-1, given_elements), 1);
    }

    #[test]
    fn second_repeated_searching_success() {
        let given_elements = Sans(
            1,
            Box::new(Sans(
                21,
                Box::new(Sans(
                    21,
                    Box::new(Sans(
                        4,
                        Box::new(Sans(5, Box::new(Sans(5, Box::new(None))))),
                    )),
                )),
            )),
        );
        assert_eq!(second_repeat_element(-1, given_elements), 5);
    }
    #[test]
    fn second_repeated_searching_successfully() {
        let given_elements = Sans(
            1,
            Box::new(Sans(
                1,
                Box::new(Sans(
                    2,
                    Box::new(Sans(
                        3,
                        Box::new(Sans(3, Box::new(Sans(5, Box::new(None))))),
                    )),
                )),
            )),
        );
        assert_eq!(second_repeat_element(-1, given_elements), 3);
    }

    #[test]
    fn third_odd_number_success() {
        let given_elements = Constructor(
            1,
            Box::new(Constructor(
                3,
                Box::new(Constructor(
                    5,
                    Box::new(Constructor(
                        7,
                        Box::new(Constructor(9, Box::new(Constructor(11, Box::new(Nothing))))),
                    )),
                )),
            )),
        );
        assert_eq!(first_odd_number(given_elements), 5);
    }
    #[test]
    fn third_odd_number_successfully() {
        let given_elements = Constructor(
            34,
            Box::new(Constructor(
                57,
                Box::new(Constructor(
                    5,
                    Box::new(Constructor(
                        12,
                        Box::new(Constructor(9, Box::new(Constructor(11, Box::new(Nothing))))),
                    )),
                )),
            )),
        );
        assert_eq!(first_odd_number(given_elements), 9);
    }

    #[test]
    fn nth_element_success() {
        let given_elements = Const(
            1,
            Box::new(Const(
                3,
                Box::new(Const(
                    5,
                    Box::new(Const(
                        7,
                        Box::new(Const(9, Box::new(Const(11, Box::new(Space))))),
                    )),
                )),
            )),
        );
        assert_eq!(nth_number(0, 0, given_elements), 1);
    }
    #[test]
    fn nth_element_successfully() {
        let given_elements = Const(
            1,
            Box::new(Const(
                3,
                Box::new(Const(
                    5,
                    Box::new(Const(
                        7,
                        Box::new(Const(9, Box::new(Const(11, Box::new(Space))))),
                    )),
                )),
            )),
        );
        assert_eq!(nth_number(0, 3, given_elements), 7);
    }
}
