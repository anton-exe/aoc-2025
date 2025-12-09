use std::cmp::{max, min};

pub fn generator(input: &str) -> &str {
    input
}

pub fn part_1(input: &str) -> u64 {
    let mut answer = 0;
    for (idx, line1) in input.lines().enumerate() {
        let x1 = line1.split_once(',').unwrap().0.parse::<i64>().unwrap();
        let y1 = line1.split_once(',').unwrap().1.parse::<i64>().unwrap();
        for line2 in input.lines().skip(idx + 1) {
            let x2 = line2.split_once(',').unwrap().0.parse::<i64>().unwrap();
            let y2 = line2.split_once(',').unwrap().1.parse::<i64>().unwrap();

            answer = max(
                answer,
                ((x1 - x2).unsigned_abs() + 1) * ((y1 - y2).unsigned_abs() + 1),
            );
        }
    }
    answer
}

fn xor(a: bool, b: bool) -> bool {
    (a || b) && !(a && b)
}

fn rect_check(v1: (i32, i32), v2: (i32, i32), poly: &[(i32, i32)]) -> bool {
    // this uses a slightly simplified of a method i found here: https://www.xjavascript.com/blog/check-if-polygon-is-inside-a-polygon/
    // (i wouldn't really count that as cheating since i used only math bits of it)
    //
    // this method checks, for all corners of the rectangle:
    //   (
    //     does this point lie on a line of the polygon? (v_confirmed)
    //     OR does a ray cast from this corner hit an *odd* number of lines? (v_ray_parity)
    //   )
    //   AND does a ray cast slightly inside this corner do that? (e_ray_parity)
    //   AND does that ray NEVER cross within the bounds of the rectangle? (lines 96..108)
    //
    // if this is false for *any* corner, we're out of bounds
    //
    // the parity check ensures the tested point is inside the polygon because werid math stuff

    let vertices = [v1, (v1.0, v2.1), v2, (v2.0, v1.1)];

    for v_idx in 0..vertices.len() {
        // get coords of 3 corners (the currently checked one + 2 adjacent)
        let (x_prev, y_prev) = vertices[(v_idx + vertices.len() - 1) % vertices.len()];
        let (x_curr, y_curr) = vertices[v_idx];
        let (x_next, y_next) = vertices[(v_idx + 1) % vertices.len()];
        // shorthand that's barely shorter but who cares
        let is_horizontal = y_curr == y_next;
        // these coords point towards the inside of the rectangle
        let centre_direction = if is_horizontal {
            ((x_next - x_curr).signum(), (y_prev - y_curr).signum())
        } else {
            ((x_prev - x_curr).signum(), (y_next - y_curr).signum())
        };
        // slightly offset for the e_ray
        let x_e = f64::from(x_curr) + (f64::from(centre_direction.0) * 0.5);
        let y_e = f64::from(y_curr) + (f64::from(centre_direction.1) * 0.5);

        let mut v_confirmed = false; // is this corner on a line?
        let mut v_ray_parity = false; // has v_ray intersected an odd number of times?
        let mut e_ray_parity = false; // has e_ray done that ^ ?

        for p_idx in 0..poly.len() {
            // this loop right here is pretty much half of what i got from the link above...
            let (x1, y1) = poly[p_idx];
            let (x2, y2) = poly[(p_idx + 1) % poly.len()];

            if !v_confirmed {
                if if y1 == y2 {
                    y1 == y_curr && min(x1, x2) <= x_curr && x_curr <= max(x1, x2)
                } else {
                    x1 == x_curr && min(y1, y2) <= y_curr && y_curr <= max(y1, y2)
                } {
                    v_confirmed = true;
                } else if ((y1 == y2) != is_horizontal)
                    && if is_horizontal {
                        x1 > x_curr && min(y1, y2) <= y_curr && max(y1, y2) >= y_curr
                    } else {
                        y1 > y_curr && min(x1, x2) <= x_curr && max(x1, x2) >= x_curr
                    }
                {
                    // ...the whole parity idea being the other half
                    v_ray_parity = !v_ray_parity;
                }
            }

            // this e_ray bullshit is all my own though
            if ((y1 == y2) != is_horizontal)
                && (if is_horizontal {
                    xor(f64::from(x1) > x_e, centre_direction.0 < 0)
                        && f64::from(min(y1, y2)) <= y_e
                        && f64::from(max(y1, y2)) >= y_e
                } else {
                    xor(f64::from(y1) > y_e, centre_direction.1 < 0)
                        && f64::from(min(x1, x2)) <= x_e
                        && f64::from(max(x1, x2)) >= x_e
                })
            {
                if if is_horizontal {
                    if centre_direction.0 > 0 {
                        x1 < max(v1.0, v2.0)
                    } else {
                        x1 > min(v1.0, v2.0)
                    }
                } else if centre_direction.1 > 0 {
                    y1 < max(v1.1, v2.1)
                } else {
                    y1 > min(v1.1, v2.1)
                } {
                    return false;
                }
                e_ray_parity = !e_ray_parity;
            }
        }

        v_confirmed |= v_ray_parity;

        if !v_confirmed || !e_ray_parity {
            return false;
        }
    }

    true
}

pub fn part_2(input: &str) -> u64 {
    let p1_ans = part_1(input);
    let mut answer = 0;
    let mut vertices = Vec::<(i32, i32)>::new();

    for line in input.lines() {
        let (x, y) = line.split_once(',').unwrap();
        vertices.push((x.parse().unwrap(), y.parse().unwrap()));
    }

    for (idx, v1) in vertices.iter().enumerate() {
        for v2 in vertices.iter().skip(idx + 1) {
            if p1_ans
                < (u64::from((v1.0 - v2.0).unsigned_abs() + 1)
                    * u64::from((v1.1 - v2.1).unsigned_abs() + 1))
                || v1.0 == v2.0
                || v1.1 == v2.1
            {
                continue;
            }
            if rect_check(*v1, *v2, &vertices) {
                answer = max(
                    answer,
                    u64::from((v1.0 - v2.0).unsigned_abs() + 1)
                        * u64::from((v1.1 - v2.1).unsigned_abs() + 1),
                );
            }
        }
    }

    answer
}
