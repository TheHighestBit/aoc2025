#![allow(unused_variables)]
#![allow(dead_code)]

fn calc_area(p1: &(u64, u64), p2: &(u64, u64)) -> u64 {
    let dx = (p1.0 as i64 - p2.0 as i64).abs() + 1;
    let dy = (p1.1 as i64 - p2.1 as i64).abs() + 1;

    (dx * dy) as u64
}

pub fn part1(input: &str) -> u64 {
    let points: Vec<(u64, u64)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            (
                parts.next().unwrap().parse::<u64>().unwrap(),
                parts.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect();

    let mut largest = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let area = calc_area(&points[i], &points[j]);
            if area > largest {
                largest = area;
            }
        }
    }

    largest
}

#[allow(clippy::type_complexity)]
fn is_point_in_poly(p: &(u64, u64), poly: &[((u64, u64), (u64, u64))]) -> bool {
    let px = p.0 as i64;
    let py = p.1 as i64;
    let mut inside = false;

    for edge in poly.iter() {
        let (x1, y1) = (edge.0.0 as i64, edge.0.1 as i64);
        let (x2, y2) = (edge.1.0 as i64, edge.1.1 as i64);

        // Check if point is exactly on this segment (on boundary = inside)
        if x1 == x2 {
            // vertical edge
            if px == x1 && py >= y1.min(y2) && py <= y1.max(y2) {
                return true;
            }
        } else {
            // horizontal edge
            if py == y1 && px >= x1.min(x2) && px <= x1.max(x2) {
                return true;
            }
        }

        // Ray-cast crossing test (horizontal ray to +inf)
        if (y1 > py) != (y2 > py) {
            let x_intersect = x1 + (py - y1) * (x2 - x1) / (y2 - y1);
            if px < x_intersect {
                inside = !inside;
            }
        }
    }

    inside
}

#[allow(clippy::type_complexity)]
fn edge_fully_inside(
    start: &(u64, u64),
    end: &(u64, u64),
    poly: &[((u64, u64), (u64, u64))],
) -> bool {
    // Check if all points along the edge are inside the polygon
    // For axis-aligned edges, we check the midpoint between each polygon edge crossing

    // Check endpoints (should already be checked but just to be safe)
    if !is_point_in_poly(start, poly) {
        return false;
    }
    if !is_point_in_poly(end, poly) {
        return false;
    }

    if start.0 == end.0 {
        // Vertical edge at x=start.0
        let x = start.0;
        let y_min = start.1.min(end.1);
        let y_max = start.1.max(end.1);

        // Collect all y-coordinates where horizontal polygon edges cross our line
        let mut crossings: Vec<u64> = vec![y_min, y_max];
        for edge in poly {
            if edge.0.1 == edge.1.1 {
                // Horizontal polygon edge at y = edge.0.1
                let ey = edge.0.1;
                let ex_min = edge.0.0.min(edge.1.0);
                let ex_max = edge.0.0.max(edge.1.0);
                if ey > y_min && ey < y_max && x >= ex_min && x <= ex_max {
                    crossings.push(ey);
                }
            }
        }
        crossings.sort();
        crossings.dedup();

        // Check midpoint of each segment
        for i in 0..crossings.len() - 1 {
            let mid_y = (crossings[i] + crossings[i + 1]) / 2;
            if !is_point_in_poly(&(x, mid_y), poly) {
                return false;
            }
        }
    } else {
        // Horizontal edge at y=start.1
        let y = start.1;
        let x_min = start.0.min(end.0);
        let x_max = start.0.max(end.0);

        // Collect all x-coordinates where vertical polygon edges cross our line
        let mut crossings: Vec<u64> = vec![x_min, x_max];
        for edge in poly {
            if edge.0.0 == edge.1.0 {
                // Vertical polygon edge at x = edge.0.0
                let ex = edge.0.0;
                let ey_min = edge.0.1.min(edge.1.1);
                let ey_max = edge.0.1.max(edge.1.1);
                if ex > x_min && ex < x_max && y >= ey_min && y <= ey_max {
                    crossings.push(ex);
                }
            }
        }
        crossings.sort();
        crossings.dedup();

        // Check midpoint of each segment
        for i in 0..crossings.len() - 1 {
            let mid_x = (crossings[i] + crossings[i + 1]) / 2;
            if !is_point_in_poly(&(mid_x, y), poly) {
                return false;
            }
        }
    }

    true
}

pub fn part2(input: &str) -> u64 {
    // Idea is simple, check if all 4 corners of rect are inside of the boundary.
    // If yes, then also check all 4 edges that make up the rectangle.
    let points: Vec<(u64, u64)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            (
                parts.next().unwrap().parse::<u64>().unwrap(),
                parts.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect();

    // Build polygon edges (cyclic)
    let mut poly = vec![(*points.last().unwrap(), *points.first().unwrap())];
    for i in 0..points.len() - 1 {
        poly.push((points[i], points[i + 1]));
    }

    let mut largest = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            let p3 = (p1.0, p2.1);
            let p4 = (p2.0, p1.1);

            // All 4 corners must be inside the polygon
            if !is_point_in_poly(&p3, &poly) || !is_point_in_poly(&p4, &poly) {
                continue;
            }

            // All 4 edges must be fully inside the polygon
            if !edge_fully_inside(&p1, &p3, &poly)
                || !edge_fully_inside(&p3, &p2, &poly)
                || !edge_fully_inside(&p2, &p4, &poly)
                || !edge_fully_inside(&p4, &p1, &poly)
            {
                continue;
            }

            let area = calc_area(&p1, &p2);
            if area > largest {
                largest = area;
            }
        }
    }

    largest
}
