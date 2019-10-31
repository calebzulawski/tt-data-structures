#[macro_export]
macro_rules! tt_list_create {
    {
        $caller:tt
    } => {
        tt_list_indicator
    }
}

#[macro_export]
macro_rules! tt_is_list {
    // validate the list format
    {
        $caller:tt
        input = [{ $( tt_list_indicator [{ $($item:tt)* }] )* }]
    } => {
        tt_call::tt_return! {
            $caller:tt
            is_list = [{ true }]
        }
    };

    // otherwise not a list
    {
        $caller:tt
        input = [{ $($other:tt)* }]
    } => {
        tt_call::tt_return! {
            $caller:tt
            is_list = [{ false }]
        }
    }
}

#[macro_export]
macro_rules! tt_list_is_empty {
    // just the list indicator
    {
        $caller:tt
        input = [{ tt_list_indicator }]
    } => {
        tt_call::tt_return! {
            $caller:tt
            list_is_empty = [{ true }]
        }
    };

    // list with some contents
    {
        $caller:tt
        input = [{ tt_list_indicator $( [{ $($list:tt)* }] )+ }]
    } => {
        tt_call::tt_return! {
            $caller:tt
            list_is_empty = [{ false }]
        }
    }
}

#[macro_export]
macro_rules! tt_list_push_front {
    {
        $caller:tt
        list = [{ tt_list_indicator $( [{ $($list:tt)* }] )* }]
        item = [{ $($item:tt)* }]
    } => {
        tt_call::tt_return! {
            $caller
            list = [{ tt_list_indicator [{ $($item:tt)* }] $( [{ $($list:tt)* }] )* }]
        }
    }
}

#[macro_export]
macro_rules! tt_list_push_back {
    {
        $caller:tt
        list = [{ tt_list_indicator $( [{ $($list:tt)* }] )* }]
        item = [{ $($item:tt)* }]
    } => {
        tt_call::tt_return! {
            $caller
            list = [{ tt_list_indicator $( [{ $($list:tt)* }] )* [{ $($item:tt)* }] }]
        }
    }
}

#[macro_export]
macro_rules! tt_list_pop_front {
    {
        $caller:tt
        input = [{ tt_list_indicator [{ $($front:tt)* }] $( [{ $($rest:tt)* }] )* }]
    } => {
        tt_call::tt_return! {
            $caller
            list = [{ tt_list_indicator $( [{ $($rest:tt)* }] )* }]
        }
    }
}

#[macro_export]
macro_rules! tt_list_pop_back {
    {
        $caller:tt
        input = [{ tt_list_indicator $( [{ $($rest:tt)* }] )* [{ $($back:tt)* }] }]
    } => {
        tt_call::tt_return! {
            $caller
            list = [{ tt_list_indicator $( [{ $($rest:tt)* }] )* }]
        }
    }
}

#[macro_export]
macro_rules! tt_list_front {
    {
        $caller:tt
        input = [{ tt_list_indicator [{ $($front:tt)* }] $( [{ $($rest:tt)* }] )* }]
    } => {
        tt_call::tt_return! {
            $caller
            front = [{ $($front:tt)* }]
        }
    }
}

#[macro_export]
macro_rules! tt_list_back {
    {
        $caller:tt
        input = [{ tt_list_indicator $( [{ $($rest:tt)* }] )* [{ $($back:tt)* }] }]
    } => {
        tt_call::tt_return! {
            $caller
            front = [{ $($back:tt)* }]
        }
    }
}

#[macro_export]
macro_rules! tt_list_reverse {
    {
        $caller:tt
        input = [{ tt_list_indicator $($items:tt)* }]
    } => {
        tt_list_reverse_impl! {
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
        tt_list_reverse_impl! {
            $caller
            input = [{ $( [{ $($rest:tt)* }] )* }]
            output = [{ $($output)* [{ $($front)* }] }]
        }
    };

    {
        $caller:tt
        input = [{ }]
        output = [{ $($output:tt)* }]
    } => {
        tt_call::tt_return! {
            $caller
            list = [{ tt_list_indicator $($output:tt)* }]
        }
    }
}

#[macro_export]
macro_rules! tt_list_transform {
    {
        $caller:tt
        macro = [{ $($m:ident)::* }]
        list = [{ tt_list_indicator $($items:tt)* }]
        $(
            $input:ident = [{ $($tokens:tt)* }]
        )*
    } => {
        tt_list_transform_impl! {
            $caller
            input = [{ $($items)* }]
            output = [{ }]
            $(
                $input = [{ $($tokens)* }]
            )*
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! tt_list_transform_impl {
    {
        $caller:tt
        macro = [{ $($m:ident)::* }]
        input = [{ [{ $($front:tt)* }] $( [{ $($rest:tt)* }] )* }]
        output = [{ $($output:tt)* }]
        $(
            $input:ident = [{ $($tokens:tt)* }]
        )*
    } => {
        tt_list_reverse_impl! {
            $caller
            input = [{ $( [{ $($rest:tt)* }] )* }]
            output = [{
                [{
                    tt_call::tt_call! {
                        macro = [{ $($m)::* }]
                        input = [{ $($front)* }]
                        $(
                            $input = [{ $($tokens)* }]
                        )*
                    }
                }]
                $($output)*
            }]
        }
    };

    {
        $caller:tt
        input = [{ }]
        output = [{ $($output:tt)* }]
    } => {
        tt_call::tt_return! {
            $caller
            list = [{ tt_list_indicator $($output:tt)* }]
        }
    }
}
