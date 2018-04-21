#[macro_use]
extern crate getset;

use submodule::other::{Plain, Generic, Where};

// For testing `pub(super)`
mod submodule {
    // For testing `pub(in super::other)`
    pub mod other {
        #[derive(Getters, Default)]
        pub struct Plain {
            /// A doc comment.
            /// Multiple lines, even.
            #[get]
            private_accessible: usize,

            /// A doc comment.
            #[get = "pub"]
            public_accessible: usize,

            // /// A doc comment.
            // #[get = "pub(crate)"]
            // crate_accessible: usize,

            // /// A doc comment.
            // #[get = "pub(super)"]
            // super_accessible: usize,

            // /// A doc comment.
            // #[get = "pub(in super::other)"]
            // scope_accessible: usize,
        }

        #[derive(Getters, Default)]
        pub struct Generic<T: Copy + Clone + Default> {
            /// A doc comment.
            /// Multiple lines, even.
            #[get]
            private_accessible: T,

            /// A doc comment.
            #[get = "pub"]
            public_accessible: T,

            // /// A doc comment.
            // #[get = "pub(crate)"]
            // crate_accessible: usize,

            // /// A doc comment.
            // #[get = "pub(super)"]
            // super_accessible: usize,

            // /// A doc comment.
            // #[get = "pub(in super::other)"]
            // scope_accessible: usize,
        }

        #[derive(Getters, Default)]
        pub struct Where<T> where T: Copy + Clone + Default {
            /// A doc comment.
            /// Multiple lines, even.
            #[get]
            private_accessible: T,

            /// A doc comment.
            #[get = "pub"]
            public_accessible: T,

            // /// A doc comment.
            // #[get = "pub(crate)"]
            // crate_accessible: usize,

            // /// A doc comment.
            // #[get = "pub(super)"]
            // super_accessible: usize,

            // /// A doc comment.
            // #[get = "pub(in super::other)"]
            // scope_accessible: usize,
        }

        #[test]
        fn test_plain() {
            let val = Plain::default();
            val.private_accessible();
        }

        #[test]
        fn test_generic() {
            let val = Generic::<usize>::default();
            val.private_accessible();
        }

        #[test]
        fn test_where() {
            let val = Where::<usize>::default();
            val.private_accessible();
        }
    }
}

#[test]
fn test_plain() {
    let val = Plain::default();
    val.public_accessible();
}

#[test]
fn test_generic() {
    let val = Generic::<usize>::default();
    val.public_accessible();
}

#[test]
fn test_where() {
    let val = Where::<usize>::default();
    val.public_accessible();
}