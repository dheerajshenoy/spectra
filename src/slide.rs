use std::vec::Vec;
use serde::{Serialize, Deserialize};

// This is used to store the presentation file contents
#[derive(Deserialize)]
pub struct SpectraFile {
    pub properties: SlideProperties,
    pub slides: Vec<Slide>,
}

#[derive(Deserialize)]
enum ElementType {
    Text,
    Image,
    BulletPoints,
    Video,
    LaTeX,
    Table
}

#[derive(Deserialize)]
struct Element {
    element: Option<String>,
    content: Option<String>,  // For text, images, etc.
    bullets: Option<Vec<String>>,  // For bullet points
    font_size: Option<u32>, // For Text and Bullet Points
    color: Option<String>,
    background_color: Option<String>,
    size: Option<Size>,
    autoplay: Option<bool>, // For Video and GIF
}

#[derive(Deserialize)]
pub struct Slide {
    layout: String,
    elements: Vec<Element>,
}

#[derive(Deserialize)]
struct Size {
    width: u32,
    height: u32,
}

#[derive(Deserialize)]
pub struct SlideProperties {
    pub paper_size: Option<String>,
    pub background: Option<String>,
    pub font_size: Option<String>,
    pub color: Option<String>
}