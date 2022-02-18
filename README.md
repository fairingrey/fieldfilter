# fieldfilter

fieldfilter is a pretty simple crate to use. Basically what it offers is a way
to essentially “filter” a struct to a smaller set of itself.

Here’s an example:

```rust
use std::collections::HashSet;
use fieldfilter::FieldFilterable;

struct User {
    id: u32,
    name: String,
    email: String,
}
#[derive(Debug, FieldFilterable)]
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

let fields = HashSet::from(["name".to_owned()]);

let filtered_user: FilteredUser = FieldFilterable::field_filter(user, fields);
println!("{:?}", filtered_user);
```

This kind of behavior is quite useful for whenever you have some struct or
entity you want to skim off the visibility of certain fields, whether that be
some SQL row being returned as some REST or GraphQL response, or just as a
simple sanitization routine.

## Inspiration

Why this crate exists is because I had a certain use case for it. I wanted to
essentially be able to map models from an ORM to a serialized HTTP response with
only the fields I wanted to be visible, dictated by policy files in my chosen
authorization engine.

Not to mention it was a reason to finally get into proc macros!
