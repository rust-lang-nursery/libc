/// A macro for defining #[cfg] if-else statements.
///
/// This is similar to the `if/elif` C preprocessor macro by allowing definition
/// of a cascade of `#[cfg]` cases, emitting the implementation which matches
/// first.
///
/// This allows you to conveniently provide a long list #[cfg]'d blocks of code
/// without having to rewrite each clause multiple times.
macro_rules! cfg_if {
    ($(
        if #[cfg($($meta:meta),*)] { $($it:item)* }
    ) else * else {
        $($it2:item)*
    }) => {
        __cfg_if_items! {
            () ;
            $( ( ($($meta),*) ($($it)*) ), )*
            ( () ($($it2)*) ),
        }
    }
}

macro_rules! __cfg_if_items {
    (($($not:meta,)*) ; ) => {};
    (($($not:meta,)*) ; ( ($($m:meta),*) ($($it:item)*) ), $($rest:tt)*) => {
        __cfg_if_apply! { cfg(all(not(any($($not),*)), $($m,)*)), $($it)* }
        __cfg_if_items! { ($($not,)* $($m,)*) ; $($rest)* }
    }
}

macro_rules! __cfg_if_apply {
    ($m:meta, $($it:item)*) => {
        $(#[$m] $it)*
    }
}

macro_rules! s {
    ($($(#[$attr:meta])* pub $t:ident $i:ident { $($field:tt)* })*) => ($(
        s!(it: $(#[$attr])* pub $t $i { $($field)* });
    )*);
    (it: $(#[$attr:meta])* pub union $i:ident { $($field:tt)* }) => (
        cfg_if! {
            if #[cfg(libc_union)] {
                __item! {
                    #[repr(C)]
                    #[cfg_attr(feature = "extra_traits", derive(Debug, Eq, Hash, PartialEq))]
                    $(#[$attr])*
                    pub union $i { $($field)* }
                }

                impl ::dox::Copy for $i {}
                impl ::dox::Clone for $i {
                    fn clone(&self) -> $i { *self }
                }
            }
        }
    );
    (it: $(#[$attr:meta])* pub struct $i:ident { $($field:tt)* }) => (
        __item! {
            #[repr(C)]
            #[cfg_attr(feature = "extra_traits", derive(Debug, Eq, Hash, PartialEq))]
            $(#[$attr])*
            pub struct $i { $($field)* }
        }
        impl ::dox::Copy for $i {}
        impl ::dox::Clone for $i {
            fn clone(&self) -> $i { *self }
        }
    );
}

macro_rules! s_no_extra_traits {
    ($($(#[$attr:meta])* pub $t:ident $i:ident { $($field:tt)* })*) => ($(
        s!(it: $(#[$attr])* pub $t $i { $($field)* });
    )*);
    (it: $(#[$attr:meta])* pub union $i:ident { $($field:tt)* }) => (
        cfg_if! {
            if #[cfg(libc_union)] {
                __item! {
                    #[repr(C)]
                    $(#[$attr])*
                    pub union $i { $($field)* }
                }

                impl ::dox::Copy for $i {}
                impl ::dox::Clone for $i {
                    fn clone(&self) -> $i { *self }
                }
            }
        }
    );
    (it: $(#[$attr:meta])* pub struct $i:ident { $($field:tt)* }) => (
        __item! {
            #[repr(C)]
            $(#[$attr])*
            pub struct $i { $($field)* }
        }
        impl ::dox::Copy for $i {}
        impl ::dox::Clone for $i {
            fn clone(&self) -> $i { *self }
        }
    );
}

#[allow(unused_macros)]
macro_rules! f {
    ($(pub fn $i:ident($($arg:ident: $argty:ty),*) -> $ret:ty {
        $($body:stmt);*
    })*) => ($(
        #[inline]
        #[cfg(not(cross_platform_docs))]
        pub unsafe extern fn $i($($arg: $argty),*) -> $ret {
            $($body);*
        }

        #[cfg(cross_platform_docs)]
        #[allow(dead_code)]
        pub unsafe extern fn $i($($arg: $argty),*) -> $ret {
            loop {}
        }
    )*)
}

macro_rules! __item {
    ($i:item) => {
        $i
    };
}

#[allow(unused_macros)]
macro_rules! align_const {
    ($($(#[$attr:meta])* pub const $name:ident : $t1:ty = $t2:ident { $($field:tt)* };)*) => ($(
        #[cfg(feature = "align")]
        $(#[$attr])*
        pub const $name : $t1 = $t2 {
            $($field)*
        };
        #[cfg(not(feature = "align"))]
        $(#[$attr])*
        pub const $name : $t1 = $t2 {
            $($field)*
            __align: [],
        };
    )*)
}
