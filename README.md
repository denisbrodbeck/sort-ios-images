# sort ios images

[![Builds](https://github.com/denisbrodbeck/sort-ios-images/workflows/CI/badge.svg)](https://github.com/denisbrodbeck/sort-ios-images/actions?query=workflow%3ACI)
[![MIT Licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![Apache2 Licensed](https://img.shields.io/badge/license-Apache2-blue.svg)](./LICENSE-APACHE)

The cli app `sort-ios-images` takes an input directory and detects all images or videos which were taken with your own iPhone. All detected items get copied to a output directory. All other files get copied into a 'nomatch' directory. Folder structure of input directory will be preserved. Cross-plattform. Written in Rust.

## CLI usage

```bash
# Sort your images from `input` directory and copy them into `output` directory
sort-ios-images ~/ferris/images/ios-imported ~/ferris/images/ios-sorted

# Sort your images from `input` directory and move them into `output` directory -- really fast
sort-ios-images --move-files ~/ferris/images/ios-imported ~/ferris/images/ios-sorted
```

All available flags:

```txt
USAGE:
    sort-ios-images [FLAGS] <input> <output>

FLAGS:
    -h, --help          Prints help information
    -m, --move-files    Move sorted files instead of copying them
    -V, --version       Prints version information

ARGS:
    <input>     Input directory
    <output>    Output directory
```

## Output Layout

After running `sort-ios-images` your `output` directory will contain two directories: `match` and `nomatch`

- `match` contains all images or videos which were taken with your iPhone.
- `nomatch` contains everything else, like movies or images received via messengers or saved items

Given `~/ferris/images/ios-imported` as input directory:

```bash
/ferris/images/ios-imported/2019-02/IMG_3433.JPEG # you shot this
/ferris/images/ios-imported/2019-02/IMG_3434.JPEG # you shot this
/ferris/images/ios-imported/2019-02/IMG_3435.JPEG # you shot this
/ferris/images/ios-imported/2019-02/LOHG2296.JPG  # you got this
/ferris/images/ios-imported/2018-11/IMG_2155.JPEG # you shot this
/ferris/images/ios-imported/2018-11/IMG_2156.MOV  # you shot this
/ferris/images/ios-imported/2018-11/IMG_2157.JPEG # you shot this
/ferris/images/ios-imported/2018-11/MYRR9748.JPG  # you got this
```

`sort-ios-images` will produce this layout in `~/ferris/images/ios-sorted` output directory:

```bash
/ferris/images/ios-sorted/match/2019-02/IMG_3433.JPEG # you shot this
/ferris/images/ios-sorted/match/2019-02/IMG_3434.JPEG # you shot this
/ferris/images/ios-sorted/match/2019-02/IMG_3435.JPEG # you shot this
/ferris/images/ios-sorted/match/2018-11/IMG_2155.JPEG # you shot this
/ferris/images/ios-sorted/match/2018-11/IMG_2156.MOV  # you shot this
/ferris/images/ios-sorted/match/2018-11/IMG_2157.JPEG # you shot this

/ferris/images/ios-sorted/nomatch/2019-02/LOHG2296.JPG  # you got this
/ferris/images/ios-sorted/nomatch/2018-11/MYRR9748.JPG  # you got this
```

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
