use crate::{edge_segment::EdgeSegment, shape::Shape, vector::Vector2, EdgeColor};

fn is_corner(a_dir: Vector2, b_dir: Vector2, cross_threshold: f64) -> bool {
    Vector2::dot_product(a_dir, b_dir) <= 0.0
        || Vector2::cross_product(a_dir, b_dir).abs() > cross_threshold
}

fn switch_color(color: &mut EdgeColor, seed: &mut usize, banned: EdgeColor) {
    let combined: EdgeColor =
        num::cast::FromPrimitive::from_usize(*color as usize & banned as usize).unwrap();

    if combined == EdgeColor::RED || combined == EdgeColor::GREEN || combined == EdgeColor::BLUE {
        *color =
            num::cast::FromPrimitive::from_usize(combined as usize ^ EdgeColor::WHITE as usize)
                .unwrap();
        return;
    }
    if *color == EdgeColor::BLACK || *color == EdgeColor::WHITE {
        match *seed % 3 {
            0 => {
                *color = EdgeColor::CYAN;
            }
            1 => {
                *color = EdgeColor::MAGENTA;
            }
            2 => {
                *color = EdgeColor::YELLOW;
            }
            _ => panic!("Not supported!"),
        }

        *seed /= 3;
        return;
    }

    let shifted = (*color as usize) << (1 + (*seed & 1));
    *color = num::cast::FromPrimitive::from_usize(
        (shifted | shifted >> 3) & (EdgeColor::WHITE as usize),
    )
    .unwrap();
    *seed >>= 1;
}

pub fn simple(shape: &mut Shape, angle_threshold: f64, mut seed: usize) {
    let cross_threshold = angle_threshold.sin();
    let mut corners = Vec::new();

    for contour in shape.contours.iter_mut() {
        corners.clear();

        let edges = &mut contour.edges;

        let edge_count = edges.len();
        if edge_count != 0 {
            let mut prev_dir = edges.last().unwrap().direction(1.0);
            for i in 0..edge_count {
                let edge = &edges[i];
                if is_corner(
                    prev_dir.normalize(false),
                    edge.direction(0.0).normalize(false),
                    cross_threshold,
                ) {
                    corners.push(i);
                }
                prev_dir = edge.direction(1.0);
            }
        }

        if corners.len() == 0 {
            for i in 0..edge_count {
                edges[i].set_color(EdgeColor::WHITE);
            }
        } else if corners.len() == 1 {
            let mut colors = vec![EdgeColor::WHITE, EdgeColor::WHITE, EdgeColor::BLACK];
            switch_color(&mut colors[0], &mut seed, EdgeColor::BLACK);
            colors[2] = colors[0];
            switch_color(&mut colors[2], &mut seed, EdgeColor::BLACK);

            let corner = corners[0];
            if edge_count >= 3 {
                let m = edge_count;
                for i in 0..m {
                    let lookup =
                        ((3.0 + 2.875 * i as f64 / (m as f64 - 1.0) - 1.4375 + 0.5) as usize - 3)
                            + 1;
                    contour.edges[(corner + i) % m].set_color(colors[lookup]);
                }
            } else if edge_count >= 1 {
                let mut parts = [EdgeSegment::default(); 7];

                let (o1, o2, o3) = edges[0].split_in_thirds();
                parts[0 + 3 * corner] = o1;
                parts[1 + 3 * corner] = o2;
                parts[2 + 3 * corner] = o3;

                if edge_count >= 2 {
                    let (o1, o2, o3) = edges[1].split_in_thirds();
                    parts[3 + 3 * corner] = o1;
                    parts[4 + 3 * corner] = o2;
                    parts[5 + 3 * corner] = o3;
                } else {
                    parts[0].set_color(colors[0]);
                    parts[1].set_color(colors[1]);
                    parts[2].set_color(colors[2]);
                }
                edges.clear();
                for i in 0..7 {
                    edges.push(parts[i]);
                }
            }
        } else {
            let corner_count = corners.len();
            let mut spline = 0;
            let start = corners[0];

            let mut color = EdgeColor::WHITE;
            switch_color(&mut color, &mut seed, EdgeColor::BLACK);
            let initial_color = color;
            for i in 0..edge_count {
                let index = (start + i) % edge_count;
                if spline + 1 < corner_count && corners[spline + 1] == index {
                    spline += 1;
                    let banned_color =
                        (if spline == corner_count - 1 { 1 } else { 0 }) * initial_color as usize;
                    switch_color(
                        &mut color,
                        &mut seed,
                        num::cast::FromPrimitive::from_usize(banned_color).unwrap(),
                    );
                }
                edges[index].set_color(color);
            }
        }
    }
}
