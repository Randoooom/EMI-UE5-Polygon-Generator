use std::f32::consts::PI;

const WIDTH: usize = 100;

struct Polygon(usize);

impl ToString for Polygon {
    fn to_string(&self) -> String {
        // calculate the degree
        let degree = 2f32 * PI / self.0 as f32;

        // builder
        let mut points = String::new();
        for i in 0..self.0 {
            let r = (WIDTH / 2) as f32;
            let x = r * f32::cos(degree * i as f32);
            let y = r * f32::sin(degree * i as f32);

            points.push_str(format!("{},{} ", x, y).as_str())
        }
       
        format!("<defs>\n   <g id=\"{}-gon\"> \n        <polygon points=\"{}\" fill=\"yellow\" />\n   </g>\n</defs>", self.0, points)
    }
}

fn main() {
    for i in vec![5, 8, 10, 15, 18, 30].iter() {
        println!("===============");
        println!("n = {}", i);
        println!("{}", Polygon(i.clone()).to_string());
    } 
}
