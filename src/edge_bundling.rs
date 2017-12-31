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

fn distance(p1x: f32, p1y: f32, p2x: f32, p2y: f32) -> f32 {
    let dx = p2x - p1x;
    let dy = p2y - p1y;
    (dx * dx + dy * dy).sqrt()
}

pub fn edge_bundling(points: &Vec<Point>, links: &Vec<Link>) -> Vec<Line> {
    let mut mid_points = Vec::new();
    let mut segments: Vec<LineSegment> = links
        .iter()
        .map(|link| LineSegment::new(link.source, link.target))
        .collect();

    let mut num_iter = 90;
    let mut alpha = 0.1;
    for cycle in 0..6 {
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

        let num_p = dp * 2 - 1;
        for _ in 0..num_iter {
            for point in mid_points.iter_mut() {
                point.vx = 0.;
                point.vy = 0.;
            }

            for segment in segments.iter() {
                let d = distance(
                    points[segment.source].x,
                    points[segment.source].y,
                    points[segment.target].x,
                    points[segment.target].y,
                );
                let kp = 0.1 / (num_p as usize as f32) / d;
                let n = segment.point_indices.len();
                for i in 0..n {
                    let p0 = if i == 0 {
                        points[segment.source]
                    } else {
                        mid_points[segment.point_indices[i - 1]]
                    };
                    let p2 = if i == n - 1 {
                        points[segment.target]
                    } else {
                        mid_points[segment.point_indices[i + 1]]
                    };
                    let ref mut p1 = mid_points[segment.point_indices[i]];
                    p1.vx += alpha * kp * (p0.x - p1.x + p2.x - p1.x);
                    p1.vy += alpha * kp * (p0.y - p1.y + p2.y - p1.y);
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
                    let p2 = distance(p1x, p1y, p2x, p2y);
                    let q2 = distance(q1x, q1y, q2x, q2y);
                    let l_avg = (p2 + q2) / 2.;
                    let pmx = (p1x + p2x) / 2.;
                    let pmy = (p1y + p2y) / 2.;
                    let qmx = (q1x + q2x) / 2.;
                    let qmy = (q1y + q2y) / 2.;
                    let c_a = {
                        let pq = (p2x - p1x) * (q2x - q1x) + (p2y - p1y) * (q2y - q1y);
                        (pq / p2 / q2).abs()
                    };
                    let c_s = 2. / (l_avg / p2.min(q2) + p2.max(q2) / l_avg);
                    let c_p = {
                        let mpq2 = distance(pmx, pmy, qmx, qmy);
                        l_avg / (l_avg + mpq2)
                    };
                    let c_v = {
                        let vp = {
                            let i0r = ((p1y - q1y) * (p1y - p2y) - (p1x - q1x) * (p2x - p1x)) /
                                p2 / p2;
                            let i0x = p1x + i0r * (p2x - p1x);
                            let i0y = p1y + i0r * (p2y - p1y);
                            let i1r = ((p1y - q2y) * (p1y - p2y) - (p1x - q2x) * (p2x - p1x)) /
                                p2 / p2;
                            let i1x = p1x + i1r * (p2x - p1x);
                            let i1y = p1y + i1r * (p2y - p1y);
                            let imx = (i0x + i1x) / 2.;
                            let imy = (i0y + i1y) / 2.;
                            (1. - 2. * distance(pmx, pmy, imx, imy) / distance(i0x, i0y, i1x, i1y))
                                .max(0.0)
                        };
                        let vq = {
                            let i0r = ((q1y - p1y) * (q1y - q2y) - (q1x - p1x) * (q2x - q1x)) /
                                q2 / q2;
                            let i0x = q1x + i0r * (q2x - q1x);
                            let i0y = q1y + i0r * (q2y - q1y);
                            let i1r = ((q1y - p2y) * (q1y - q2y) - (q1x - p2x) * (q2x - q1x)) /
                                q2 / q2;
                            let i1x = q1x + i1r * (q2x - q1x);
                            let i1y = q1y + i1r * (q2y - q1y);
                            let imx = (i0x + i1x) / 2.;
                            let imy = (i0y + i1y) / 2.;
                            (1. - 2. * distance(qmx, qmy, imx, imy) / distance(i0x, i0y, i1x, i1y))
                                .max(0.0)
                        };
                        vp.min(vq)
                    };
                    let c_e = c_a * c_s * c_p * c_v;
                    if c_e < 0.6 {
                        continue;
                    }
                    for i in 0..num_p {
                        let pi = mid_points[segment_p.point_indices[i as usize]];
                        let qi = mid_points[segment_q.point_indices[i as usize]];
                        let dx = qi.x - pi.x;
                        let dy = qi.y - pi.y;
                        if dx.abs() > 1e-6 && dy.abs() > 1e-6 {
                            let w = alpha / (dx * dx + dy * dy).sqrt();
                            {
                                let ref mut qi = mid_points[segment_q.point_indices[i as usize]];
                                qi.vx -= dx * w;
                                qi.vy -= dy * w;
                            }
                            {
                                let ref mut pi = mid_points[segment_p.point_indices[i as usize]];
                                pi.vx += dx * w;
                                pi.vy += dy * w;
                            }
                        }
                    }
                }
            }

            for point in mid_points.iter_mut() {
                point.x += point.vx;
                point.y += point.vy;
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
