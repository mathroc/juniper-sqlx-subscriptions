Trying to use a sqlx database pool inside a juniper GraphQL subscription resolver

The issue:

```
$ cargo run
   Compiling juniper-sqlx-subscriptions v0.1.0 (/home/mathieu/Projects/repro/juniper-sqlx-subscriptions)
error[E0759]: `executor` has lifetime `'ref_e` but it needs to satisfy a `'static` lifetime requirement
  --> src/main.rs:47:19
   |
40 |   #[graphql_subscription(context = Context)]
   |   ------------------------------------------ this data with lifetime `'ref_e`...
...
47 |           let stream = async_stream::stream! {
   |  ______________________^
48 | |             loop {
49 | |                 interval.tick().await;
50 | |
...  |
56 | |             }
57 | |         };
   | |_________^ ...is captured here...
   |
note: ...and is required to live as long as `'static` here
  --> src/main.rs:40:1
   |
40 | #[graphql_subscription(context = Context)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: this error originates in the macro `$crate::async_stream_impl::stream_inner` (in Nightly builds, run with -Z macro-backtrace for more info)
For more information about this error, try `rustc --explain E0759`.
error: could not compile `juniper-sqlx-subscriptions` due to previous error
```

some context:

* https://github.com/graphql-rust/juniper/issues/989
* https://stackoverflow.com/questions/65101589/how-does-one-use-sqlx-with-juniper-subscriptions-in-rust
* https://github.com/graphql-rust/juniper/issues/143

If someone has a solution, feel free to open a Pull Request ! 🙏
