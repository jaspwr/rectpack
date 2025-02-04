# rectpack

2D rectangle bin packing algorithms focused on simplicity for efficiently and dynamically packing rectangles into a bin. 
Features an `alloc` `free` function to dynamically allocate and deallocate rectangles.
The implementation in only a few hundred lines and is designed to be easy to hack.

## Example

```rust
use rectpack::*;

fn main() {
    let mut arena = Arena::new(100, 100);

    let rect1 = arena.pack(10, 10).unwrap();
    let rect2 = arena.pack(20, 20).unwrap();

    println!("{:?}", rect1);
    println!("{:?}", rect2);

    arena.free(rect1);
    arena.free(rect2);
}
```
