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
    fn id(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    fn coalesce(&self, other: &Rectangle) -> Option<Rectangle> {
        if self.width == other.width && self.x == other.x {
            if self.y + self.height == other.y {
                return Some(Rectangle {
                    height: self.height + other.height,
                    ..*self
                });
            } else if self.y == other.y + other.height {
                return Some(Rectangle {
                    height: self.height + other.height,
                    ..*other
                });
            }
        } else if self.height == other.height && self.y == other.y {
            if self.x + self.width == other.x {
                return Some(Rectangle {
                    width: self.width + other.width,
                    ..*self
                });
            } else if other.x + other.width == self.x {
                return Some(Rectangle {
                    width: self.width + other.width,
                    ..*other
                });
            }
        }

        None
    }

    fn split_h(self, width: u32) -> (Rectangle, Rectangle) {
        assert!(width <= self.width);

        (
            Rectangle { width, ..self },
            Rectangle {
                x: self.x + width,
                width: self.width - width,
                ..self
            },
        )
    }

    fn split_v(self, height: u32) -> (Rectangle, Rectangle) {
        assert!(height <= self.height);

        (
            Rectangle { height, ..self },
            Rectangle {
                y: self.y + height,
                height: self.height - height,
                ..self
            },
        )
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

/// Returns the number of rectangles coalesced
fn coalesce_all(rects: &mut RectMap) -> usize {
    let mut remove = vec![];
    let mut new_rects = vec![];

    for (i, (id1, rect1)) in rects.iter().enumerate() {
        for (id2, rect2) in rects.iter().skip(i + 1) {
            if let Some(rect) = rect1.coalesce(rect2) {
                remove.push(*id1);
                remove.push(*id2);
                new_rects.push(rect);
            }
        }
    }

    let num_coalesced = new_rects.len();

    for id in remove {
        rects.remove(&id);
    }

    for rect in new_rects {
        append_rect(rects, rect);
    }

    if num_coalesced > 0 {
        coalesce_all(rects);
    }

    num_coalesced
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
        append_rect(&mut free, Rectangle {
            x: 0,
            y: 0,
            width,
            height,
        });

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

        coalesce_all(&mut self.free);

        // Find rect of same width and height
        if let Some(rect) = self
            .free
            .values()
            .find(|rect| rect.width == width && rect.height == height)
            .cloned()
        {
            self.free.remove(&rect.id());
            append_rect(&mut self.allocated, rect.clone());
            return Ok(rect);
        }

        // Find rect of same width
        if let Some(rect) = self
            .free
            .values()
            .find(|rect| rect.width == width && rect.height >= height)
            .cloned()
        {
            self.free.remove(&rect.id());
            let (alloced, remaining) = rect.split_h(width);
            append_rect(&mut self.free, remaining);
            append_rect(&mut self.allocated, alloced.clone());
            return Ok(alloced);
        }

        // Find rect of same height
        if let Some(rect) = self
            .free
            .values()
            .find(|rect| rect.height == height && rect.width >= width)
            .cloned()
        {
            self.free.remove(&rect.id());
            let (alloced, remaining) = rect.split_h(height);
            append_rect(&mut self.free, remaining);
            append_rect(&mut self.allocated, alloced.clone());
            return Ok(alloced);
        }

        // Find any rect that fits

        if let Some(rect) = self
            .free
            .values()
            .find(|rect| rect.width >= width && rect.height >= height)
            .cloned()
        {
            self.free.remove(&rect.id());
            let (alloced, remaining) = rect.split_h(width);
            append_rect(&mut self.free, remaining);
            let (alloced, remaining) = alloced.split_v(height);
            append_rect(&mut self.free, remaining);
            append_rect(&mut self.allocated, alloced.clone());
            return Ok(alloced);
        }

        Err(Error::OutOfSpace)
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

