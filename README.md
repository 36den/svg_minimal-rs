# svg_minimal-rs

A minimal svg library making use of the `<path>` tag and little else. The only path options are `M: move to`, `L: line to`, `C: bezier curve` and `Z: close path`.

## Example
```Rust
    use svg_minimal::*;

    fn main() {
        let mut svg = MinSVG::new([0,0,100,100]).
        svg.set_background_color(Color::Green);

        let mut path = Path::new();
        path.set_stroke_color(Color::Black);
        path.set_stroke_width(3);

        path.move_to([0,0]);
        path.line_to([100,100]);
        path.bezier([100,80,20,0,0,0]); // Hope this is acceptable

        svg.add_path(path);

        let svg_string = svg.create();
    }
```
