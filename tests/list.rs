use tt_call::tt_call;
use tt_data_structures::*;

macro_rules! assert_generated {
    {
        expected = [{ $($expected:tt)* }]
        $($name:ident = [{ $($value:tt)* }])*
    } => {
        {
            let expected = stringify!($($expected)*);
            let actual = stringify!(
                $($name = [{ $($value)* }])*
            );
            assert_eq!(expected, actual);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn list_create() {
        tt_call! {
            macro = [{ tt_list_create }]
            ~~> assert_generated! {
                expected = [{
                    list = [{ tt_list }]
                }]
            }
        };
    }

    #[test]
    fn is_list_with_empty() {
        tt_call! {
            macro = [{ tt_is_list }]
            input = [{ tt_list }]
            ~~> assert_generated! {
                expected = [{
                    is_list = [{ true }]
                }]
            }
        };
    }

    #[test]
    fn is_list_with_not_empty() {
        tt_call! {
            macro = [{ tt_is_list }]
            input = [{ tt_list [{ foo }] [{ {bar} 1.0 (baz) }] }]
            ~~> assert_generated! {
                expected = [{
                    is_list = [{ true }]
                }]
            }
        };
    }

    #[test]
    fn is_list_with_not_a_list() {
        tt_call! {
            macro = [{ tt_is_list }]
            input = [{ tt_list foo }]
            ~~> assert_generated! {
                expected = [{
                    is_list = [{ false }]
                }]
            }
        };
    }

    #[test]
    fn list_is_empty_with_empty() {
        tt_call! {
            macro = [{ tt_list_is_empty }]
            input = [{ tt_list }]
            ~~> assert_generated! {
                expected = [{
                    list_is_empty = [{ true }]
                }]
            }
        };
    }

    #[test]
    fn list_is_empty_with_not_empty() {
        tt_call! {
            macro = [{ tt_list_is_empty }]
            input = [{ tt_list [{ foo }] }]
            ~~> assert_generated! {
                expected = [{
                    list_is_empty = [{ false }]
                }]
            }
        };
    }

    #[test]
    fn list_push_front() {
        tt_call! {
            macro = [{ tt_list_push_front }]
            list = [{ tt_list [{ foo }] }]
            item = [{ bar }]
            ~~> assert_generated! {
                expected = [{
                    list = [{ tt_list [{ bar }] [{ foo }] }]
                }]
            }
        };
    }

    #[test]
    fn list_push_back() {
        tt_call! {
            macro = [{ tt_list_push_back }]
            list = [{ tt_list [{ foo }] }]
            item = [{ bar }]
            ~~> assert_generated! {
                expected = [{
                    list = [{ tt_list [{ foo }] [{ bar }] }]
                }]
            }
        };
    }

    #[test]
    fn list_pop_front() {
        tt_call! {
            macro = [{ tt_list_pop_front }]
            input = [{ tt_list [{ foo }] [{ bar }] }]
            ~~> assert_generated! {
                expected = [{
                    list = [{ tt_list [{ bar }] }]
                }]
            }
        };
    }

    #[test]
    fn list_pop_back() {
        tt_call! {
            macro = [{ tt_list_pop_back }]
            input = [{ tt_list [{ foo }] [{ bar }] }]
            ~~> assert_generated! {
                expected = [{
                    list = [{ tt_list [{ foo }] }]
                }]
            }
        };
    }

    #[test]
    fn list_front() {
        tt_call! {
            macro = [{ tt_list_front }]
            input = [{ tt_list [{ foo }] [{ bar }] }]
            ~~> assert_generated! {
                expected = [{
                    front = [{ foo }]
                }]
            }
        };
    }

    #[test]
    fn list_back() {
        tt_call! {
            macro = [{ tt_list_back }]
            input = [{ tt_list [{ foo }] [{ bar }] }]
            ~~> assert_generated! {
                expected = [{
                    back = [{ bar }]
                }]
            }
        };
    }
}
