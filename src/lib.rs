//! fieldfilter is a pretty simple crate to use. Basically what it offers is
//! a way to essentially "filter" a struct to a smaller set of itself.
//!
//! Here's an example:
//! ```
//! use std::collections::HashSet;
//! use fieldfilter::FieldFilterable;
//!
//! struct User {
//!     id: u32,
//!     name: String,
//!     email: String,
//! }
//! #[derive(Debug, FieldFilterable)]
//! #[field_filterable_on(User)]
//! struct FilteredUser {
//!     id: u32,
//!     name: Option<String>,
//!     email: Option<String>,
//! }
//!
//! let user = User {
//!     id: 1,
//!     name: "Allen".to_string(),
//!     email: "allen@example.org".to_string(),
//! };
//!
//! let fields = HashSet::from(["name".to_owned()]);
//!
//! let filtered_user: FilteredUser = FieldFilterable::field_filter(user, fields);
//! println!("{:?}", filtered_user);
//! ```
//!
//! This kind of behavior is quite useful for whenever you have some struct or
//! entity you want to skim off the visibility of certain fields, whether that be
//! some SQL row being returned as some REST or GraphQL response, or just as a simple
//! sanitization routine.
//!
#![warn(
    missing_docs,
    missing_debug_implementations,
    unreachable_pub,
    future_incompatible,
    rust_2018_idioms,
    rust_2021_compatibility
)]
use std::collections::HashSet;

/// A struct that acts as a filter of another struct given a set of fields.
pub trait FieldFilterable<T> {
    /// field filter
    fn field_filter(o: T, fields: HashSet<String>) -> Self;
}

pub use fieldfilter_derive::FieldFilterable;

#[cfg(test)]
mod tests {
    use super::FieldFilterable;
    use std::collections::HashSet;

    #[test]
    fn test_basic() {
        struct User {
            id: u32,
            name: String,
            email: String,
        }

        #[derive(Debug, PartialEq, FieldFilterable)]
        #[field_filterable_on(User)]
        struct FilteredUser {
            id: u32,
            name: Option<String>,
            email: Option<String>,
        }

        let user = User {
            id: 1,
            name: "Allen".to_string(),
            email: "allen@example.org".to_string(),
        };

        let fields = HashSet::from(["email".to_owned()]);

        let filtered_user = FieldFilterable::field_filter(user, fields);

        assert_eq!(
            FilteredUser {
                id: 1,
                name: None,
                email: Some("allen@example.org".to_string())
            },
            filtered_user
        );
    }

    #[test]
    fn test_optional_base() {
        struct User {
            id: u32,
            name: String,
            email: String,
            status: Option<String>,
        }

        #[derive(Debug, PartialEq, FieldFilterable)]
        #[field_filterable_on(User)]
        struct FilteredUser {
            id: u32,
            name: Option<String>,
            email: Option<String>,
            status: Option<Option<String>>,
        }

        let user = User {
            id: 1,
            name: "Allen".to_string(),
            email: "allen@example.org".to_string(),
            status: None,
        };

        let fields = HashSet::from(["email".to_owned()]);

        let filtered_user = FieldFilterable::field_filter(user, fields);

        assert_eq!(
            FilteredUser {
                id: 1,
                name: None,
                email: Some("allen@example.org".to_string()),
                status: None,
            },
            filtered_user
        );
    }
}
