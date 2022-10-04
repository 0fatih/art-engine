use rand::Rng;

use std::fs;
use std::io::{Error, ErrorKind::AlreadyExists};
use std::path::Path;
use image::DynamicImage;
use indicatif::{ProgressBar, ProgressStyle};
use serde_json;
use serde::{Serialize, Deserialize};

use crate::utils::get_asset_quantity;
use crate::images::merge;
use crate::utils::REQUIRED_PATHS;

const OUTPUT_PATH: &str = "./output/";

#[derive(Serialize, Deserialize)]
struct Attribute {
    trait_type: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    name: String,
    description: String,
    image: String,
    attributes: Vec<Attribute>
}

pub struct Collection {
    pub name: String,
    pub description: String,
    pub base_uri: String,
}

/// Generates random NFT images and metadata
pub fn generate_all(amount: usize, layers: Vec<&str>, collection: Collection) -> Result<(), Error> {
    set_output_dir()?;

    let bar = ProgressBar::new(amount.try_into().unwrap());
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("##-"));

    println!("Starting...");
    for token_id in 1..=amount {
        let (base_image, base_property) = get_random_property(layers[0]);
        let mut images: Vec<DynamicImage> = Vec::new();
        let mut attributes: Vec<Attribute> = Vec::new();

        attributes.push(Attribute { trait_type: layers[0].to_string(), value: base_property });

        for i in 1..layers.len() {
            let (image, property) = get_random_property(layers[i]);
            images.push(image);
            attributes.push(Attribute { trait_type: layers[i].to_string(), value: property});
        }

        let new_image_path = OUTPUT_PATH.to_owned()  + "images/" + token_id.to_string().as_str() + ".png";
        let new_metadata_path = OUTPUT_PATH.to_owned() + "metadata/" + token_id.to_string().as_str();

        // create image
        merge(base_image, &images).save(new_image_path).unwrap();

        // create metadata
        let metadata: Metadata = Metadata {
            name: collection.name.clone() + " #" + token_id.to_string().as_str(),
            description: collection.description.clone(),
            image: collection.base_uri.clone() + token_id.to_string().as_str(),
            attributes
        };

        let json = serde_json::to_string(&metadata).unwrap();

        fs::write(new_metadata_path, json).expect("unable to write file");

        bar.inc(1);
    }

    Ok(())
}

/// Returns random asset from the corresponding directory
fn get_random_property(asset_name: &str) -> (DynamicImage, String) {
    let asset_quantity = get_asset_quantity(asset_name).unwrap();

    let random_num = rand::thread_rng().gen_range(0..asset_quantity);

    let asset_path: &str = &(REQUIRED_PATHS[0].to_owned() + asset_name);

    let random_image_path = Path::new(asset_path)
        .read_dir().unwrap().nth(random_num).unwrap().unwrap();

    let property_name = random_image_path.path().file_stem().unwrap().to_str().unwrap().to_string();

    (image::open(random_image_path.path()).unwrap(), property_name)
}

/// If the output directory exists, clears it
fn set_output_dir() -> Result<(), Error> {
    set_given_dir(OUTPUT_PATH)?;

    let images_path: &str = &(OUTPUT_PATH.to_owned() + "images");
    let metadata_path: &str = &(OUTPUT_PATH.to_owned() + "metadata");

    set_given_dir(images_path)?;
    set_given_dir(metadata_path)?;

    Ok(())
}

fn set_given_dir(dir: &str) -> Result<(), Error> {
    match fs::create_dir(dir)  {
        Err(err) => {
            if err.kind() == AlreadyExists {
                fs::remove_dir_all(dir)?;
                fs::create_dir(dir)?;
            } else {
                return Err(err);
            }
        },
        Ok(_) => {}
    }

    Ok(())
}

#[test]
#[serial]
fn test_set_given_dir_non_exist() {
    let path = "./test_output";
    if Path::new(path).exists() {
        fs::remove_dir_all(path).unwrap();
    }

    set_given_dir(path).unwrap();

    assert_eq!(Path::new(path).exists(), true);

    fs::remove_dir_all(path).unwrap();
}

#[test]
#[serial]
fn test_set_given_dir_exist() {
    let path = "./test_output";

    match fs::create_dir(path) {
        Err(err) => {
            if err.kind() != AlreadyExists {
                panic!("An error occurred while trying to create path");
            }
        },
        Ok(_) => {
            set_given_dir(path).unwrap();
        }
    }

    assert_eq!(Path::new(path).exists(), true);
    fs::remove_dir_all(path).unwrap();
}

#[test]
#[serial]
fn test_set_output_exist_with_sub_directories() {
    let path = "./test_output/testing";
    let root_path = "./test_output";

    match fs::create_dir_all(path) {
        Err(err) => {
            if err.kind() != AlreadyExists {
                panic!("An error occurred while trying to create path");
            }
        },
        Ok(_) => {
            set_given_dir(root_path).unwrap();
        }
    }

    assert_eq!(Path::new(root_path).exists(), true);
    assert_eq!(Path::new(path).exists(), false);
    fs::remove_dir_all(root_path).unwrap();
}