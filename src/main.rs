use clap::Parser;
use image::{DynamicImage, GenericImageView, ImageError};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the base image
    #[arg(short, long)]
    image: PathBuf,

    /// Path to the logo image
    #[arg(short, long)]
    logo: PathBuf,

    /// Logo size as percentage of the base image (between 1 and 100)
    #[arg(short, long, default_value_t = 5.0)]
    percentage: f32,

    /// Position of the logo (top-right, top-left, bottom-right, bottom-left)
    #[arg(short, long, default_value = "top-right")]
    position: String,

    /// Output path for the resulting image
    #[arg(short, long)]
    output: PathBuf,
}

#[derive(Debug)]
enum Position {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

impl From<&str> for Position {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "top-left" => Position::TopLeft,
            "bottom-right" => Position::BottomRight,
            "bottom-left" => Position::BottomLeft,
            _ => Position::TopRight, // Default to top-right for any unrecognized input
        }
    }
}

fn calculate_logo_dimensions(
    base_width: u32,
    base_height: u32,
    logo_aspect_ratio: f32,
    percentage: f32,
) -> (u32, u32) {
    // Calculate the area that the logo should occupy
    let base_area = (base_width * base_height) as f32;
    let logo_area = base_area * (percentage / 100.0);

    // Calculate logo dimensions while maintaining aspect ratio
    let logo_width = (logo_area * logo_aspect_ratio).sqrt() as u32;
    let logo_height = (logo_area / logo_aspect_ratio).sqrt() as u32;

    (logo_width, logo_height)
}

fn calculate_position(
    position: Position,
    base_width: u32,
    base_height: u32,
    logo_width: u32,
    logo_height: u32,
) -> (i64, i64) {
    // Changed return type to i64
    let padding = 20; // Padding from the edges

    match position {
        Position::TopRight => ((base_width - logo_width - padding) as i64, padding as i64),
        Position::TopLeft => (padding as i64, padding as i64),
        Position::BottomRight => (
            (base_width - logo_width - padding) as i64,
            (base_height - logo_height - padding) as i64,
        ),
        Position::BottomLeft => (padding as i64, (base_height - logo_height - padding) as i64),
    }
}

fn overlay_logo(
    mut base_image: DynamicImage,
    logo: DynamicImage,
    percentage: f32,
    position: Position,
) -> Result<DynamicImage, ImageError> {
    let (base_width, base_height) = base_image.dimensions();
    let logo_aspect_ratio = logo.width() as f32 / logo.height() as f32;

    // Calculate new logo dimensions
    let (logo_width, logo_height) =
        calculate_logo_dimensions(base_width, base_height, logo_aspect_ratio, percentage);

    // Resize logo
    let resized_logo = logo.resize(
        logo_width,
        logo_height,
        image::imageops::FilterType::Lanczos3,
    );

    // Calculate position
    let (x, y) = calculate_position(position, base_width, base_height, logo_width, logo_height);

    // Overlay the logo
    image::imageops::overlay(&mut base_image, &resized_logo, x, y);

    Ok(base_image)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Validate percentage
    if !(1.0..=100.0).contains(&args.percentage) {
        return Err("Percentage must be between 1 and 100".into());
    }

    // Load images
    let base_image = image::open(&args.image)?;
    let logo = image::open(&args.logo)?;

    // Process the image
    let position = Position::from(args.position.as_str());
    let result = overlay_logo(base_image, logo, args.percentage, position)?;

    // Save the result
    result.save(&args.output)?;
    println!("Successfully saved output image to {:?}", args.output);

    Ok(())
}
