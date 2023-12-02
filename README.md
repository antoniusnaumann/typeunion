# typeunion
Instead of declaring simple type unions using an enum like this
```Rust
pub enum MyTypes {
  String(String),
  Int(i32),
}
```
this macro lets you simply write this:
```Rust
// Use an alias to give the enum case a custom name.
// By default, the associated type name is used.
type Int = i32;

#[type_union]
pub type MyTypes = String + Int;
```

It also can generate `From`-implementations for subsets:

```Rust
use typeunion::type_union;

#[type_union]
type SuperSet = String + TypeB + TypeC;

#[type_union(super = SuperSet)]
type SubSet = String + TypeB;

fn main() {
  // `From` is automatically implemented for all types contained in SubSet
  let sub_set: SubSet = "hello".to_string().into();
  let super_set: SuperSet = sub.into();
}
```

As the name of the generated enum case is automatically derived from the given type names, only identifiers are allowed as members of a type union.
To work around this, you can create a type alias:
```Rust
type ArcStr = Arc<str>;
type IntVec = Vec<i64>;

#[type_union]
type Types = ArcStr + IntVec;
```
