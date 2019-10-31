/// Creates an empty list. <sup>**[tt-call]**</sup>
///
/// # Input
/// * None
///
/// # Output
/// * `list = [{ tt_list }]`
#[macro_export]
macro_rules! tt_list_create {
    {
        $caller:tt
    } => {
        tt_call::tt_return! {
            $caller
            list = [{ tt_list }]
        }
    }
}

/// Predicate that accepts tokens and determines whether it is a list.
/// <sup>**[tt-call]**</sup>
///
/// # Input
/// * `input = [{` tokens `}]`
///
/// # Output
/// * `is_list = [{` true or false `}]`
#[macro_export]
macro_rules! tt_is_list {
    // validate the list format
    {
        $caller:tt
        input = [{ tt_list $( [{ $($item:tt)* }] )* }]
    } => {
        tt_call::tt_return! {
            $caller
            is_list = [{ true }]
        }
    };

    // otherwise not a list
    {
        $caller:tt
        input = [{ $($other:tt)* }]
    } => {
        tt_call::tt_return! {
            $caller
            is_list = [{ false }]
        }
    }
}

/// Predicate that accepts a list and determines whether it is empty. <sup>**[tt-call]**</sup>
///
/// # Input
/// * `input = [{` a list `}]`
///
/// # Output
/// * `list_is_empty = [{` true or false `}]`
#[macro_export]
macro_rules! tt_list_is_empty {
    // just the list indicator
    {
        $caller:tt
        input = [{ tt_list }]
    } => {
        tt_call::tt_return! {
            $caller
            list_is_empty = [{ true }]
        }
    };

    // list with some contents
    {
        $caller:tt
        input = [{ tt_list $( [{ $($list:tt)* }] )+ }]
    } => {
        tt_call::tt_return! {
            $caller
            list_is_empty = [{ false }]
        }
    }
}

/// Pushes an item to the front of a list. <sup>**[tt-call]**</sup>
///
/// # Input
/// * `list = [{` a list `}]`
/// * `item = [{` tokens `}]`
///
/// # Output
/// * `list = [{` the new list }]`
#[macro_export]
macro_rules! tt_list_push_front {
    {
        $caller:tt
        list = [{ tt_list $( [{ $($list:tt)* }] )* }]
        item = [{ $($item:tt)* }]
    } => {
        tt_call::tt_return! {
            $caller
            list = [{ tt_list [{ $($item)* }] $( [{ $($list)* }] )* }]
        }
    }
}

/// Pushes an item to the back of a list. <sup>**[tt-call]**</sup>
///
/// # Input
/// * `list = [{` a list `}]`
/// * `item = [{` tokens `}]`
///
/// # Output
/// * `list = [{` the new list `}]`
#[macro_export]
macro_rules! tt_list_push_back {
    {
        $caller:tt
        list = [{ tt_list $( [{ $($list:tt)* }] )* }]
        item = [{ $($item:tt)* }]
    } => {
        tt_call::tt_return! {
            $caller
            list = [{ tt_list $( [{ $($list)* }] )* [{ $($item)* }] }]
        }
    }
}

/// Pops an item from the front of a list. <sup>**[tt-call]**</sup>
///
/// # Input
/// * `list = [{` a list `}]`
///
/// # Output
/// * `list = [{` the new list `}]`
#[macro_export]
macro_rules! tt_list_pop_front {
    {
        $caller:tt
        input = [{ tt_list [{ $($front:tt)* }] $( [{ $($rest:tt)* }] )* }]
    } => {
        tt_call::tt_return! {
            $caller
            list = [{ tt_list $( [{ $($rest)* }] )* }]
        }
    }
}

/// Pops an item from the back of a list. <sup>**[tt-call]**</sup>
///
/// # Input
/// * `list = [{` a list `}]`
///
/// # Output
/// * `list = [{` the new list `}]`
#[macro_export]
macro_rules! tt_list_pop_back {
    {
        $caller:tt
        input = [{ tt_list $( [{ $($items:tt)* }] )+ }]
    } => {
        $crate::tt_list_pop_back_impl! {
            $caller
            list = [{ $( [{ $($items)* }] )+ }]
            output = [{ tt_list }]
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! tt_list_pop_back_impl {
    // on the last item
    {
        $caller:tt
        list = [{ $item:tt }]
        output = [{ $($output:tt)* }]
    } => {
        tt_call::tt_return! {
            $caller
            list = [{ $($output)* }]
        }
    };

    // more than one item left
    {
        $caller:tt
        list = [{ $first:tt $($rest:tt)+ }]
        output = [{ $($output:tt)* }]
    } => {
        $crate::tt_list_pop_back_impl! {
            $caller
            list = [{ $($rest)+ }]
            output = [{ $($output)* $first }]
        }
    };
}

/// Accesses the front of a list. <sup>**[tt-call]**</sup>
///
/// # Input
/// * `list = [{` a list `}]`
///
/// # Output
/// * `front = [{` the first item in the list `}]`
#[macro_export]
macro_rules! tt_list_front {
    {
        $caller:tt
        input = [{ tt_list [{ $($front:tt)* }] $( [{ $($rest:tt)* }] )* }]
    } => {
        tt_call::tt_return! {
            $caller
            front = [{ $($front)* }]
        }
    }
}

/// Accesses the back of a list. <sup>**[tt-call]**</sup>
///
/// # Input
/// * `list = [{` a list `}]`
///
/// # Output
/// * `back = [{` the last item in the list `}]`
#[macro_export]
macro_rules! tt_list_back {
    // only one item
    {
        $caller:tt
        input = [{ tt_list [{ $($item:tt)* }] }]
    } => {
        tt_call::tt_return! {
            $caller
            back = [{ $($item)* }]
        }
    };

    // pop items off until we get the last one
    {
        $caller:tt
        input = [{ tt_list [{ $($front:tt)* }] $( [{ $($rest:tt)* }] )* }]
    } => {
        tt_list_back! {
            $caller
            input = [{ tt_list $( [{ $($rest)* }] )* }]
        }
    }
}

/// Reverses a list. <sup>**[tt-call]**</sup>
///
/// # Input
/// * `list = [{` a list `}]`
///
/// # Output
/// * `list = [{` the reversed list `}]`
#[macro_export]
macro_rules! tt_list_reverse {
    {
        $caller:tt
        input = [{ tt_list $($items:tt)* }]
    } => {
        $crate::tt_list_reverse_impl! {
            $caller
            input = [{ $($items)* }]
            output = [{ }]
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! tt_list_reverse_impl {
    {
        $caller:tt
        input = [{ [{ $($front:tt)* }] $( [{ $($rest:tt)* }] )* }]
        output = [{ $($output:tt)* }]
    } => {
        $crate::tt_list_reverse_impl! {
            $caller
            input = [{ $( [{ $($rest)* }] )* }]
            output = [{ [{ $($front)* }] $($output)* }]
        }
    };

    {
        $caller:tt
        input = [{ }]
        output = [{ $($output:tt)* }]
    } => {
        tt_call::tt_return! {
            $caller
            list = [{ tt_list $($output)* }]
        }
    }
}
