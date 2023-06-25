use std::{collections::HashMap, path::PathBuf};

use itertools::Itertools;
use osu_db::{
    collection::{Collection, CollectionList},
    listing::Listing,
    score::ScoreList,
};

const MAIN_DB_NAME: &'static str = "osu!.db";
const SCORES_DB_NAME: &'static str = "scores.db";

const UNPLAYED_COLLECTION_DB_NAME: &'static str = "collection.unplayed.db";
const UNPLAYED_COLLECTION_NAME: &'static str = "Unplayed Beatmaps";

fn main() -> anyhow::Result<()> {
    let osu_folder_path = PathBuf::from(
        std::env::args()
            .nth(1)
            .expect("No osu! folder path provided"),
    );

    if !osu_folder_path.is_dir() {
        panic!("Provided folder path doesn't exist: {:?}", &osu_folder_path);
    }

    let main_db_path = osu_folder_path.join(MAIN_DB_NAME);
    let scores_db_path = osu_folder_path.join(SCORES_DB_NAME);

    if !main_db_path.is_file() {
        panic!("No {} found in {:?}", MAIN_DB_NAME, &osu_folder_path);
    }

    if !scores_db_path.is_file() {
        panic!("No {} found in {:?}", SCORES_DB_NAME, &osu_folder_path);
    }

    println!("Reading {} from {:?}...", MAIN_DB_NAME, &osu_folder_path);
    let main_db = Listing::from_file(main_db_path)?;
    let main_db_version = main_db.version;
    println!("Loaded {} (version {})", MAIN_DB_NAME, main_db_version);

    println!("Reading {} from {:?}...", SCORES_DB_NAME, &osu_folder_path);
    let scores_db = ScoreList::from_file(scores_db_path)?;
    let scores_db_version = scores_db.version;
    println!("Loaded {} (version {})", SCORES_DB_NAME, scores_db_version);

    // Make a mapping from beatmap MD5 hash to beatmapset
    let mut beatmapset_mapping: HashMap<String, i32> = HashMap::new();

    for beatmap in main_db.beatmaps.iter().filter(|&b| b.beatmapset_id != -1) {
        beatmapset_mapping.insert(
            beatmap.hash.clone().unwrap_or_default(),
            beatmap.beatmapset_id,
        );
    }

    let total_beatmaps = beatmapset_mapping.len();
    let total_beatmapsets = beatmapset_mapping.values().unique().count();

    println!(
        "Found {} beatmapsets ({} beatmaps)",
        total_beatmapsets, total_beatmaps
    );

    // Go through each beatmap MD5 hash from the scores DB and remove them from the mapping
    for beatmap_scores in scores_db.beatmaps.iter() {
        if let Some(beatmap_hash) = beatmap_scores.hash.as_ref() {
            if let Some(&beatmapset_id) = beatmapset_mapping.get(beatmap_hash) {
                beatmapset_mapping.retain(|_, id| *id != beatmapset_id);
            }
        }
    }

    let total_unplayed_beatmaps = beatmapset_mapping.len();
    let total_unplayed_beatmapsets = beatmapset_mapping.values().unique().count();

    println!(
        "Found {} unplayed beatmapsets ({} beatmaps)",
        total_unplayed_beatmapsets, total_unplayed_beatmaps
    );

    // Create a new collections DB with the unplayed beatmap hashes
    let unplayed_collection_db = CollectionList {
        version: main_db_version,
        collections: vec![Collection {
            name: Some(UNPLAYED_COLLECTION_NAME.to_string()),
            beatmap_hashes: beatmapset_mapping.keys().map(|k| Some(k.clone())).collect(),
        }],
    };

    let unplayed_collection_db_path = osu_folder_path.join(UNPLAYED_COLLECTION_DB_NAME);

    println!(
        "Creating {} in {:?}...",
        UNPLAYED_COLLECTION_DB_NAME, osu_folder_path
    );
    unplayed_collection_db.to_file(unplayed_collection_db_path)?;
    println!(
        "Created {} in {:?}",
        UNPLAYED_COLLECTION_DB_NAME, osu_folder_path
    );

    Ok(())
}
