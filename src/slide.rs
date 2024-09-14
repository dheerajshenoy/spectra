use std::vec::Vec;
use serde::{Serialize, Deserialize};

// This is used to store the presentation file contents
#[derive(Deserialize)]
pub struct SpectraFile {
    pub properties: SlideProperties,
    pub slides: Vec<Slide>,
}

#[derive(Deserialize)]
pub enum ElementType {
    Text,
    Image,
    BulletPoints,
    Video,
    LaTeX,
    Table
}

#[derive(Deserialize)]
pub struct Element {
    pub oftype: String,
    pub content: Option<String>,  // For text, images, etc.
    pub bullets: Option<Vec<String>>,  // For bullet points
    pub font_size: Option<f32>, // For Text and Bullet Points
    pub color: Option<String>,
    pub background_color: Option<String>,
    pub size: Option<Size>,
    pub autoplay: Option<bool>, // For Video and GIF
}

#[derive(Deserialize)]
pub struct Slide {
    pub layout: String,
    pub elements: Vec<Element>,
}

#[derive(Deserialize)]
struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize)]
pub struct SlideProperties {
    pub paper_size: Option<String>,
    pub background: Option<String>,
    pub font_size: Option<String>,
    pub color: Option<String>
}