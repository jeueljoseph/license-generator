# license-generator [![Build Status](https://travis-ci.org/azu/license-generator.svg?branch=master)](https://travis-ci.org/azu/license-generator)

Command line tool for generating license files.

## Installation

To install the original https://github.com/azu/license-generator as hosted on https://crates.io/:

    cargo install license-generator

To install this fork, clone this repo and build using

    cargo build --release
    
and move license.exe into your .cargo\bin folder.

## Usage

    $ license -a <author> LICENSE_TYPE [SECOND_LICENSE_TYPE] [...NTH LICENSE TYPE]

    LICENSE_TYPE:
    - AGPL
    - Apache
    - CC-BY
    - CC-BY-NC
    - CC-BY-NC-SA
    - CC-BY-SA
    - CC0
    - GPL
    - LGPL
    - MIT
    - MPL
    - Unlicense

    Options:
      -a, --author Author
      -p, --project Project name as required by some licenses
      -y, --year License year 
      
    Examples:
    This generates `LICENSE_MIT` and `LICENSE_APACHE` in the current directory
       $ license -a Author_Name MIT Apache
       
    This generates the GPL license with filename `LICENSE` in the current directory
       $ license -a Author_Name GPL
    

## Supported Licenses

The following licenses are supported:

- [AGPL-3.0](http://www.gnu.org/licenses/agpl-3.0)
- [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [CC-BY-4.0](https://creativecommons.org/licenses/by/4.0/)
- [CC-BY-NC-4.0](https://creativecommons.org/licenses/by-nc/4.0/)
- [CC-BY-NC-SA-4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/)
- [CC-BY-SA-4.0](https://creativecommons.org/licenses/by-sa/4.0/)
- [CC0-1.0](http://creativecommons.org/publicdomain/zero/1.0/)
- [GPL-3.0](http://www.gnu.org/licenses/gpl-3.0)
- [LGPL-3.0](http://www.gnu.org/licenses/lgpl-3.0)
- [MIT](https://opensource.org/licenses/MIT)
- [MPL-2.0](https://www.mozilla.org/en-US/MPL/2.0/)
- [Unlicense](http://unlicense.org/)

## Tests

    cargo test

## Releases

Use [cargo-release](https://github.com/sunng87/cargo-release).

    cargo release --no-dev-version {patch,minor,major}

## Contributing

1. Fork it!
2. Create your feature branch: `git checkout -b my-new-feature`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin my-new-feature`
5. Submit a pull request :D

## License

MIT © azu
