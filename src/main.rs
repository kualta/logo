use clap::Parser;
use image::{DynamicImage, GenericImageView, ImageError};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the base image
    #[arg(short, long)]
    image: PathBuf,

    /// Path to the logo image
    #[arg(short, long, default_value = "./logo.png")]
    logo: PathBuf,

    /// Logo size as percentage of the base image (between 1 and 100)
    #[arg(long, default_value_t = 10.0)]
    percentage: f32,

    /// Position of the logo (top-right, top-left, bottom-right, bottom-left)
    #[arg(short, long, default_value = "top-right")]
    position: String,

    /// Output path for the resulting image
    #[arg(short, long)]
    output: Option<PathBuf>,
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

fn generate_random_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn calculate_logo_dimensions(
    base_width: u32,
    _base_height: u32,
    logo_aspect_ratio: f32,
    percentage: f32,
) -> (u32, u32) {
    // Calculate the target width based on the base image width and percentage
    let target_width = (base_width as f32 * (percentage / 100.0)) as u32;

    // Calculate height while maintaining aspect ratio
    let target_height = (target_width as f32 / logo_aspect_ratio) as u32;

    (target_width, target_height)
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
    println!(
        "Original logo dimensions: {}x{}",
        logo.width(),
        logo.height()
    );
    println!("New logo dimensions: {}x{}", logo_width, logo_height);

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
    let mut args = Args::parse();

    // Validate percentage
    if !(1.0..=100.0).contains(&args.percentage) {
        return Err("Percentage must be between 1 and 100".into());
    }

    // Generate default output path if not provided
    if args.output.is_none() {
        let random_suffix = generate_random_string(8);
        let input_stem = args
            .image
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("Invalid input filename")?;
        args.output = Some(PathBuf::from(format!(
            "{}_with_logo_{}.png",
            input_stem, random_suffix
        )));
    }

    // Load images
    let base_image = image::open(&args.image)?;
    let logo = image::open(&args.logo)?;

    // Process the image
    let position = Position::from(args.position.as_str());
    let result = overlay_logo(base_image, logo, args.percentage, position)?;

    // Save the result
    let output_path = args.output.as_ref().unwrap();
    result.save(output_path)?;
    println!(
        "Successfully saved output image to {:?}",
        output_path
    );

    Ok(())
}
