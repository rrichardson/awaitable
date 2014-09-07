#![feature(globs)]
#![feature(macro_rules)]
#![feature(trace_macros)]
#![feature(phase)]

//use event::*;

macro_rules! awaitable_events {
    ($(#[$attr:meta])* events $Events:ident: $T:ty {
        $($(#[$Event_attr:meta])* $Event:ident),+
    }) => {
        #[deriving(PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
        $(#[$attr])*
        pub struct $Events {
            value: $T,
        }
       
        // TODO this won't compile because of running functions at static init time
        // replace this with a compile time solution
        $($(#[$Event_attr])* pub static $Event: $Events = $Events { value: ::std::hash::sip::hash(&stringify!($Event)) };)+

        impl $Events {
            /// Returns the raw value of the flags currently stored.
            pub fn value(&self) -> $T {
                self.value
            }
        }

    };
    ($(#[$attr:meta])* events $Events:ident: $T:ty {
        $($(#[$Event_attr:meta])*  $Event:ident),+,
    }) => {
        awaitable_event! {
            $(#[$attr])*
            events $Events: $T {
                $($(#[$Event_attr])*  $Event),+
            }
        }
    };
}


trace_macros!(true)

awaitable_events! {
  events MyEvents: u64 {
     EventA,
     EventB,
     EventC
  }
}


fn main() {
  println!("event a is {}", EventA.value());
}
