extern crate fd_layout;

use std::f32;
use fd_layout::force::{Point, Link, Force};
use fd_layout::many_body_force::ManyBodyForce;
use fd_layout::link_force::LinkForce;
use fd_layout::center_force::CenterForce;

fn main() {
    let mut points = Vec::new();
    let n = 77;
    for i in 0..n {
        let r = (i as usize as f32).sqrt();
        let theta = f32::consts::PI * (3. - (5. as f32).sqrt()) * (i as usize as f32);
        let x = r * theta.cos();
        let y = r * theta.sin();
        points.push(Point::new(x, y));
    }
    let mut links = Vec::new();
    links.push(Link::new(1, 0));
    links.push(Link::new(2, 0));
    links.push(Link::new(3, 0));
    links.push(Link::new(3, 2));
    links.push(Link::new(4, 0));
    links.push(Link::new(5, 0));
    links.push(Link::new(6, 0));
    links.push(Link::new(7, 0));
    links.push(Link::new(8, 0));
    links.push(Link::new(9, 0));
    links.push(Link::new(11, 10));
    links.push(Link::new(11, 3));
    links.push(Link::new(11, 2));
    links.push(Link::new(11, 0));
    links.push(Link::new(12, 11));
    links.push(Link::new(13, 11));
    links.push(Link::new(14, 11));
    links.push(Link::new(15, 11));
    links.push(Link::new(17, 16));
    links.push(Link::new(18, 16));
    links.push(Link::new(18, 17));
    links.push(Link::new(19, 16));
    links.push(Link::new(19, 17));
    links.push(Link::new(19, 18));
    links.push(Link::new(20, 16));
    links.push(Link::new(20, 17));
    links.push(Link::new(20, 18));
    links.push(Link::new(20, 19));
    links.push(Link::new(21, 16));
    links.push(Link::new(21, 17));
    links.push(Link::new(21, 18));
    links.push(Link::new(21, 19));
    links.push(Link::new(21, 20));
    links.push(Link::new(22, 16));
    links.push(Link::new(22, 17));
    links.push(Link::new(22, 18));
    links.push(Link::new(22, 19));
    links.push(Link::new(22, 20));
    links.push(Link::new(22, 21));
    links.push(Link::new(23, 16));
    links.push(Link::new(23, 17));
    links.push(Link::new(23, 18));
    links.push(Link::new(23, 19));
    links.push(Link::new(23, 20));
    links.push(Link::new(23, 21));
    links.push(Link::new(23, 22));
    links.push(Link::new(23, 12));
    links.push(Link::new(23, 11));
    links.push(Link::new(24, 23));
    links.push(Link::new(24, 11));
    links.push(Link::new(25, 24));
    links.push(Link::new(25, 23));
    links.push(Link::new(25, 11));
    links.push(Link::new(26, 24));
    links.push(Link::new(26, 11));
    links.push(Link::new(26, 16));
    links.push(Link::new(26, 25));
    links.push(Link::new(27, 11));
    links.push(Link::new(27, 23));
    links.push(Link::new(27, 25));
    links.push(Link::new(27, 24));
    links.push(Link::new(27, 26));
    links.push(Link::new(28, 11));
    links.push(Link::new(28, 27));
    links.push(Link::new(29, 23));
    links.push(Link::new(29, 27));
    links.push(Link::new(29, 11));
    links.push(Link::new(30, 23));
    links.push(Link::new(31, 30));
    links.push(Link::new(31, 11));
    links.push(Link::new(31, 23));
    links.push(Link::new(31, 27));
    links.push(Link::new(32, 11));
    links.push(Link::new(33, 11));
    links.push(Link::new(33, 27));
    links.push(Link::new(34, 11));
    links.push(Link::new(34, 29));
    links.push(Link::new(35, 11));
    links.push(Link::new(35, 34));
    links.push(Link::new(35, 29));
    links.push(Link::new(36, 34));
    links.push(Link::new(36, 35));
    links.push(Link::new(36, 11));
    links.push(Link::new(36, 29));
    links.push(Link::new(37, 34));
    links.push(Link::new(37, 35));
    links.push(Link::new(37, 36));
    links.push(Link::new(37, 11));
    links.push(Link::new(37, 29));
    links.push(Link::new(38, 34));
    links.push(Link::new(38, 35));
    links.push(Link::new(38, 36));
    links.push(Link::new(38, 37));
    links.push(Link::new(38, 11));
    links.push(Link::new(38, 29));
    links.push(Link::new(39, 25));
    links.push(Link::new(40, 25));
    links.push(Link::new(41, 24));
    links.push(Link::new(41, 25));
    links.push(Link::new(42, 41));
    links.push(Link::new(42, 25));
    links.push(Link::new(42, 24));
    links.push(Link::new(43, 11));
    links.push(Link::new(43, 26));
    links.push(Link::new(43, 27));
    links.push(Link::new(44, 28));
    links.push(Link::new(44, 11));
    links.push(Link::new(45, 28));
    links.push(Link::new(47, 46));
    links.push(Link::new(48, 47));
    links.push(Link::new(48, 25));
    links.push(Link::new(48, 27));
    links.push(Link::new(48, 11));
    links.push(Link::new(49, 26));
    links.push(Link::new(49, 11));
    links.push(Link::new(50, 49));
    links.push(Link::new(50, 24));
    links.push(Link::new(51, 49));
    links.push(Link::new(51, 26));
    links.push(Link::new(51, 11));
    links.push(Link::new(52, 51));
    links.push(Link::new(52, 39));
    links.push(Link::new(53, 51));
    links.push(Link::new(54, 51));
    links.push(Link::new(54, 49));
    links.push(Link::new(54, 26));
    links.push(Link::new(55, 51));
    links.push(Link::new(55, 49));
    links.push(Link::new(55, 39));
    links.push(Link::new(55, 54));
    links.push(Link::new(55, 26));
    links.push(Link::new(55, 11));
    links.push(Link::new(55, 16));
    links.push(Link::new(55, 25));
    links.push(Link::new(55, 41));
    links.push(Link::new(55, 48));
    links.push(Link::new(56, 49));
    links.push(Link::new(56, 55));
    links.push(Link::new(57, 55));
    links.push(Link::new(57, 41));
    links.push(Link::new(57, 48));
    links.push(Link::new(58, 55));
    links.push(Link::new(58, 48));
    links.push(Link::new(58, 27));
    links.push(Link::new(58, 57));
    links.push(Link::new(58, 11));
    links.push(Link::new(59, 58));
    links.push(Link::new(59, 55));
    links.push(Link::new(59, 48));
    links.push(Link::new(59, 57));
    links.push(Link::new(60, 48));
    links.push(Link::new(60, 58));
    links.push(Link::new(60, 59));
    links.push(Link::new(61, 48));
    links.push(Link::new(61, 58));
    links.push(Link::new(61, 60));
    links.push(Link::new(61, 59));
    links.push(Link::new(61, 57));
    links.push(Link::new(61, 55));
    links.push(Link::new(62, 55));
    links.push(Link::new(62, 58));
    links.push(Link::new(62, 59));
    links.push(Link::new(62, 48));
    links.push(Link::new(62, 57));
    links.push(Link::new(62, 41));
    links.push(Link::new(62, 61));
    links.push(Link::new(62, 60));
    links.push(Link::new(63, 59));
    links.push(Link::new(63, 48));
    links.push(Link::new(63, 62));
    links.push(Link::new(63, 57));
    links.push(Link::new(63, 58));
    links.push(Link::new(63, 61));
    links.push(Link::new(63, 60));
    links.push(Link::new(63, 55));
    links.push(Link::new(64, 55));
    links.push(Link::new(64, 62));
    links.push(Link::new(64, 48));
    links.push(Link::new(64, 63));
    links.push(Link::new(64, 58));
    links.push(Link::new(64, 61));
    links.push(Link::new(64, 60));
    links.push(Link::new(64, 59));
    links.push(Link::new(64, 57));
    links.push(Link::new(64, 11));
    links.push(Link::new(65, 63));
    links.push(Link::new(65, 64));
    links.push(Link::new(65, 48));
    links.push(Link::new(65, 62));
    links.push(Link::new(65, 58));
    links.push(Link::new(65, 61));
    links.push(Link::new(65, 60));
    links.push(Link::new(65, 59));
    links.push(Link::new(65, 57));
    links.push(Link::new(65, 55));
    links.push(Link::new(66, 64));
    links.push(Link::new(66, 58));
    links.push(Link::new(66, 59));
    links.push(Link::new(66, 62));
    links.push(Link::new(66, 65));
    links.push(Link::new(66, 48));
    links.push(Link::new(66, 63));
    links.push(Link::new(66, 61));
    links.push(Link::new(66, 60));
    links.push(Link::new(67, 57));
    links.push(Link::new(68, 25));
    links.push(Link::new(68, 11));
    links.push(Link::new(68, 24));
    links.push(Link::new(68, 27));
    links.push(Link::new(68, 48));
    links.push(Link::new(68, 41));
    links.push(Link::new(69, 25));
    links.push(Link::new(69, 68));
    links.push(Link::new(69, 11));
    links.push(Link::new(69, 24));
    links.push(Link::new(69, 27));
    links.push(Link::new(69, 48));
    links.push(Link::new(69, 41));
    links.push(Link::new(70, 25));
    links.push(Link::new(70, 69));
    links.push(Link::new(70, 68));
    links.push(Link::new(70, 11));
    links.push(Link::new(70, 24));
    links.push(Link::new(70, 27));
    links.push(Link::new(70, 41));
    links.push(Link::new(70, 58));
    links.push(Link::new(71, 27));
    links.push(Link::new(71, 69));
    links.push(Link::new(71, 68));
    links.push(Link::new(71, 70));
    links.push(Link::new(71, 11));
    links.push(Link::new(71, 48));
    links.push(Link::new(71, 41));
    links.push(Link::new(71, 25));
    links.push(Link::new(72, 26));
    links.push(Link::new(72, 27));
    links.push(Link::new(72, 11));
    links.push(Link::new(73, 48));
    links.push(Link::new(74, 48));
    links.push(Link::new(74, 73));
    links.push(Link::new(75, 69));
    links.push(Link::new(75, 68));
    links.push(Link::new(75, 25));
    links.push(Link::new(75, 48));
    links.push(Link::new(75, 41));
    links.push(Link::new(75, 70));
    links.push(Link::new(75, 71));
    links.push(Link::new(76, 64));
    links.push(Link::new(76, 65));
    links.push(Link::new(76, 66));
    links.push(Link::new(76, 63));
    links.push(Link::new(76, 62));
    links.push(Link::new(76, 48));
    links.push(Link::new(76, 58));

    let many_body_force = ManyBodyForce::new();
    let link_force = LinkForce::new(&links);
    let center_force = CenterForce::new();

    let mut alpha = 1.;
    let alpha_min = 0.001;
    let alpha_decay = 1. - (alpha_min as f32).powf(1. / 300.);
    let alpha_target = 0.;
    let velocity_decay = 0.6;
    loop {
        alpha += (alpha_target - alpha) * alpha_decay;
        link_force.apply(&mut points, alpha);
        many_body_force.apply(&mut points, alpha);
        center_force.apply(&mut points, alpha);
        for point in points.iter_mut() {
            point.vx *= velocity_decay;
            point.x += point.vx;
            point.vy *= velocity_decay;
            point.y += point.vy;
        }
        if alpha < alpha_min {
            break;
        }
    }

    let width = 800.;
    let height = 800.;
    let margin = 10.;
    println!(
        "<svg version=\"1.1\" width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">",
        width + margin * 2., height + margin * 2.,
    );
    println!(
        "<g transform=\"translate({},{})\">",
        width / 2. + margin,
        height / 2. + margin,
    );
    for link in links.iter() {
        let x1 = points[link.source].x;
        let y1 = points[link.source].y;
        let x2 = points[link.target].x;
        let y2 = points[link.target].y;
        println!(
            "<line x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\" stroke=\"#888\" />",
            x1,
            x2,
            y1,
            y2
        );
    }
    for point in points.iter() {
        println!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"5\" fill=\"green\" />",
            point.x,
            point.y
        );
    }
    println!("</g>\n</svg>");
}
