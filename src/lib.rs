/// To specify in which color to draw either a path or the svg background
#[derive(Debug,PartialEq)]
pub enum Color {
    None,
    Black,
    Blue,
    Green,
    Red,
    RGB(u8,u8,u8),
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
            Color::RGB(r,g,b) => {
                path.push_str(&format!("rgb({},{},{})\" ",r,g,b));
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
            Color::RGB(r,g,b) => {
                path.push_str(&format!("rgb({},{},{})\" />",r,g,b));
            },
        }

        path
    }

    /// Adds (for example) `"M 0 0 ... L 100 100"` to the current path rules
    pub fn add_rule_raw(&mut self, rule: String) {
        self.rules.push(rule);
    }

    /// Returns a `String` with path information. Example: `"M 0 0 L 100 100 ..."`
    pub fn create_raw(&mut self) -> String {
        let mut path = String::new();

        for rule in &self.rules {
            path.push_str(rule);
        }

        path
    }
}

/// Create an svg structure to hold one or more paths with options to set the viewbox, xmlns, and background color
pub struct MinSVG {
    viewbox: [usize;4],
    xmlns: Option<String>,
    paths: Vec<Path>,
    background: Color,
}

impl MinSVG {
    /// Construct a new svg with the required viewBox
    /// 
    /// Example: `[0,0,100,100]` will result in `viewBox=\"0 0 100 100\"`
    pub fn new(viewbox: [usize;4]) -> MinSVG {
        MinSVG {
            viewbox,
            xmlns: None,
            paths: Vec::new(),
            background: Color::None,
        }
    }

    /// Set a custom namespace. If not invoked the namespace will be `http://www.w3.org/2000/svg`
    pub fn set_xmlns(&mut self, xmlns: String) {
        self.xmlns = Some(xmlns);
    }

    /// Set a background color. If not invoked there will be none.
    pub fn set_background_color(&mut self, color: Color) {
        self.background = color;
    }

    /// Add a path created with `svg_minimal::Path` to the svg.
    pub fn add_path(&mut self, path: Path) {
        self.paths.push(path);
    }

    /// Will return a complete svg with all the requirements.
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
            Color::RGB(r,g,b) => {
                svg.push_str(&format!("<rect width=\"{}\" height=\"{}\" style=\"fill:rgb({},{},{})\" />", self.viewbox[2], self.viewbox[3],r,g,b));
            },
        }

        for path in &mut self.paths {
            svg.push_str(&*path.create());
        }

        svg.push_str("</svg>");

        svg
    }

    /// Will return an svg without the `<svg>` tag.
    pub fn create_raw(&mut self) -> String {
        let mut svg = String::new();

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
            Color::RGB(r,g,b) => {
                svg.push_str(&format!("<rect width=\"{}\" height=\"{}\" style=\"fill:rgb({},{},{})\" />", self.viewbox[2], self.viewbox[3],r,g,b));
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
    fn test_create_raw_path() {
        let mut path = Path::new();
        path.move_to([0,0]);
        path.line_to([100,100]);
        assert_eq!("M 0 0 L 100 100 ".to_string(),path.create_raw());
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
    fn test_svg_and_path_rgb() {
        let mut svg = MinSVG::new([0,0,100,100]);
        svg.set_background_color(Color::RGB(0,0,0));

        let mut path = Path::new();
        path.set_stroke_color(Color::RGB(10,100,50));

        svg.add_path(path);

        assert_eq!("<rect width=\"100\" height=\"100\" style=\"fill:rgb(0,0,0)\" /><path d=\"\" stroke=\"rgb(10,100,50)\" stroke-width=\"0\" fill=\"none\" /></svg>".to_string(),svg.create_raw());
    }

    #[test]
    fn test_create_svg() {
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

        assert_eq!("<svg viewBox=\"0 0 500 500\" xmlns=\"http://www.w3.org/2000/svg\"><rect width=\"500\" height=\"500\" style=\"fill:green\" /><path d=\"M 0 0 L 0 50 L 450 500 L 500 500 L 0 0 \" stroke=\"black\" stroke-width=\"3\" fill=\"black\" /></svg>".to_string(),svg.create());

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
