# osu-unplayed-beatmaps

Small app that creates a collection of unplayed beatmaps that can be viewed in osu!. Useful for clearing out old beatmaps.

## Usage

- Install Rust stable (tested with 1.70)
- Run `cargo run -- <path to osu! folder>`

This will read the `osu!.db` and `scores.db` to search for beatmapsets that have no scores, and create a `collection.unplayed.db` in your osu! folder. Rename this to `collection.db` to view the collection of unplayed beatmaps.
