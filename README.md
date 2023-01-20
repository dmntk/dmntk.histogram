# dmntk.histogram
DMNTK | Histogram generator

[![MIT licensed][mit-badge]][mit-url]
[![Apache 2.0 licensed][apache-badge]][apache-url]
[![Contributor Covenant][coc-badge]](https://github.com/dmntk/dmntk.rs/blob/main/CODE_OF_CONDUCT.md)

[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg
[mit-url]: https://github.com/dmntk/dmntk.rs/blob/main/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[apache-url]: https://github.com/dmntk/dmntk.rs/blob/main/LICENSE-APACHE
[coc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg

## Overview

This application reads an input file named **benchmarks.txt** containing Rust 
benchmark results and generates histogram data and image.

Example content of **benchmarks.txt** file:

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

Printed output:

```
=============================================================================
  Total count:         13 [µs]
          Min:          0 [µs]
          Max:          5 [µs]
         Mean:          1 [µs]
      Std Dev:          1 [µs]
=============================================================================
  30.8'th percentile of data is <=     0 [µs]  with     4 sample(s)
  84.6'th percentile of data is <=     1 [µs]  with     7 sample(s)
  92.3'th percentile of data is <=     3 [µs]  with     1 sample(s)
 100.0'th percentile of data is <=     5 [µs]  with     1 sample(s)
=============================================================================


     Value   Percentile    TotalCount    1/(1-Percentile)

         0     0.307692             4          1.44
         1     0.846154            11          6.50
         3     0.923077            12         13.00
         5     1.000000            13           inf

=============================================================================

Percentile   Microseconds
    30.77              0
    84.62              1
    92.31              3
   100.00              5

=============================================================================
```

The histogram is generated in SVG file named **benchmarks.svg**.

To generate PNG file ImageMagic may be used:

```
$ convert -size 2000x600 benchmarks.svg benchmarks.png
```

There is `task` command provided that automates generating histogram.

## License

Licensed under either of

- [MIT license](https://opensource.org/licenses/MIT) ([LICENSE-MIT](https://github.com/dmntk/dmntk.rs/blob/main/LICENSE-MIT)), or
- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](https://github.com/dmntk/dmntk.rs/blob/main/LICENSE-APACHE))

at your option.

## Contribution

All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.

