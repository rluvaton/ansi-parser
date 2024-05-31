use std::iter::Iterator;

// https://stackoverflow.com/a/78533644/5923666
pub trait ComposeByIterator: Iterator {
    fn compose<F, OutputItem, OutputIterator>(self, f: F) -> OutputIterator
    where
        Self: Sized,
        OutputIterator: Iterator<Item = OutputItem>,
        F: Fn(Self) -> OutputIterator,
    {
        f(self)
    }
}
impl<I: Iterator> ComposeByIterator for I {}

#[cfg(test)]
mod tests {
    use super::*;
    use genawaiter::{rc::gen, yield_};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_allow_changing_the_input_type() {
        let input = vec![1, 2, 3, 4, 5];
        let output: Vec<String> = input
            .iter()
            .compose(|iter| iter.map(|x| x.to_string()))
            .collect();

        let expected = vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
        ];
        assert_eq!(output, expected);
    }

    #[test]
    fn should_allow_multiple_compositions() {
        let input = vec![1, 2, 3, 4, 5];
        let output: Vec<String> = input
            .iter()
            .compose(|iter| iter.map(|x| x.to_string()))
            .compose(|iter| iter.map(|x| x + "!"))
            .collect();

        let expected = vec![
            "1!".to_string(),
            "2!".to_string(),
            "3!".to_string(),
            "4!".to_string(),
            "5!".to_string(),
        ];
        assert_eq!(output, expected);
    }

    #[test]
    fn should_allow_multiple_compositions_that_change_type() {
        #[derive(Debug, PartialEq)]
        struct MyStruct {
            value: String,
        }
        let input = vec![1, 2, 3, 4, 5];
        let output: Vec<MyStruct> = input
            .iter()
            .compose(|iter| iter.map(|x| x.to_string()))
            .compose(|iter| iter.map(|x| MyStruct { value: x }))
            .collect();

        let expected = vec![
            MyStruct {
                value: "1".to_string(),
            },
            MyStruct {
                value: "2".to_string(),
            },
            MyStruct {
                value: "3".to_string(),
            },
            MyStruct {
                value: "4".to_string(),
            },
            MyStruct {
                value: "5".to_string(),
            },
        ];
        assert_eq!(output, expected);
    }

    #[test]
    fn should_support_gen() {
        fn wrap_with_option<Input, I: Iterator<Item = Input>>(
            iter: I,
        ) -> impl Iterator<Item = Option<Input>> {
            return gen!({
                for item in iter {
                    yield_!(Some(item));
                }
            })
            .into_iter();
        }

        let input: Vec<i32> = vec![1, 2, 3, 4, 5];
        let output: Vec<Option<&i32>> = input.iter().compose(wrap_with_option).collect();

        let expected = vec![Some(&1), Some(&2), Some(&3), Some(&4), Some(&5)];
        assert_eq!(output, expected);
    }
}
