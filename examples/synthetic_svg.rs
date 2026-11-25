//! Synthetic demo harness: no filesystem, no renderer deps.
//!
//! Builds a fake project tree, runs [`cushion_treemap::hierarchical_layout`],
//! and writes a self-contained SVG where every tile carries a native hover
//! tooltip (`<title>` with name + size).
//!
//! Run with `cargo run --example synthetic_svg`, then open the printed path
//! in a browser.

use cushion_treemap::{depth_color, extension_color, hierarchical_layout, LayoutRect, TreemapTree};
use std::fmt::Write as _;

struct Node {
    name: String,
    ext: String,
    size: u64,
    folder: bool,
    children: Vec<Node>,
}

impl Node {
    fn file(name: &str, size: u64) -> Self {
        let ext = name.rsplit('.').next().unwrap_or("").to_lowercase();
        Self {
            name: name.to_string(),
            ext,
            size,
            folder: false,
            children: Vec::new(),
        }
    }

    fn folder(name: &str, children: Vec<Node>) -> Self {
        let size = children.iter().map(|c| c.size).sum();
        Self {
            name: name.to_string(),
            ext: String::new(),
            size,
            folder: true,
            children,
        }
    }
}

impl TreemapTree for Node {
    fn treemap_size(&self) -> u64 {
        self.size
    }
    fn treemap_children(&self) -> &[Self] {
        &self.children
    }
}

fn node_at<'a>(root: &'a Node, path: &[usize]) -> Option<&'a Node> {
    let mut node = root;
    for step in path {
        node = node.children.get(*step)?;
    }
    Some(node)
}

fn xml_escape(into: &mut String, text: &str) {
    for ch in text.chars() {
        match ch {
            '&' => into.push_str("&amp;"),
            '<' => into.push_str("&lt;"),
            '>' => into.push_str("&gt;"),
            '"' => into.push_str("&quot;"),
            _ => into.push(ch),
        }
    }
}

fn fmt_size(size: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut value = size as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{size} B")
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

fn run() -> std::io::Result<String> {
    let root = Node::folder(
        "demo-project",
        vec![
            Node::folder(
                "videos",
                vec![
                    Node::file("trailer.mp4", 820_000_000),
                    Node::file("clip.mkv", 310_000_000),
                ],
            ),
            Node::folder(
                "archives",
                vec![
                    Node::file("backup.zip", 450_000_000),
                    Node::file("image.iso", 1_100_000_000),
                ],
            ),
            Node::folder(
                "src",
                vec![
                    Node::file("main.rs", 48_000),
                    Node::file("layout.rs", 32_000),
                    Node::file("engine.dll", 12_000_000),
                ],
            ),
            Node::file("model.safetensors", 2_400_000_000),
            Node::file("report.pdf", 4_000_000),
        ],
    );

    let bounds = LayoutRect {
        x: 0.0,
        y: 0.0,
        w: 960.0,
        h: 600.0,
    };
    let tiles = hierarchical_layout(&root, bounds, 5);

    let mut svg = String::new();
    svg.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"960\" height=\"600\" font-family=\"sans-serif\">\n");
    svg.push_str("<rect width=\"960\" height=\"600\" fill=\"#0b0f19\"/>\n");
    for tile in &tiles {
        let Some(node) = node_at(&root, &tile.index_path) else {
            continue;
        };
        let color = if node.folder {
            depth_color(tile.depth)
        } else {
            extension_color(&node.ext, false)
        };
        let rect = &tile.rect;
        let _ = write!(
            svg,
            "<g><title>{} - {} ({} bytes)</title>",
            xml_escaped(&node.name),
            fmt_size(node.size),
            node.size
        );
        let _ = write!(
            svg,
            "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"rgb({},{},{})\" stroke=\"#000\" stroke-opacity=\"0.6\"/>",
            rect.x, rect.y, rect.w, rect.h, color[0], color[1], color[2]
        );
        if rect.w > 60.0 && rect.h > 20.0 {
            let _ = write!(
                svg,
                "<text x=\"{:.1}\" y=\"{:.1}\" font-size=\"11\" fill=\"#fff\">{}</text>",
                rect.x + 5.0,
                rect.y + 14.0,
                xml_escaped(&node.name)
            );
        }
        svg.push_str("</g>\n");
    }
    svg.push_str("</svg>\n");

    let out = std::env::temp_dir().join("cushion_treemap_demo.svg");
    std::fs::write(&out, svg)?;
    Ok(out.display().to_string())
}

fn xml_escaped(text: &str) -> String {
    let mut escaped = String::new();
    xml_escape(&mut escaped, text);
    escaped
}

fn main() {
    match run() {
        Ok(path) => println!("demo treemap written to {path} - open it in a browser"),
        Err(e) => {
            eprintln!("demo failed: {e}");
            std::process::exit(1);
        }
    }
}
