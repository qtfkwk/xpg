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
$$ xpg -w 2 -d 3
~~~

~~~
$$ xpg -w 2 -s 2
~~~

~~~
$$ xpg -w 2 -d 1 -s 1
~~~

~~~
$$ xpg -l 2
~~~

~~~
$$ xpg -u 2
~~~

~~~
$$ xpg -a 5
~~~

~~~
$$ xpg -w 0 -a 20
~~~

~~~
$$ xpg --analyze
~~~

~~~
$$ xpg --analyze -w 8
~~~

# Changelog

* 0.4.0 (2022-10-02)
    * add --lowercase, --uppercase, and --any options
    * enable --words option to be zero if --any option is greater than zero
    * enable infinite passwords via `-c 0`
* 0.3.1 (2022-10-01)
    * fix readme
* 0.3.0 (2022-10-01)
    * update dependencies
    * add digits and symbols options
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

