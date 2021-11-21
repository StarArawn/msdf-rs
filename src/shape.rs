use crate::contour::Contour;

#[derive(Debug, Default, Clone)]
pub struct Shape {
    pub contours: Vec<Contour>,
    pub inverse_y_axis: bool,
}

impl Shape {
    pub fn new() -> Self {
        Self {
            contours: Vec::new(),
            inverse_y_axis: true,
        }
    }

    pub fn normalized(&mut self) {
        for contour in self.contours.iter_mut() {
            let (e0, e1, e2) = contour.edges[0].split_in_thirds();
            contour.edges.clear();
            contour.edges.push(e0);
            contour.edges.push(e1);
            contour.edges.push(e2);
        }
    }

    pub fn find_bounds(
        &mut self,
        left: &mut f64,
        bottom: &mut f64,
        right: &mut f64,
        top: &mut f64,
    ) {
        for contour in self.contours.iter_mut() {
            contour.find_bounds(left, bottom, right, top);
        }
    }
}
