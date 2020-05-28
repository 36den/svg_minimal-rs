/// To specify in which color to draw the path or fill it
#[derive(Debug,PartialEq)]
pub enum Color {
    None,
    Black,
    Blue,
    Green,
    Red,
}

/// Create a path by specifying stroke, stroke-width, fill and rules such as Move To, etc.
pub struct Path {
    rules: Vec<String>,
    stroke: Color,
    stroke_width: usize,
    fill: Color,
}

impl Path {
    pub fn new() -> Path {
        Path {
            rules: Vec::new(),
            stroke: Color::None,
            stroke_width: 0,
            fill: Color::None,
        }
    }

    /// Sets `stroke=\"YourColor\"`. If unset it will remain as `stroke="none"`.
    pub fn set_stroke_color(&mut self, color: Color) {
        self.stroke = color;
    }

    /// Sets `stroke-width=\"YourWidth\"`. If unset it will remain as `stroke-width=\"0\"`.
    pub fn set_stroke_width(&mut self, width: usize) {
        self.stroke_width = width;
    }

    /// Sets fill=\"YourColor\". If unset it will remain as `fill=\"none\"`.
    pub fn set_fill_color(&mut self, color: Color) {
        self.fill = color;
    }

    /// Adds rule `"M x y"`
    pub fn move_to(&mut self, pos: [usize;2]) {
        self.rules.push(format!("M {} {} ",pos[0],pos[1]));
    }

    /// Adds rule `"l x y"`
    pub fn line_to(&mut self, pos: [usize;2]) {
        self.rules.push(format!("L {} {} ",pos[0],pos[1]));
    }

    /// Adds rule `"c x1 y1, x2 y2, x y"`
    pub fn bezier(&mut self, points: [usize;6]) {
        self.rules.push(format!("C {} {}, {} {}, {} {} ",points[0],points[1],points[2],points[3],points[4],points[5]));
    }

    /// Closes the path with `'Z'`
    pub fn close_path(&mut self) {
        self.rules.push("Z ".to_string());
    }

    /// Removes the last rule
    pub fn undo(&mut self) {
        self.rules.pop();
    }

    /// Returns a `String` with a path of type `"<path d=\"...\" stroke=\"...\" stroke-width=\"...\" fill=\"...\" />"`
    pub fn create(&mut self) -> String {
        let mut path = String::new();

        path.push_str(&format!("<path d=\""));

        for rule in &self.rules {
            path.push_str(&rule);
        }
        

        path.push_str("\" stroke=\"");

        match &self.stroke {
            Color::None => {
                path.push_str("none\" ");
            },
            Color::Black => {
                path.push_str("black\" ");
            },
            Color::Blue => {
                path.push_str("blue\" ");
            },
            Color::Green => {
                path.push_str("green\" ");
            },
            Color::Red => {
                path.push_str("red\" ");
            },
        }

        path.push_str(&format!("stroke-width=\"{}\" fill=\"", self.stroke_width));

        match &self.fill {
            Color::None => {
                path.push_str("none\" />");
            },
            Color::Black => {
                path.push_str("black\" />");
            },
            Color::Blue => {
                path.push_str("blue\" />");
            },
            Color::Green => {
                path.push_str("green\" />");
            },
            Color::Red => {
                path.push_str("red\" />");
            },
        }

        path
    }

    pub fn create_raw(&mut self) -> String {
        let mut path = String::new();

        for rule in &self.rules {
            path.push_str(rule);
        }

        path
    }
}

pub struct MinSVG {
    viewbox: [usize;4],
    xmlns: Option<String>,
    paths: Vec<Path>,
    background: Color,
}

impl MinSVG {
    pub fn new(viewbox: [usize;4]) -> MinSVG {
        MinSVG {
            viewbox,
            xmlns: None,
            paths: Vec::new(),
            background: Color::None,
        }
    }

    pub fn set_xmlns(&mut self, xmlns: String) {
        self.xmlns = Some(xmlns);
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background = color;
    }

    pub fn add_path(&mut self, path: Path) {
        self.paths.push(path);
    }

    pub fn create(&mut self) -> String {
        let mut svg = String::new();

        match &self.xmlns {
            None => {
                svg.push_str(&format!("<svg viewBox=\"{} {} {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
                                self.viewbox[0],self.viewbox[1],self.viewbox[2],self.viewbox[3]));
            },
            Some(xmlns) => {
                svg.push_str(&format!("<svg viewBox=\"{} {} {} {}\" xmlns=\"{}\">",
                                self.viewbox[0],self.viewbox[1],self.viewbox[2],self.viewbox[3],xmlns));
            }
        }

        match &self.background {
            Color::None => {

            },
            Color::Black => {
                svg.push_str(&format!("<rect width=\"{}\" height=\"{}\" style=\"fill:{}\" />", self.viewbox[2], self.viewbox[3],"black"));
            },
            Color::Blue => {
                svg.push_str(&format!("<rect width=\"{}\" height=\"{}\" style=\"fill:{}\" />", self.viewbox[2], self.viewbox[3],"blue"));
            },
            Color::Green => {
                svg.push_str(&format!("<rect width=\"{}\" height=\"{}\" style=\"fill:{}\" />", self.viewbox[2], self.viewbox[3],"green"));
            },
            Color::Red => {
                svg.push_str(&format!("<rect width=\"{}\" height=\"{}\" style=\"fill:{}\" />", self.viewbox[2], self.viewbox[3],"red"));
            },
        }

        for path in &mut self.paths {
            svg.push_str(&*path.create());
        }

        svg.push_str("</svg>");

        svg
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_path() {
        let path = Path::new();
        assert_eq!(0,path.rules.len());
        assert_eq!(Color::None,path.stroke);
        assert_eq!(0,path.stroke_width);
        assert_eq!(Color::None,path.fill);
    }

    #[test]
    fn test_add_path_rule() {
        let mut path = Path::new();
        path.move_to([0,0]);
        path.line_to([100,100]);
        assert_eq!("<path d=\"M 0 0 L 100 100 \" stroke=\"none\" stroke-width=\"0\" fill=\"none\" />".to_string(),path.create());
    }

    #[test]
    fn test_construct_svg() {
        let svg = MinSVG::new([0,0,100,100]);
        assert_eq!([0,0,100,100], svg.viewbox);
        assert_eq!(None, svg.xmlns);
    }

    #[test]
    fn test_add_xmlns() {
        let mut svg = MinSVG::new([0,0,100,100]);
        svg.set_xmlns("Some namespace".to_string());
        assert_eq!(Some("Some namespace".to_string()),svg.xmlns);
    }

    #[test]
    fn test_without_xmlns() {
        let svg = MinSVG::new([0,0,100,100]);
        assert_eq!(None,svg.xmlns);
    }

    #[test]
    fn test_create_svg1() {
        use std::fs::File;
        use std::io::prelude::*;

        let mut svg = MinSVG::new([0,0,500,500]);

        let mut path = Path::new();
        path.set_stroke_color(Color::Black);
        path.set_fill_color(Color::Black);
        path.set_stroke_width(3);
        path.move_to([0,0]);
        path.line_to([0,50]);
        path.line_to([450,500]);
        path.line_to([500,500]);
        path.line_to([0,0]);

        svg.add_path(path);

        svg.set_background_color(Color::Green);

        match File::create("test.svg") {
            Ok(mut file) => {
                match file.write_all(svg.create().as_bytes()) {
                    Ok(()) => {
                        assert!(true);
                    },
                    Err(e) => {
                        panic!(e);
                    }
                }
            },
            Err(e) => {
                panic!(e);
            }
        }
    }

 }
