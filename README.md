# svg_minimal-rs

A minimal svg library making use of the `<path>` tag and little else. The only path options are `M: move to`, `L: line to`, `C: bezier curve` and `Z: close path`.

## Example
```rust
    use svg_minimal::*;

    fn main() {
        let mut svg = MinSVG::new([0,0,100,100]). // Construct an svg with the viewBox you would like
        svg.set_background_color(Color::Green); // If not invoked there will be no background

        let mut path = Path::new(); // Construct a new path element. I guess you only really need "one per color".
        path.set_stroke_color(Color::Black); // If not invoked the color is none
        path.set_stroke_width(3); // If not invoked the stroke-width is 0

        path.move_to([0,0]);
        path.line_to([100,100]);
        path.bezier([100,80,20,0,0,0]); // Hope this works :)

        path.set_fill_color(Color::Black); // Fill the drawn path if you want to;

        svg.add_path(path); // Add the path to the svg

        let svg_string = svg.create(); // Returns the svg as a string
    }
```
