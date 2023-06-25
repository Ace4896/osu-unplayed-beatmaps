use std::path::PathBuf;

use osu_db::{listing::Listing, score::ScoreList};

const MAIN_DB_NAME: &'static str = "osu!.db";
const SCORES_DB_NAME: &'static str = "scores.db";

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

    // First 10 Beatmaps
    for beatmap in main_db
        .beatmaps
        .iter()
        .filter(|&b| b.beatmapset_id == -1)
        .take(10)
    {
        println!(
            "{} - {} [{}] {}",
            beatmap.beatmapset_id,
            beatmap.title_ascii.as_deref().unwrap_or(""),
            beatmap.difficulty_name.as_deref().unwrap_or(""),
            beatmap.last_online_check
        );
    }

    Ok(())
}
