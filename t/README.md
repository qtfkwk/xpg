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
$ xpg -p wwwwwwww
!run:../target/release/xpg -p wwwwwwww
```

```text
$ xpg -p wwwwwwww -c 10
!run:../target/release/xpg -p wwwwwwww -c 10
```

```text
$ xpg -p wwddd
!run:../target/release/xpg -p wwddd
```

```text
$ xpg -p wwss
!run:../target/release/xpg -p wwss
```

```text
$ xpg -p wwds
!run:../target/release/xpg -p wwds
```

```text
$ xpg -p wwwwcc
!run:../target/release/xpg -p wwwwcc
```

```text
$ xpg -p wwwwCC
!run:../target/release/xpg -p wwwwCC
```

```text
$ xpg -p TTTT
!run:../target/release/xpg -p TTTT
```

```text
$ xpg -p WWWW
!run:../target/release/xpg -p WWWW
```

```text
$ xpg -p wwwwaaaaa
!run:../target/release/xpg -p wwwwaaaaa
```

```text
$ xpg -p aaaaaaaaaaaaaaaaaaaa
!run:../target/release/xpg -p aaaaaaaaaaaaaaaaaaaa
```

```text
$ xpg --keychain
!run:../target/release/xpg --keychain
```

```text
$ xpg --code-name
!run:../target/release/xpg --code-name
```

!inc:../CHANGELOG.md

# References

* Word list originates from Bart Busschots'
  [HSXKPasswd](https://www.bartbusschots.ie/s/publications/software/xkpasswd/)
  Perl module ([GitHub](https://github.com/bbusschots/hsxkpasswd),
  [CPAN: Crypt::HSXKPasswd](http://search.cpan.org/perldoc?Crypt%3A%3AHSXKPasswd)),
  specifically
  [lib/Crypt/HSXKPasswd/Dictionary/EN.pm@1d88564:38](https://github.com/bbusschots/hsxkpasswd/blob/1d88564d5bf74cf48025b372bcb635fc022962dd/lib/Crypt/HSXKPasswd/Dictionary/EN.pm#L38)

