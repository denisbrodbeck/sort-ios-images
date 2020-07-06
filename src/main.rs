use anyhow::Result;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

mod cli;
use crate::cli::{new_opt, Opt};

fn main() -> Result<()> {
    // parse args
    let opt = new_opt();

    // create output directories
    fs::create_dir_all(opt.output.join("match"))?;
    fs::create_dir_all(opt.output.join("nomatch"))?;

    // walk the walk
    walk_dir(opt)?;
    Ok(())
}

fn walk_dir(opt: Opt) -> Result<()> {
    let mut counter_match: u64 = 0;
    let mut counter_nomatch: u64 = 0;

    for entry in WalkDir::new(opt.input.as_path())
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir() {
            continue;
        }
        let path = entry.path();
        match matches(path) {
            Ok(found) => {
                if found || is_movie(&entry) {
                    if opt.move_files {
                        move_file(opt.input.as_path(), path, opt.output.join("match"))?;
                    } else {
                        copy_file(opt.input.as_path(), path, opt.output.join("match"))?;
                    }

                    counter_match += 1;
                } else {
                    if opt.move_files {
                        move_file(opt.input.as_path(), path, opt.output.join("nomatch"))?;
                    } else {
                        copy_file(opt.input.as_path(), path, opt.output.join("nomatch"))?;
                    }

                    counter_nomatch += 1;
                }
            }
            Err(err) => return Err(err),
        }
    }

    println!("Files match: {}", counter_match);
    println!("Files no match: {}", counter_nomatch);
    Ok(())
}

// very basic check
// but most shared movies habe GUID names or the mp4 file ending
fn is_movie(entry: &DirEntry) -> bool {
    return entry
        .file_name()
        .to_string_lossy()
        .to_lowercase()
        .starts_with("img_")
        && entry
            .path()
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase()
            == "mov";
}

fn matches<P: AsRef<Path>>(path: P) -> Result<bool> {
    let file = File::open(path)?;

    let exif = match exif::Reader::new().read_from_container(&mut BufReader::new(&file)) {
        Ok(exif) => exif,
        Err(exif::Error::NotFound(_)) => {
            return Ok(false);
        }
        Err(exif::Error::InvalidFormat(_)) => {
            return Ok(false);
        }
        Err(err) => return Err(err.into()),
    };

    match image_has_fields(exif) {
        Some(has) => Ok(has),
        None => Ok(false),
    }
}

// identify my own images by checking for exif label Make=Apple and wether the image has GPS data attached
// all other images are usually received by WhatsApp & Co and are not images I took myself with my phone
fn image_has_fields(exif: exif::Exif) -> Option<bool> {
    if exif
        .get_field(exif::Tag::Make, exif::In::PRIMARY)?
        .display_value()
        .to_string()
        .contains("Apple")
        && exif
            .get_field(exif::Tag::GPSDateStamp, exif::In::PRIMARY)
            .is_some()
    {
        return Some(true);
    }
    Some(false)
}

// base:    ~/ferris/images/ios
// from:    ~/ferris/images/ios/2019-02/IMG_3433.JPEG
// dst_dir: ~/ferris/images/ios-sorted
// to:      ~/ferris/images/ios-sorted/2019-02/IMG_3433.JPEG
fn copy_file<P: AsRef<Path>, Q: AsRef<Path>, R: AsRef<Path>>(
    base: P,
    from: Q,
    dst_dir: R,
) -> Result<()> {
    let to = dst_dir
        .as_ref()
        .join(from.as_ref().strip_prefix(base.as_ref())?);
    fs::create_dir_all(to.parent().unwrap())?;
    match std::fs::copy(from, to) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

// base:    ~/ferris/images/ios
// from:    ~/ferris/images/ios/2019-02/IMG_3433.JPEG
// dst_dir: ~/ferris/images/ios-sorted
// to:      ~/ferris/images/ios-sorted/2019-02/IMG_3433.JPEG
fn move_file<P: AsRef<Path>, Q: AsRef<Path>, R: AsRef<Path>>(
    base: P,
    from: Q,
    dst_dir: R,
) -> Result<()> {
    let to = dst_dir
        .as_ref()
        .join(from.as_ref().strip_prefix(base.as_ref())?);
    fs::create_dir_all(to.parent().unwrap())?;
    match std::fs::rename(from, to) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}
