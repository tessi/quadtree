use std::mem;

use crate::{Point2D, Rectangle};

#[derive(Debug)]
pub enum QuadTree<T: std::fmt::Debug> {
    Leaf {
        boundary: Rectangle,
        points: Vec<Point2D<T>>,
    },
    Root {
        boundary: Rectangle,
        points: Vec<Point2D<T>>,
        ne: Box<QuadTree<T>>,
        se: Box<QuadTree<T>>,
        sw: Box<QuadTree<T>>,
        nw: Box<QuadTree<T>>,
    },
}

impl<T: std::fmt::Debug> QuadTree<T> {
    const MAX_CAPACITY: usize = 4;

    pub fn new(boundary: Rectangle) -> Self {
        QuadTree::Leaf {
            boundary,
            points: Vec::new(),
        }
    }

    pub fn count(&self) -> usize {
        match self {
            QuadTree::Leaf {
                boundary: _,
                points,
            } => return points.len(),
            QuadTree::Root { ne, se, sw, nw, points, .. } => {
                return points.len() + ne.count() + se.count() + sw.count() + nw.count()
            }
        }
    }

    pub fn insert(&mut self, point: Point2D<T>) -> Result<(), &'static str> {
        match self {
            QuadTree::Leaf { boundary, points } => {
                if !boundary.contains(point.x, point.y) {
                    return Err("Boundary doesn't contain point");
                } else if points.len() == QuadTree::<T>::MAX_CAPACITY {
                    self.subdivide();
                    return self.insert(point);
                } else {
                    points.push(point);
                    return Ok(());
                }
            }
            QuadTree::Root { ne, se, sw, nw, points, boundary } => {
                if !boundary.contains(point.x, point.y) {
                    return Err("Boundary doesn't contain point");
                } else if points.len() < QuadTree::<T>::MAX_CAPACITY {
                    points.push(point);
                    return Ok(());
                } else if ne.covers(point.x, point.y) {
                    ne.insert(point)?;
                    return Ok(());
                } else if se.covers(point.x, point.y) {
                    se.insert(point)?;
                    return Ok(());
                } else if sw.covers(point.x, point.y) {
                    sw.insert(point)?;
                    return Ok(());
                } else if nw.covers(point.x, point.y) {
                    nw.insert(point)?;
                    return Ok(());
                }
                return Err("Point couldn't be inserted in any sub-tree");
            }
        }
    }

    pub fn query(&self, boundary: Rectangle) -> Vec<&Point2D<T>> {
        let mut result = Vec::new();
        match self {
            QuadTree::Leaf { points, .. } => {
                for point in points {
                    if boundary.contains(point.x, point.y) {
                        result.push(point);
                    }
                }
            }
            QuadTree::Root { ne, se, sw, nw, points, .. } => {
                for point in points {
                    if boundary.contains(point.x, point.y) {
                        result.push(point);
                    }
                }
                result.append(&mut ne.query(boundary));
                result.append(&mut se.query(boundary));
                result.append(&mut sw.query(boundary));
                result.append(&mut nw.query(boundary));
            }
        }
        result
    }

    fn covers(&self, x: f64, y: f64) -> bool {
        match self {
            QuadTree::Leaf { boundary, .. } => return boundary.contains(x, y),
            QuadTree::Root { boundary, .. } => return boundary.contains(x, y)
        }
    }

    fn subdivide(&mut self) {
        match self {
            QuadTree::Leaf { boundary, points } => {
                let new_width = boundary.width / 2.0;
                let new_height = boundary.height / 2.0;

                let new = QuadTree::Root {
                    points: points.drain(0..).collect(),
                    boundary: boundary.clone(),
                    ne: Box::new(QuadTree::new(Rectangle::new(
                        boundary.x + new_width,
                        boundary.y,
                        new_width,
                        new_height,
                    ))),
                    se: Box::new(QuadTree::new(Rectangle::new(
                        boundary.x + new_width,
                        boundary.y + new_height,
                        new_width,
                        new_height,
                    ))),
                    sw: Box::new(QuadTree::new(Rectangle::new(
                        boundary.x,
                        boundary.y + new_height,
                        new_width,
                        new_height,
                    ))),
                    nw: Box::new(QuadTree::new(Rectangle::new(
                        boundary.x,
                        boundary.y,
                        new_width,
                        new_height,
                    ))),
                };
                
                let _ = mem::replace(self, new);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::{Point2D, Rectangle};

    use super::*;

    #[test]
    fn it_inserts_a_point() -> Result<(), Box<dyn std::error::Error>> {
        let mut quadtree = QuadTree::<u8>::new(Rectangle::new(0.0, 0.0, 100.0, 100.0));
        assert_eq!(quadtree.count(), 0);

        let point = Point2D {
            x: 10.0,
            y: 10.0,
            data: 42,
        };
        quadtree.insert(point)?;
        assert_eq!(quadtree.count(), 1);

        let points = quadtree.query(Rectangle::new(0.0, 0.0, 100.0, 100.0));
        assert_eq!(points.len(), 1);
        assert!(points[0].data == 42);

        let points = quadtree.query(Rectangle::new(9.0, 9.0, 11.0, 11.0));
        assert_eq!(points.len(), 1);
        assert!(points[0].data == 42);

        Ok(())
    }

    #[test]
    fn it_inserts_many_points() -> Result<(), Box<dyn std::error::Error>> {
        let mut quadtree = QuadTree::<u8>::new(Rectangle::new(0.0, 0.0, 100.0, 100.0));

        for i in 0..10 {
            let point = Point2D {
                x: 10.0 + i as f64,
                y: 10.0 + i as f64,
                data: 42,
            };
            quadtree.insert(point)?;
        }

        for i in 0..10 {
            let point = Point2D {
                x: 90.0 + i as f64,
                y: 90.0 + i as f64,
                data: 42,
            };
            quadtree.insert(point)?;
        }
        assert_eq!(quadtree.count(), 20);

        let points = quadtree.query(Rectangle::new(0.0, 0.0, 100.0, 100.0));
        assert_eq!(points.len(), 20);
        assert!(points[0].data == 42);

        let points = quadtree.query(Rectangle::new(9.0, 9.0, 11.0, 11.0));
        assert_eq!(points.len(), 10);
        assert!(points[0].data == 42);

        Ok(())
    }

    #[test]
    fn it_inserts_the_same_point_often() -> Result<(), Box<dyn std::error::Error>> {
        let mut quadtree = QuadTree::<u8>::new(Rectangle::new(0.0, 0.0, 100.0, 100.0));

        for _i in 0..10 {
            let point = Point2D {
                x: 10.0 as f64,
                y: 10.0 as f64,
                data: 42,
            };
            quadtree.insert(point)?;
        }

        let points = quadtree.query(Rectangle::new(0.0, 0.0, 100.0, 100.0));
        assert_eq!(points.len(), 10);
        assert!(points[0].data == 42);

        Ok(())
    }
}
