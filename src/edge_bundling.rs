use force::{Point, Link, Force};

pub struct LineSegment {
    source: usize,
    target: usize,
    point_indices: Vec<usize>,
}

pub struct Line {
    pub source: usize,
    pub target: usize,
    pub points: Vec<Point>,
}

impl LineSegment {
    fn new(source: usize, target: usize) -> LineSegment {
        LineSegment {
            source: source,
            target: target,
            point_indices: Vec::new(),
        }
    }
}

pub struct EdgeBundlingForce {}

impl EdgeBundlingForce {
    pub fn new() -> EdgeBundlingForce {
        EdgeBundlingForce {}
    }
}

impl Force for EdgeBundlingForce {
    fn apply(&self, _points: &mut Vec<Point>, _alpha: f32) {}
}

pub fn edge_bundling(points: &Vec<Point>, links: &Vec<Link>) -> Vec<Line> {
    let mut mid_points = Vec::new();
    let mut segments: Vec<LineSegment> = links
        .iter()
        .map(|link| LineSegment::new(link.source, link.target))
        .collect();

    let mut num_iter = 50;
    let mut alpha = 0.04;
    for cycle in 0..5 {
        let dp = (2 as i32).pow(cycle);
        for segment in segments.iter_mut() {
            for j in 0..dp {
                let p0 = if j == 0 {
                    points[segment.source]
                } else {
                    mid_points[segment.point_indices[(j * 2 - 1) as usize]]
                };
                let p1 = if j == dp - 1 {
                    points[segment.target]
                } else {
                    mid_points[segment.point_indices[(j * 2) as usize]]
                };
                mid_points.push(Point::new((p0.x + p1.x) / 2., (p0.y + p1.y) / 2.));
                segment.point_indices.insert(
                    (j * 2) as usize,
                    mid_points.len() - 1,
                );
            }
        }

        for point in mid_points.iter_mut() {
            point.vx = 0.;
            point.vy = 0.;
        }

        let num_p = dp * 2 - 1;
        let kp = 1. / num_p as usize as f32;
        for _ in 0..num_iter {
            for segment in segments.iter() {
                let n = segment.point_indices.len();
                for i in 0..n {
                    let p0 = if i == 0 {
                        points[segment.source]
                    } else {
                        mid_points[segment.point_indices[i - 1]]
                    };
                    let p1 = mid_points[segment.point_indices[i]];
                    let p2 = if i == n - 1 {
                        points[segment.target]
                    } else {
                        mid_points[segment.point_indices[i + 1]]
                    };
                    let dp0x = p0.x - p1.x;
                    let dp0y = p0.y - p1.y;
                    let dp2x = p2.x - p1.x;
                    let dp2y = p2.y - p1.y;
                    let w0 = (dp0x * dp0x + dp0y * dp0y).sqrt();
                    let w2 = (dp2x * dp2x + dp2y * dp2y).sqrt();
                    mid_points[segment.point_indices[i]].vx += alpha * kp * (dp0x + dp2x);
                    mid_points[segment.point_indices[i]].vy += alpha * kp * (dp0y + dp2y);
                    // eprintln!("{:?} {:?} {:?}", p0, p1, p2);
                    // eprintln!("{} {}", dp0x + dp2x, dp0y + dp2y);
                }
            }

            let m = segments.len();
            for p in 0..m {
                let segment_p = &segments[p];
                for q in (p + 1)..m {
                    let segment_q = &segments[q];
                    let p1x = points[segment_p.source].x;
                    let p2x = points[segment_p.target].x;
                    let p1y = points[segment_p.source].y;
                    let p2y = points[segment_p.target].y;
                    let q1x = points[segment_q.source].x;
                    let q2x = points[segment_q.target].x;
                    let q1y = points[segment_q.source].y;
                    let q2y = points[segment_q.target].y;
                    let dpx = p2x - p1x;
                    let dpy = p2y - p1y;
                    let dqx = q2x - q1x;
                    let dqy = q2y - q1y;
                    let pq = dpx * dqx + dpy * dqy;
                    let p2 = (dpx * dpx + dpy * dpy).sqrt();
                    let q2 = (dqx * dqx + dqy * dqy).sqrt();
                    let c_a = pq / p2 / q2;
                    let l_avg = (p2 + q2) / 2.;
                    let c_s = 2. / (l_avg * p2.min(q2) + p2.max(q2) / l_avg);
                    let pmx = mid_points[segment_p.point_indices[(num_p / 2) as usize]].x;
                    let pmy = mid_points[segment_p.point_indices[(num_p / 2) as usize]].y;
                    let qmx = mid_points[segment_q.point_indices[(num_p / 2) as usize]].x;
                    let qmy = mid_points[segment_q.point_indices[(num_p / 2) as usize]].y;
                    let dmx = pmx - qmx;
                    let dmy = pmy - qmy;
                    let mpq2 = (dmx * dmx + dmy * dmy).sqrt();
                    let c_p = l_avg / (l_avg + mpq2);
                    let c_e = c_a * c_p;
                    for i in 0..num_p {
                        let pi = mid_points[segment_p.point_indices[i as usize]];
                        let qi = mid_points[segment_q.point_indices[i as usize]];
                        let dx = qi.x - pi.x;
                        let dy = qi.y - pi.y;
                        let w = alpha * c_e / (dx * dx + dy * dy).sqrt();
                        mid_points[segment_q.point_indices[i as usize]].vx -= dx * w;
                        mid_points[segment_q.point_indices[i as usize]].vy -= dy * w;
                        mid_points[segment_p.point_indices[i as usize]].vx += dx * w;
                        mid_points[segment_p.point_indices[i as usize]].vy += dy * w;
                    }
                }
            }

            for point in mid_points.iter_mut() {
                point.x += 0.6 * point.vx;
                point.y += 0.6 * point.vy;
            }
        }

        alpha /= 2.;
        num_iter = num_iter * 2 / 3;
    }

    segments
        .iter()
        .map(|segment| {
            let mut ps: Vec<Point> = segment
                .point_indices
                .iter()
                .map(|i| mid_points[*i])
                .collect();
            ps.insert(0, points[segment.source]);
            ps.push(points[segment.target]);
            Line {
                source: segment.source,
                target: segment.target,
                points: ps,
            }
        })
        .collect()
}
