use crate::geometry::{Point2D, Rectangle};

#[derive(Debug)]
pub struct QuadTree<T: std::fmt::Debug> {
    boundary: Rectangle,
    points: Vec<Point2D<T>>,
    ne: Option<Box<QuadTree<T>>>,
    se: Option<Box<QuadTree<T>>>,
    sw: Option<Box<QuadTree<T>>>,
    nw: Option<Box<QuadTree<T>>>,
}

impl<T: std::fmt::Debug> QuadTree<T> {
    const MAX_CAPACITY: usize = 4;

    pub fn new(boundary: Rectangle) -> Self {
        QuadTree {
            boundary,
            points: Vec::new(),
            ne: None,
            se: None,
            sw: None,
            nw: None,
        }
    }

    pub fn count(&self) -> usize {
        return self.points.len()
            + self.ne.as_ref().map_or(0, |ne| ne.count())
            + self.se.as_ref().map_or(0, |se| se.count())
            + self.sw.as_ref().map_or(0, |sw| sw.count())
            + self.nw.as_ref().map_or(0, |nw| nw.count());
    }

    pub fn insert(&mut self, point: Point2D<T>) -> Result<(), &'static str> {
        if !self.boundary.contains(point.x, point.y) {
            return Err("Boundary doesn't contain point");
        }
        
        if self.points.len() < QuadTree::<T>::MAX_CAPACITY {
            self.points.push(point);
            return Ok(());
        }

        // we need to insert the point in a sub-tree
        // if the sub-tree doesn't exist, create it
        let half_width = self.boundary.width / 2.0;
        let half_height = self.boundary.height / 2.0;
        let half_x = half_width + self.boundary.x;
        let half_y = half_height + self.boundary.y;

        let subtree = if point.x < half_x {
            if point.y < half_y {
                if self.nw.is_none() {
                    self.nw = Some(Box::new(QuadTree::new(self.boundary.new_nw())));
                }
                self.nw.as_mut().unwrap()
            } else {
                if self.sw.is_none() {
                    self.sw = Some(Box::new(QuadTree::new(self.boundary.new_sw())));
                }
                self.sw.as_mut().unwrap()
            }
        } else {
            if point.y < half_y {
                if self.ne.is_none() {
                    self.ne = Some(Box::new(QuadTree::new(self.boundary.new_ne())));
                }
                self.ne.as_mut().unwrap()
            } else {
                if self.se.is_none() {
                    self.se = Some(Box::new(QuadTree::new(self.boundary.new_se())));
                }
                self.se.as_mut().unwrap()
            }
        };
        return subtree.insert(point);
    }

    pub fn query(&self, boundary: Rectangle) -> Vec<&Point2D<T>> {
        let mut result: Vec<&Point2D<T>> = Vec::new();

        for point in self.points.iter() {
            if boundary.contains(point.x, point.y) {
                result.push(point);
            }
        }

        self.ne
            .iter()
            .chain(self.se.iter())
            .chain(self.sw.iter())
            .chain(self.nw.iter())
            .for_each(|subtree| {
                result.append(&mut subtree.query(boundary));
            });

        result
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
