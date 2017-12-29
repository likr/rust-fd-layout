use std::collections::HashMap;
use quadtree::{Quadtree, Rect, NodeId, Element};

#[derive(Copy, Clone, Debug)]
pub struct Body {
    x: f32,
    y: f32,
    strength: f32,
}

impl Body {
    fn new(x: f32, y: f32, strength: f32) -> Body {
        Body {
            x: x,
            y: y,
            strength: strength,
        }
    }
}

impl Default for Body {
    fn default() -> Body {
        Body::new(0., 0., 0.)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point {
            x: x,
            y: y,
            vx: 0.,
            vy: 0.,
        }
    }
}

pub struct Link {
    source: usize,
    target: usize,
}

impl Link {
    pub fn new(source: usize, target: usize) -> Link {
        Link {
            source: source,
            target: target,
        }
    }
}

pub struct Force {
    points: Vec<Point>,
}

fn accumulate(tree: &mut Quadtree<Body>, node_id: NodeId) {
    let mut sum_weight = 0.;
    let mut sum_strength = 0.;
    let mut sum_x = 0.;
    let mut sum_y = 0.;
    for &(ref e, _) in tree.elements(node_id).iter() {
        match **e {
            Element::Leaf { x, y, n } => {
                let strength = -30. * n as f32;
                let weight = strength.abs();
                sum_strength += strength;
                sum_weight += weight;
                sum_x += x * weight;
                sum_y += y * weight;
            }
            Element::Node { node_id } => {
                accumulate(tree, node_id);
                let data = tree.data(node_id);
                let strength = data.strength;
                let weight = strength.abs();
                sum_strength += strength;
                sum_weight += weight;
                sum_x += data.x * weight;
                sum_y += data.y * weight;
            }
            Element::Empty => {}
        }
    }
    let data = tree.data_mut(node_id);
    data.strength = sum_strength;
    data.x = sum_x / sum_weight;
    data.y = sum_y / sum_weight;
}

fn apply_many_body(
    point: &mut Point,
    tree: &Quadtree<Body>,
    node_id: NodeId,
    alpha: f32,
    theta2: f32,
) {
    for &(ref e, _) in tree.elements(node_id).iter() {
        match **e {
            Element::Node { node_id } => {
                let data = tree.data(node_id);
                let rect = tree.rect(node_id);
                let dx = rect.cx - point.x;
                let dy = rect.cy - point.y;
                let w = rect.width;
                let l = dx * dx + dy * dy;
                if w * w / theta2 < l {
                    point.vx += dx * data.strength * alpha / l;
                    point.vy += dy * data.strength * alpha / l;
                } else {
                    apply_many_body(point, tree, node_id, alpha, theta2);
                }
            }
            Element::Leaf { x, y, n } => {
                if x != point.x || y != point.y {
                    let strength = -30. * n as f32;
                    let dx = x - point.x;
                    let dy = y - point.y;
                    let l = dx * dx + dy * dy;
                    point.vx += dx * strength * alpha / l;
                    point.vy += dy * strength * alpha / l;
                }
            }
            Element::Empty => {}
        }
    }
}

impl Force {
    pub fn new() -> Force {
        Force { points: Vec::new() }
    }

    pub fn many_body(&mut self, alpha: f32) {
        let max_x = self.points.iter().fold(0.0 / 0.0, |m, v| v.x.max(m));
        let min_x = self.points.iter().fold(0.0 / 0.0, |m, v| v.x.min(m));
        let max_y = self.points.iter().fold(0.0 / 0.0, |m, v| v.y.max(m));
        let min_y = self.points.iter().fold(0.0 / 0.0, |m, v| v.y.min(m));
        let width = max_x - min_x;
        let height = max_y - min_y;
        let mut tree = Quadtree::new(Rect {
            cx: min_x + width / 2.,
            cy: min_y + height / 2.,
            width: width,
            height: height,
        });
        let root = tree.root();
        for point in self.points.iter() {
            tree.insert(root, point.x, point.y);
        }
        accumulate(&mut tree, root);
        for mut point in self.points.iter_mut() {
            apply_many_body(&mut point, &tree, root, alpha, 0.81);
        }
    }

    pub fn link(&mut self, alpha: f32, links: Vec<Link>) {
        let mut count: HashMap<usize, usize> = HashMap::new();
        for link in links.iter() {
            if !count.contains_key(&link.source) {
                count.insert(link.source, 0);
            }
            if !count.contains_key(&link.target) {
                count.insert(link.target, 0);
            }
            {
                let v = count.get_mut(&link.source).unwrap();
                *v += 1;
            }
            {
                let v = count.get_mut(&link.target).unwrap();
                *v += 1;
            }
        }
        let bias = links
            .iter()
            .map(|link| {
                let source_count = *count.get(&link.source).unwrap();
                let target_count = *count.get(&link.target).unwrap();
                source_count as f32 / (source_count + target_count) as f32
            })
            .collect::<Vec<f32>>();
        for (link, b) in links.iter().zip(bias.iter()) {
            let source = self.points[link.source];
            let target = self.points[link.target];
            let source_count = count.get(&link.source).unwrap();
            let target_count = count.get(&link.target).unwrap();
            let dx = target.x - source.x;
            let dy = target.y - source.y;
            let l = (dx * dx + dy * dy).sqrt();
            let strength = 1. / *source_count.min(target_count) as f32;
            let w = (l - 30.) / l * alpha * strength;
            {
                let ref mut target = self.points[link.target];
                target.vx -= dx * w * b;
                target.vy -= dy * w * b;
            }
            {
                let ref mut source = self.points[link.source];
                source.vx += dx * w * (1. - b);
                source.vy += dy * w * (1. - b);
            }
        }
    }
}

#[test]
fn test_many_body() {
    let mut force = Force::new();
    force.points.push(Point::new(10., 10.));
    force.points.push(Point::new(10., -10.));
    force.points.push(Point::new(-10., 10.));
    force.points.push(Point::new(-10., -10.));
    force.many_body(1.);
    assert!(force.points[0].vx == 2.25);
    assert!(force.points[0].vy == 2.25);
    assert!(force.points[1].vx == 2.25);
    assert!(force.points[1].vy == -2.25);
    assert!(force.points[2].vx == -2.25);
    assert!(force.points[2].vy == 2.25);
    assert!(force.points[3].vx == -2.25);
    assert!(force.points[3].vy == -2.25);
}

#[test]
fn test_link() {
    let mut force = Force::new();
    force.points.push(Point::new(10., 10.));
    force.points.push(Point::new(10., -10.));
    force.points.push(Point::new(-10., 10.));
    force.points.push(Point::new(-10., -10.));
    let mut links = Vec::new();
    links.push(Link::new(0, 1));
    links.push(Link::new(1, 3));
    links.push(Link::new(3, 2));
    links.push(Link::new(2, 0));
    force.link(1., links);
    assert!(force.points[0].vx == 2.5);
    assert!(force.points[0].vy == 2.5);
    assert!(force.points[1].vx == 2.5);
    assert!(force.points[1].vy == -2.5);
    assert!(force.points[2].vx == -2.5);
    assert!(force.points[2].vy == 2.5);
    assert!(force.points[3].vx == -2.5);
    assert!(force.points[3].vy == -2.5);
}
