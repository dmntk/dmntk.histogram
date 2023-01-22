**DMNTK** | Decision Model and Notation Toolkit

# Histogram generator

[![MIT licensed][mit-badge]][mit-url]
[![Apache 2.0 licensed][apache-badge]][apache-url]
[![Contributor Covenant][coc-badge]](https://github.com/dmntk/dmntk.rs/blob/main/CODE_OF_CONDUCT.md)

[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg
[mit-url]: https://github.com/dmntk/dmntk.rs/blob/main/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[apache-url]: https://github.com/dmntk/dmntk.rs/blob/main/LICENSE-APACHE
[coc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg

## Overview

Histogram generator reads an input file named **benchmarks.txt** placed in **data** directory.
Input file must contain results from Rust benchmarks, exactly like in the example shown below:

```
test compatibility::dmn_3_0056::_0019   ... bench:       5,255 ns/iter (+/- 64)
test compatibility::dmn_3_0056::_0020   ... bench:       1,646 ns/iter (+/- 28)
test compatibility::dmn_3_0056::_0021   ... bench:         642 ns/iter (+/- 11)
test compatibility::dmn_3_0056::_0022   ... bench:         661 ns/iter (+/- 11)
test compatibility::dmn_3_0056::_0023   ... bench:         661 ns/iter (+/- 13)
test compatibility::dmn_3_0056::_0024   ... bench:         666 ns/iter (+/- 12)
test compatibility::dmn_3_0056::_0025   ... bench:       1,107 ns/iter (+/- 21)
test compatibility::dmn_3_0056::_0026   ... bench:       1,122 ns/iter (+/- 38)
test compatibility::dmn_3_0056::_0027   ... bench:       1,125 ns/iter (+/- 21)
test compatibility::dmn_3_0056::_0028   ... bench:       1,131 ns/iter (+/- 17)
test compatibility::dmn_3_0057::_0001   ... bench:       1,460 ns/iter (+/- 33)
test compatibility::dmn_3_0057::_0002   ... bench:       3,397 ns/iter (+/- 35)
test compatibility::dmn_3_0057::_0003   ... bench:       1,302 ns/iter (+/- 18)
```

As a result, this application generates output files (placed in **data** directory): 

- **README.md** - performance summary.
- **benchmarks.hgrm** - histogram data file.
- **benchmarks.svg** - histogram chart in SVG format.

To generate PNG file from SVG file, the [ImageMagic](https://imagemagick.org/) can be used:

```
$ convert -size 2000x600 data/benchmarks.svg data/benchmarks.png
```

There is a [Task](https://taskfile.dev) command provided to automate generating the histogram in PNG format.

```
$ task gen
```

## License

Licensed under either of

- [MIT license](https://opensource.org/licenses/MIT) ([LICENSE-MIT](https://github.com/dmntk/dmntk.rs/blob/main/LICENSE-MIT)), or
- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](https://github.com/dmntk/dmntk.rs/blob/main/LICENSE-APACHE))

at your option.

## Contribution

All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.

