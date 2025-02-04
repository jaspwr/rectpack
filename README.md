# rectpack

A 2D rectangle bin packer for efficiently and dynamically packing rectangles with a focus on simplicity. 
Features `alloc` and `free` functions to dynamically allocate and deallocate rectangles.
The implementation is only a few hundred lines and is designed to be easy to hack.

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
