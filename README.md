# Artigen

Artigen Rust library to generate patterns. Uses `image` and `imageproc` for drawing utilities. Can output patterns in either base64 strings or image file on the file system

For an example of usage, see the sample [Android app](https://github.com/shaejaz/artigen)

## Usage

```rust
use artigen::patterns::{blocks};
use artigen::output::file;

fn main() {
    let blocks_pattern = blocks::Blocks {
        config: blocks::Config {
            x: 800,
            y: 640,
            color1: "#000000",
            color2: "#FFFFFF",
            color3: "#FF0000",
            bg_color: "#FFFFFF",
            block_size: 30,
            line_size: 30,
            density: 30.0,
        }
    };

    let img = blocks_pattern.generate();
    file::save_image(&img, "blocks.png");
}
```