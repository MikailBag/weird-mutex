# Syncing unsyncable
This library shows a case when you can safely
implement Sync for your type without
imposing Sync bound on generic parameters.
See `WeirdMutex` documentation
for more details.
## Example
Imagine that yoi have a struct that stores some future in it.
In this example we will use boxed future, but the same would
apply to taking future type as a generic parameter.
```rust
use std::{pin::Pin, future::Future};
struct Thing {
   fut: Pin<Box<dyn Future<Output=<()> + Send>>>,
   // ...
}
```
Unfortunately, this type is not Sync so it can be unergonomic to use
with some libraries that require Sync.
Naive solution is to write something like this:
```rust
use std::{pin::Pin, future::Future};
struct Thing {
   fut: Pin<Box<dyn Future<Output=<()> + Send + Sync>>>,
   // ...
}
```
Unfortunately, now we impose additional restricitons on the
stored future. For example, future using `Cell` type can no longer
be stored in our Thing.
Better solution is to use WeirdMutex:
```rust
use std::{pin::Pin, future::Future};
struct Thing {
   fut: weird_mutex::WeirdMutex<Pin<Box<dyn Future<Output=<()> + Send>>>>,
   // ...
}
```
