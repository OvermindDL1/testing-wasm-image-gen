mod utils;

use image::imageops::overlay;
use image::{ColorType, RgbImage};
use noise::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern "C" {
// 	fn alert(s: &str);
// }

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
	utils::set_panic_hook();

	web_sys::console::log_1(&JsValue::from_str("WASM Initialized"));
	println!("and from println");
	eprintln!("and from eprintln");

	// FileAssets::iter()
	// 	.for_each(|p| web_sys::console::log_2(&"Found included image".into(), &p.as_ref().into()));

	Ok(())
}

#[wasm_bindgen]
pub fn gen_background(id: &str, chunk_x: u32, chunk_y: u32, seed: u32) -> Option<Vec<u8>> {
	web_sys::console::log_5(
		&"Generating background".into(),
		&id.into(),
		&(chunk_x as u32).into(),
		&(chunk_y as u32).into(),
		&(seed as u32).into(),
	);
	utils::set_panic_hook();
	let data = match id {
		"factorio" => gen_background_factorio(chunk_x, chunk_y, seed),
		_ => Err(format!("Invalid ID: {}", id)),
	};
	match data {
		Ok(data) => Some(data),
		Err(reason) => {
			eprintln!("Failed to generate image due to:  {}", reason);
			web_sys::console::error_2(&"Failed to generate image due to:".into(), &reason.into());
			None
		}
	}
}

#[derive(rust_embed::RustEmbed)]
#[folder = "assets/"]
struct FileAssets;

#[derive(Default)]
pub struct Assets {
	images: HashMap<String, RgbImage>,
}

pub static ASSETS: Lazy<Mutex<Assets>> = Lazy::new(|| Mutex::new(Assets::default()));
impl Assets {
	pub fn get_image(&self, path: &str) -> Result<&RgbImage, String> {
		self.images
			.get(path)
			.ok_or_else(|| format!("Missing image at path: {}", path))
	}

	// Uncompress the images into data and store in a cache on demand and get the resultant image reference
	pub fn load_image(&mut self, path: &str) -> Result<&RgbImage, String> {
		if self.images.contains_key(path) {
			return self
				.images
				.get(path)
				.ok_or_else(|| format!("Failed to acquire image when it exists: {}", path));
		} else if let Some(data) = FileAssets::get(path) {
			let image = image::load_from_memory(data.data.as_ref())
				.map_err(|e| format!("Invalid image format for `{}`: {:?}", path, e))?
				.into_rgb8();
			return Ok(self.images.entry(path.into()).or_insert(image));
		} else {
			Err(format!(
				"requested an image that does not exist at the path: {}",
				path,
			))
		}
	}

	pub fn preload_images_starts_with(&mut self, starts_with: &str) -> Result<(), String> {
		for p in FileAssets::iter().filter(|s| s.starts_with(starts_with)) {
			self.load_image(&p)?;
		}
		Ok(())
	}
}

const FACTORIO_TILE_SIZE: u32 = 128;
const FACTORIO_CHUNK_SIZE: u32 = 1024;

fn gen_background_factorio(width: u32, height: u32, seed: u32) -> Result<Vec<u8>, String> {
	web_sys::console::log_1(&"Background Factorio pre-assets...".into());
	let mut assets = ASSETS
		.lock()
		.map_err(|e| format!("Corrupt Cache: {:?}", e))?;
	web_sys::console::log_1(&"Background Factorio new image...".into());
	let mut img = RgbImage::new(width, height);

	web_sys::console::log_1(&"Background Factorio preloading images...".into());
	assets.preload_images_starts_with("factorio")?;
	let p = noise::Perlin::new().set_seed(seed);
	web_sys::console::log_1(&"Background Factorio generating ground...".into());
	{
		let dirts = [
			assets.get_image("factorio/dirt0.png")?,
			assets.get_image("factorio/dirt1.png")?,
			assets.get_image("factorio/dirt2.png")?,
			assets.get_image("factorio/dirt3.png")?,
			assets.get_image("factorio/dirt4.png")?,
			assets.get_image("factorio/dirt5.png")?,
			assets.get_image("factorio/dirt6.png")?,
			assets.get_image("factorio/dirt7.png")?,
			assets.get_image("factorio/dirt8.png")?,
			assets.get_image("factorio/dirt9.png")?,
			assets.get_image("factorio/dirt10.png")?,
			assets.get_image("factorio/dirt11.png")?,
			assets.get_image("factorio/dirt12.png")?,
			assets.get_image("factorio/dirt13.png")?,
			assets.get_image("factorio/dirt14.png")?,
			assets.get_image("factorio/dirt15.png")?,
			assets.get_image("factorio/dirt16.png")?,
			assets.get_image("factorio/dirt17.png")?,
			assets.get_image("factorio/dirt18.png")?,
			assets.get_image("factorio/dirt19.png")?,
			assets.get_image("factorio/dirt20.png")?,
		];
		let grasses = [
			assets.get_image("factorio/grass0.png")?,
			assets.get_image("factorio/grass1.png")?,
			assets.get_image("factorio/grass2.png")?,
			assets.get_image("factorio/grass3.png")?,
			assets.get_image("factorio/grass4.png")?,
			assets.get_image("factorio/grass5.png")?,
			assets.get_image("factorio/grass6.png")?,
			assets.get_image("factorio/grass7.png")?,
			assets.get_image("factorio/grass8.png")?,
			assets.get_image("factorio/grass9.png")?,
			assets.get_image("factorio/grass10.png")?,
			assets.get_image("factorio/grass11.png")?,
			assets.get_image("factorio/grass12.png")?,
			assets.get_image("factorio/grass13.png")?,
			assets.get_image("factorio/grass14.png")?,
			assets.get_image("factorio/grass15.png")?,
		];
		// Render terrain
		for x in (0..width).step_by(FACTORIO_TILE_SIZE as usize) {
			for y in (0..height).step_by(FACTORIO_TILE_SIZE as usize) {
				let v = p.get([x as f64 * 0.001, y as f64 * 0.001]);
				let tile = if v < 0.0 {
					dirts[((-v * dirts.len() as f64) as usize).min(dirts.len() - 1)]
				} else {
					grasses[((v * grasses.len() as f64) as usize).min(grasses.len() - 1)]
				};
				overlay(&mut img, tile, x, y);
			}
		}
	}

	web_sys::console::log_1(&"Background Factorio encode into image...".into());
	let (width, height) = img.dimensions();
	let mut bytes = Vec::with_capacity((width * height * 3) as usize);
	// For PNG compressed
	// DynamicImage::ImageRgb8(img)
	// 	.write_to(&mut bytes, image::ImageOutputFormat::Png)
	// 	.map_err(|e| format!("invalid png encoding: {:?}", e))?;
	// For PNG barely compressed
	// image::codecs::png::PngEncoder::new_with_quality(
	// 	&mut bytes,
	// 	CompressionType::Rle,
	// 	FilterType::NoFilter,
	// )
	// .encode(&img, width, height, ColorType::Rgb8)
	// .map_err(|e| format!("Failed to encode image: {:?}", e))?;
	// For BMP
	image::codecs::bmp::BmpEncoder::new(&mut bytes)
		.encode(&img, width, height, ColorType::Rgb8)
		.map_err(|e| format!("Failed to encode image: {:?}", e))?;
	web_sys::console::log_1(&"Background Factorio encoding complete, returning...".into());
	Ok(bytes)
}
