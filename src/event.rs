#![macro_escape]

#[macro_export]
macro_rules! awaitable_events {
    ($(#[$attr:meta])* events $Events:ident: $T:ty {
        $($(#[$Event_attr:meta])* static $Event:ident),+
    }) => {
        #[deriving(PartialEq, Eq, Clone, PartialOrd, Ord)]
        $(#[$attr])*
        pub struct $Events {
            value: $T,
        }

        $($(#[$Event_attr])* pub static $Event: $Events = $Events { value: std::hash::sip(stringify!($Event)) };)+

        impl $Events {
            /// Returns an empty set of flags.
            pub fn empty() -> $Events {
                $Events { value: 0 }
            }

            /// Returns the set containing all flags.
            pub fn all() -> $Events {
                $Events { value: $($value)|+ }
            }

            /// Returns the raw value of the flags currently stored.
            pub fn value(&self) -> $T {
                self.value
            }

            /// Convert from underlying bit representation, unless that
            /// representation contains value that do not correspond to a flag.
            pub fn from_value(value: $T) -> ::std::option::Option<$Events> {
                if (value & !$Events::all().value()) != 0 {
                    ::std::option::None
                } else {
                    ::std::option::Some($Events { value: value })
                }
            }

            /// Convert from underlying bit representation, dropping any value
            /// that do not correspond to flags.
            pub fn from_value_truncate(value: $T) -> $Events {
                $Events { value: value } & $Events::all()
            }

    };
    ($(#[$attr:meta])* events $Events:ident: $T:ty {
        $($(#[$Event_attr:meta])* static $Event:ident),+,
    }) => {
        awaitable_event! {
            $(#[$attr])*
            events $Events: u32 {
                $($(#[$Event_attr])* static $Event),+
            }
        }
    };
}
}

