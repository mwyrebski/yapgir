yapgir
======

YAPGIR is Another Password Generator (In Rust).

Building
--------

The program needs [Rust](https://www.rust-lang.org/) and uses `cargo` to build.

To build it locally, execute:

    cargo build

Installing
----------

Using `cargo`, the application can be installed:

    cargo install --git https://github.com/mwyrebski/yapgir.git

It should then usable from your PATH as `yapgir`.

Running
-------

To show usage help, type `yapgir -h`:

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

### Example

In order to generate 3 passwords consisting of 11 lowercase, uppercase and number chars:

    $ yapgir -l 11 -c 3 -t nul
    onMfurfIYAu
    pOJ05h3art6
    ZJnqSk9J9lb

License
=======

[MIT](https://opensource.org/licenses/MIT)
