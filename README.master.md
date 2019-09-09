# About

xkcd-style password generator

# Features

* Balanced code verbosity, comments, documentation, tests, benchmarks, external
  dependencies
* CLI utility
* Rust library, function, macro

# Usage

## CLI

~~~
$$ xpg -h
~~~

~~~
$$ xpg -V
~~~

### Examples

~~~
$$ xpg
~~~

~~~
$$ xpg -c 10
~~~

~~~
$$ xpg -w 8
~~~

~~~
$$ xpg -w 8 -c 10
~~~

~~~
$$ xpg --analyze
~~~

~~~
$$ xpg --analyze -w 8
~~~

# Changelog

* 0.2.0 (2019-09-09)
    * expose `xpg!` macro
    * improve documentation
* 0.1.2 (2019-09-08)
    * minor fixes
* 0.1.1 (2019-09-08)
    * minor fixes
* 0.1.0 (2019-09-08)
    * store wordlist as constant and operate on it without copying
    * select N words without repetition via `choose_multiple`
    * tests
    * benchmark various implementations
    * `xpg` function
    * `xpg!` macro to enable optional word count argument
    * command line interface via clap

# References

* [xkcd #936: Password Strength](https://xkcd.com/936/)

  ![](fig/password_strength.png)

* Word list comes from Bart Busschots'
  [HSXKPasswd](https://www.bartbusschots.ie/s/publications/software/xkpasswd/)
  Perl module ([GitHub](https://github.com/bbusschots/hsxkpasswd),
  [CPAN: Crypt::HSXKPasswd](http://search.cpan.org/perldoc?Crypt%3A%3AHSXKPasswd)),
  specifically
  [lib/Crypt/HSXKPasswd/Dictionary/EN.pm@1d88564:38](https://github.com/bbusschots/hsxkpasswd/blob/1d88564d5bf74cf48025b372bcb635fc022962dd/lib/Crypt/HSXKPasswd/Dictionary/EN.pm#L38)

# Legal

```
$ cat LICENSE.txt
```

[MIT License](https://opensource.org/licenses/MIT)

# Tests

~~~
$$ cargo test
~~~

# Benchmarks

Benchmark | time
---|---
1 | 54.084 us 54.232 us 54.383 us
2 | 191.25 ns 191.64 ns 192.26 ns
3 | 190.73 ns 190.86 ns 191.03 ns

## Benchmark 1

[src/lib.rs:1403](src/lib.rs#L1403)

~~~rust
$ head -1410 src/lib.rs |tail -8 |sed 's/^    //'
~~~

[benches/xpg.rs.1.log](benches/xpg.rs.1.log)

~~~
$ cat benches/xpg.rs.1.log
~~~

## Benchmark 2

[src/lib.rs:1413](src/lib.rs#L1413)

~~~rust
$ head -1415 src/lib.rs |tail -3 |sed 's/^    //'
~~~

[benches/xpg.rs.2.log](benches/xpg.rs.2.log)

~~~
$ cat benches/xpg.rs.2.log
~~~

## Benchmark 3

[src/lib.rs:1419](src/lib.rs#L1419)

~~~rust
$ head -1420 src/lib.rs |tail -2 |sed 's/^    //'
~~~

[benches/xpg.rs.3.log](benches/xpg.rs.3.log)

~~~
$ cat benches/xpg.rs.3.log
~~~

