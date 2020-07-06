use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
#[structopt(
    name = "sort-ios-images",
    about = "sort-ios-images takes an input directory and sorts all images/videos which were taken with your own iPhone to an output directory. All other files get sorted into a 'nomatch' directory.",
    after_help = "Try:\n    sort-ios-images ~/ferris/images/ios ~/ferris/images/ios-sorted\n    sort-ios-images --move-files ~/ferris/images/ios ~/ferris/images/ios-sorted"
)]
pub struct Opt {
    /// Input directory
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,

    /// Output directory
    #[structopt(parse(from_os_str))]
    pub output: PathBuf,

    /// Move sorted files instead of copying them
    #[structopt(short, long)]
    pub move_files: bool,
}

pub fn new_opt() -> Opt {
    Opt::from_args()
}
