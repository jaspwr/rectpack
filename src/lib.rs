use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a rectangle within the arena. Can be passed to `free` on the arena to deallocate the
/// rectangle.
pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    fn id(&self) -> RectId {
        (self.x, self.y)
    }

    fn intersects(&self, other: &Rectangle) -> bool {
        self.x < other.end_x()
            && self.end_x() > other.x
            && self.y < other.end_y()
            && self.end_y() > other.y
    }

    /// The x coordinate of the far edge of the rectangle.
    pub fn end_x(&self) -> u32 {
        self.x + self.width
    }

    /// The y coordinate of the far edge of the rectangle.
    pub fn end_y(&self) -> u32 {
        self.y + self.height
    }
}

/// A 2D arena for allocating rectangles.
pub struct Arena {
    width: u32,
    height: u32,
    allocated: RectMap,
    free: RectMap,
}

impl Arena {
    /// Create a new arena with the given width and height.
    pub fn new(width: u32, height: u32) -> Self {
        let mut free = HashMap::new();
        append_rect(
            &mut free,
            Rectangle {
                x: 0,
                y: 0,
                width,
                height,
            },
        );

        Self {
            width,
            height,
            allocated: HashMap::new(),
            free,
        }
    }

    /// Allocate a rectangle of the given width and height.
    /// Returns an error if the size is invalid or there is not enough space remaining.
    pub fn alloc(&mut self, width: u32, height: u32) -> Result<Rectangle, Error> {
        if width == 0 || height == 0 || width > self.width || height > self.height {
            return Err(Error::InvalidSize);
        }

        let mut spot = None;
        'outer: for (id, rect) in self.free.iter() {
            let rect = Rectangle { width, height, ..*rect };
            
            if rect.end_x() > self.width || rect.end_y() > self.height {
                continue;
            }

            for allocated in self.allocated.values() {
                if rect.intersects(allocated) {
                    continue 'outer;
                }
            }

            // TODO: check heuristics for best fit
            spot = Some(*id);
        }

        let Some(id) = spot else {
            return Err(Error::OutOfSpace);
        };

        let rect = self
            .free
            .remove(&id)
            .unwrap();

        let alloced = Rectangle { width, height, ..rect };
        append_rect(&mut self.allocated, alloced.clone());

        append_rect(&mut self.free, Rectangle { x: alloced.end_x(), ..rect });
        append_rect(&mut self.free, Rectangle { y: alloced.end_y(), ..rect });

        assert_eq!(alloced.width, width);
        assert_eq!(alloced.height, height);

        Ok(alloced)
    }

    /// Deallocate the given rectangle and free the area to be allocated again.
    /// Returns an error if the rectangle was not found.
    pub fn free(&mut self, rect: Rectangle) -> Result<(), Error> {
        let rect = self
            .allocated
            .remove(&rect.id())
            .ok_or(Error::RectangleNotFound)?;

        append_rect(&mut self.free, rect);

        Ok(())
    }

    /// Returns an iterator over all allocated rectangles.
    pub fn allocated(&self) -> impl Iterator<Item = &Rectangle> {
        self.allocated.values()
    }

    /// Returns total dimensions of the arena.
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

type RectId = (u32, u32);
type RectMap = HashMap<RectId, Rectangle>;

fn append_rect(rects: &mut RectMap, rect: Rectangle) {
    rects.insert(rect.id(), rect);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidSize,
    OutOfSpace,
    RectangleNotFound,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidSize => write!(f, "Invalid size"),
            Error::OutOfSpace => write!(f, "Out of space"),
            Error::RectangleNotFound => write!(f, "Rectangle not found"),
        }
    }
}

impl std::error::Error for Error {}
