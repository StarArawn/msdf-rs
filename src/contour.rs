use crate::{edge_segment::EdgeSegment, vector::Vector2, EdgeColor};

#[derive(Debug, Default, Clone)]
pub struct Contour {
    pub edges: Vec<EdgeSegment>,
    has_calculated_bounds: bool,
    bounds_left: f64,
    bounds_right: f64,
    bounds_top: f64,
    bounds_bottom: f64,
}

impl Contour {
    pub fn new() -> Self {
        Self {
            edges: Vec::new(),
            has_calculated_bounds: false,
            bounds_left: 0.0,
            bounds_right: 0.0,
            bounds_top: 0.0,
            bounds_bottom: 0.0,
        }
    }

    pub fn add_edge(&mut self, edge: EdgeSegment) -> &EdgeSegment {
        self.edges.push(edge);
        self.edges.last().unwrap()
    }

    pub fn add_line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64) -> &EdgeSegment {
        self.add_edge(EdgeSegment::new_linear(
            Vector2::new(x0, y0),
            Vector2::new(x1, y1),
            EdgeColor::WHITE,
        ))
    }

    pub fn add_quadratic_segment(
        &mut self,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    ) -> &EdgeSegment {
        self.add_edge(EdgeSegment::new_quadratic(
            Vector2::new(x0, y0),
            Vector2::new(x1, y1),
            Vector2::new(x2, y2),
            EdgeColor::WHITE,
        ))
    }

    pub fn add_cubic_segment(
        &mut self,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        x3: f64,
        y3: f64,
    ) -> &EdgeSegment {
        self.add_edge(EdgeSegment::new_cubic(
            Vector2::new(x0, y0),
            Vector2::new(x1, y1),
            Vector2::new(x2, y2),
            Vector2::new(x3, y3),
            EdgeColor::WHITE,
        ))
    }

    pub fn find_bounds(
        &mut self,
        left: &mut f64,
        bottom: &mut f64,
        right: &mut f64,
        top: &mut f64,
    ) {
        if !self.has_calculated_bounds {
            self.bounds_left = std::f64::MAX;
            self.bounds_right = std::f64::MIN;
            self.bounds_top = std::f64::MIN;
            self.bounds_bottom = std::f64::MAX;

            for edge in self.edges.iter() {
                edge.find_bounds(
                    &mut self.bounds_left,
                    &mut self.bounds_bottom,
                    &mut self.bounds_right,
                    &mut self.bounds_top,
                );
            }
            self.has_calculated_bounds = true;
        }
        if self.bounds_left < *left {
            *left = self.bounds_left;
        }
        if self.bounds_right > *right {
            *right = self.bounds_right;
        }
        if self.bounds_bottom < *bottom {
            *bottom = self.bounds_bottom;
        }
        if self.bounds_bottom > *top {
            *top = self.bounds_bottom;
        }
    }

    pub fn winding(&self) -> i32 {
        let mut total: f64 = 0.0;
        match self.edges.len() {
            0 => {
                return 0;
            }
            1 => {
                let a = self.edges[0].point(0.0);
                let b = self.edges[0].point(1.0 / 3.0);
                let c = self.edges[0].point(2.0 / 3.0);
                total += Vector2::shoelace(a, b);
                total += Vector2::shoelace(b, c);
                total += Vector2::shoelace(c, a);
            }
            2 => {
                let a = self.edges[0].point(0.0);
                let b = self.edges[0].point(0.5);
                let c = self.edges[1].point(0.0);
                let d = self.edges[1].point(0.5);

                total += Vector2::shoelace(a, b);
                total += Vector2::shoelace(b, c);
                total += Vector2::shoelace(c, d);
                total += Vector2::shoelace(d, a);
            }
            _ => {
                let mut prev = self.edges.last().unwrap().point(0.0);
                for edge in self.edges.iter() {
                    let cur = edge.point(0.0);
                    total += Vector2::shoelace(prev, cur);
                    prev = cur;
                }
            }
        }
        return Vector2::sign(total) as i32;
    }
}
