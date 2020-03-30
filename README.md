yapgir
======

YAPGIR is Another Password Generator (In Rust).

Building
--------

The program needs [Rust](https://www.rust-lang.org/) and uses cargo to build.

To build it, execute:

    cargo build

Running
-------

You can run it using cargo. Options can be passed after double dash, e.g.:

    cargo run -- -l 11 -c 7 -t nul

Pass `-h` to show help:

```
Usage:
        yapgir

Options:
        -v            display information about the program and exit
        -l <length>   length of the generated passwords (default: 10)
        -c <length>   number of passwords to generate (default: 1)
        -t [nul]      type of the passwords, any of:
                      l - lowercase
                      u - uppercase
                      n - number
                      (default: lun - all options)
```

License
=======

[MIT](https://opensource.org/licenses/MIT)
