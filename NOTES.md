# Flexo

no_std flexbox implementation

## API

```rust
Box<C> {
    padding: u32,
    margin: u32,
    children: &[T]
}

type StyledBox<C> = Styled<Box, PrimitiveStyle<C>>;
```

```rust
let display = ...;

let container = Box::with_children(&[...]);

let another = Box::with_child(Circle::new(...));

let more = Box::empty();

display.layout(&[&container, &another, &more]);

```

The playground compiles this:

```rust
trait Foo {}

struct A { doob: u32 }
struct B;

impl A{
    fn new() -> Self {
        Self { doob: 100 }
    }
}

impl Foo for A {}
impl Foo for B {}

struct DoStuff<'a> {
    children: &'a [&'a dyn Foo]
}

fn do_stuff(stuff: DoStuff) {
    // Do stuff
}

fn main() {
    // let a = A;
    let b = B;

    do_stuff(DoStuff { children: &[&b, &A::new()] });
}
```
