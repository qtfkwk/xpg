# About

xkcd-style password generator

![](fig/password_strength.png)

[xkcd #936: Password Strength](https://xkcd.com/936/)

# Usage

```text
$ xpg -h
!run:../target/release/xpg -h
```

```text
$ xpg -V
!run:../target/release/xpg -V
```

### Examples

```text
$ xpg
!run:../target/release/xpg
```

```text
$ xpg -c 10
!run:../target/release/xpg -c 10
```

```text
$ xpg -w 8
!run:../target/release/xpg -w 8
```

```text
$ xpg -w 8 -c 10
!run:../target/release/xpg -w 8 -c 10
```

```text
$ xpg -w 2 -d 3
!run:../target/release/xpg -w 2 -d 3
```

```text
$ xpg -w 2 -s 2
!run:../target/release/xpg -w 2 -s 2
```

```text
$ xpg -w 2 -d 1 -s 1
!run:../target/release/xpg -w 2 -d 1 -s 1
```

```text
$ xpg -l 2
!run:../target/release/xpg -l 2
```

```text
$ xpg -u 2
!run:../target/release/xpg -u 2
```

```text
$ xpg -a 5
!run:../target/release/xpg -a 5
```

```text
$ xpg -w 0 -a 20
!run:../target/release/xpg -w 0 -a 20
```

```text
$ xpg --keychain
!run:../target/release/xpg --keychain
```

```text
$ xpg --code-name
!run:../target/release/xpg --code-name
```

```text
$ xpg --analyze
!run:../target/release/xpg --analyze
```

```text
$ xpg --analyze -w 8
!run:../target/release/xpg --analyze -w 8
```

!inc:../CHANGELOG.md

# References

* Word list comes from Bart Busschots'
  [HSXKPasswd](https://www.bartbusschots.ie/s/publications/software/xkpasswd/)
  Perl module ([GitHub](https://github.com/bbusschots/hsxkpasswd),
  [CPAN: Crypt::HSXKPasswd](http://search.cpan.org/perldoc?Crypt%3A%3AHSXKPasswd)),
  specifically
  [lib/Crypt/HSXKPasswd/Dictionary/EN.pm@1d88564:38](https://github.com/bbusschots/hsxkpasswd/blob/1d88564d5bf74cf48025b372bcb635fc022962dd/lib/Crypt/HSXKPasswd/Dictionary/EN.pm#L38)

