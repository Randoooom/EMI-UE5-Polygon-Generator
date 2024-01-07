use std::f32::consts::PI;

const WIDTH: usize = 100;

struct Polygon {
    points: Vec<(f32, f32)>,
    name: String,
}

impl Polygon {
    pub fn regular(edges: usize) -> Self {
        let degree = 2f32 * PI / edges as f32;
        let mut points = Vec::new();

        for i in 0..edges {
            let r = (WIDTH / 2) as f32;
            let x = r * f32::cos(degree * i as f32);
            let y = r * f32::sin(degree * i as f32);

            points.push((x, y));
        }

        Self {
            points,
            name: format!("{}-gon", edges),
        }
    }

    pub fn star(spikes: usize) -> Self {
        let degree = 2f32 * PI / spikes as f32;
        let mut points = Vec::new();

        for i in 0..spikes {
            let r = (WIDTH / 2) as f32;

            // spike
            let x = r * f32::cos(degree * i as f32);
            let y = r * f32::sin(degree * i as f32);
            points.push((x, y));

            // inner
            let x = (r / 2.0) * f32::cos(degree * (i as f32+ 0.5));
            let y = (r / 2.0) * f32::sin(degree * (i as f32 + 0.5));
            points.push((x, y));
        }

        Self {
            points,
            name: format!("{}-star", spikes),
        }
    }
}

impl ToString for Polygon {
    fn to_string(&self) -> String {
        format!(
            "<defs>\n   <g id=\"{}\"> \n        <polygon points=\"{}\" fill=\"yellow\" />\n   </g>\n</defs>",
            self.name,
            self.points.iter().map(|(x, y)| format!("{},{}", x, y)).collect::<Vec<String>>().join(" ")
        )
    }
}

fn main() {
    for i in vec![5, 8, 10, 15, 18, 30].iter() {
        println!("===============");
        println!("n = {}", i);
        println!("{}", Polygon::regular(i.clone()).to_string());

        println!("\n\nStar:");
        println!("n = {}", i);
        println!("{}", Polygon::star(i.clone()).to_string());
    }
}
