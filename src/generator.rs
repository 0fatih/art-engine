use std::borrow::ToOwned;
use rand::Rng;
use std::collections::HashMap;

use image::DynamicImage;
use serde::{Deserialize, Serialize};
use serde_json;
use sha2::{Digest, Sha256};
use std::fs;
use std::fs::DirEntry;
use std::io::{Error, ErrorKind, ErrorKind::AlreadyExists};
use std::path::{Path, PathBuf};
use serial_test::serial;

use crate::images::merge;
use crate::utils::{get_asset_quantity, progress_bar};
use crate::utils::REQUIRED_PATHS;

const OUTPUT_PATH: &str = "./output/";
const IMAGES_PATH: &str = "./output/images/";
const METADATA_PATH: &str = "./output/metadata/";

#[derive(Serialize, Deserialize, Clone)]
struct Attribute {
    trait_type: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    name: String,
    description: String,
    image: String,
    attributes: Vec<Attribute>,
}

pub struct Collection {
    pub name: String,
    pub description: String,
    pub base_uri: String,
}

/// Generates given amount of random NFT images and metadata
pub fn generate_all(amount: usize, layers: Vec<&str>, collection: Collection) -> Result<(), Error> {
    if amount > possible_nfts() {
        return Err(Error::new(ErrorKind::InvalidInput, "insufficient amount of assets"));
    }

    let mut identifiers: HashMap<String, bool> = HashMap::new();

    // we'll use this for uniqueness

    let bar = progress_bar(amount);

    println!("Starting...");
    for token_id in 1..=amount {
        let mut attributes: Vec<Attribute> = get_random_attributes(&layers);
        let mut identifier = generate_identifier(&attributes);

        // we need unique NFTs
        while identifiers.get(&generate_identifier(&attributes)) != None {
            attributes = get_random_attributes(&layers);
            identifier = generate_identifier(&attributes);
        }

        identifiers.insert(identifier, true);
        generate_one(&collection, &layers, &attributes, token_id);
        bar.inc(1);
    }

    Ok(())
}

/// Returns all possible combination amount of NFTs
fn possible_nfts() -> usize {
    let assets_path = Path::new(REQUIRED_PATHS[0]).read_dir().unwrap();

    let mut amount: usize = 1;

    for asset_dir in assets_path {
        let asset_dir = asset_dir.unwrap().path();
        amount *= file_amount(&asset_dir);
    }

    amount
}

/// Returns file count in a directory
fn file_amount(path: &PathBuf) -> usize {
    let files = path.read_dir().unwrap();

    let mut amount = 0;

    for _ in files {
        amount += 1;
    }

    amount
}

/// Generates one NFT and metadata for the given information
fn generate_one(collection: &Collection, layers: &Vec<&str>, attributes: &Vec<Attribute>, token_id: usize) {
    let base_image = attribute_to_dynamic_image(&attributes[0]);
    let mut images: Vec<DynamicImage> = Vec::new();

    for i in 1..layers.len() {
        let image = attribute_to_dynamic_image(&attributes[i]);
        images.push(image);
    }

    let new_image_path =
        IMAGES_PATH.to_owned() + token_id.to_string().as_str() + ".png";
    let new_metadata_path =
        METADATA_PATH.to_owned() + token_id.to_string().as_str();

    // create image
    merge(base_image, &images).save(new_image_path).unwrap();

    // create metadata
    let metadata: Metadata = Metadata {
        name: collection.name.clone() + " #" + token_id.to_string().as_str(),
        description: collection.description.clone(),
        image: collection.base_uri.clone() + token_id.to_string().as_str(),
        attributes: attributes.into_iter().cloned().collect(),
    };

    let json = serde_json::to_string(&metadata).unwrap();

    fs::write(new_metadata_path, json).expect("unable to write file");
}

/// Produces an DynamicImage for attributes (assets)
fn attribute_to_dynamic_image(attr: &Attribute) -> DynamicImage {
    let path_str = REQUIRED_PATHS[0].to_owned() + attr.trait_type.as_str() + "/" + attr.value.as_str() + ".png";
    let path = Path::new(&path_str);
    path_to_dynamic_image(&path.to_path_buf())
}

/// Returns an identifier for the given attributes
fn generate_identifier(attributes: &Vec<Attribute>) -> String {
    let mut identifier = Sha256::new();

    for i in 0..attributes.len() {
        identifier.update(attributes[i].value.clone());
    }

    format!("{:X}", identifier.finalize())
}

/// Returns an NFT with full attributes
fn get_random_attributes(layers: &Vec<&str>) -> Vec<Attribute> {
    let mut attributes: Vec<Attribute> = Vec::new();

    for i in 0..layers.len() {
        let random_asset_path = get_random_asset_path(layers[i]).unwrap();
        attributes.push(Attribute {
            trait_type: layers[i].to_string(),
            value: path_to_asset_name(&random_asset_path),
        });
    }

    attributes
}

/// Finds an asset path randomly
fn get_random_asset_path(trait_type: &str) -> Result<PathBuf, String> {
    let asset_quantity = get_asset_quantity(trait_type).unwrap();

    let random_num = rand::thread_rng().gen_range(0..asset_quantity);

     let asset_path = &(REQUIRED_PATHS[0].to_owned() + trait_type);

    Ok(Path::new(asset_path).read_dir().unwrap().nth(random_num).unwrap().unwrap().path())
}

/// Produces a DynamicImage from given path
fn path_to_dynamic_image(path: &PathBuf) -> DynamicImage {
    image::open(path).unwrap()
}

/// Finds asset's name for given path
fn path_to_asset_name(path: &PathBuf) -> String {
    let asset_name = path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    asset_name
}

/// Returns random asset from the corresponding directory
// fn get_random_asset(asset_name: &str) -> (DynamicImage, String) {
//     let random_image_path = get_random_asset_path(asset_name).unwrap();
//
//     (
//         path_to_dynamic_image(&random_image_path.path()),
//         path_to_asset_name(&random_image_path),
//     )
// }

/// If the output directory exists, clears it
pub fn set_output_dir() -> Result<(), Error> {
    set_given_dir(OUTPUT_PATH)?;

    set_given_dir(IMAGES_PATH)?;
    set_given_dir(METADATA_PATH)?;

    Ok(())
}

/// If the given directory does not exists, creates one.
/// But, if it is exists, then it'll clear it.
fn set_given_dir(dir: &str) -> Result<(), Error> {
    match fs::create_dir(dir) {
        Err(err) => {
            if err.kind() == AlreadyExists {
                fs::remove_dir_all(dir)?;
                fs::create_dir(dir)?;
            } else {
                return Err(err);
            }
        }
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
        }
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
        }
        Ok(_) => {
            set_given_dir(root_path).unwrap();
        }
    }

    assert_eq!(Path::new(root_path).exists(), true);
    assert_eq!(Path::new(path).exists(), false);
    fs::remove_dir_all(root_path).unwrap();
}
