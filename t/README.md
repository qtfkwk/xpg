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

*See also the [Configuration] section below.*

[Configuration]: #configuration

### Examples

```text
$ xpg
!run:../target/release/xpg
```

Generate 10 passwords:

```text
$ xpg -c 10
!run:../target/release/xpg -c 10
```

Generate passwords forever (or until Ctrl+C / SIGINT):

```text
$ xpg -c 0
!run:../target/release/xpg -c 10
^C
```

Generate a password with 8 words:

```text
$ xpg wwwwwwww
!run:../target/release/xpg wwwwwwww
```

Generate 10 passwords with 8 words:

```text
$ xpg wwwwwwww -c 10
!run:../target/release/xpg wwwwwwww -c 10
```

Generate a password with 2 words followed by 3 digits:

```text
$ xpg wwddd
!run:../target/release/xpg wwddd
```

Generate a password with 2 words followed by 2 symbols:

```text
$ xpg wwss
!run:../target/release/xpg wwss
```

Generate a password with 2 words followed by a digit and a symbol:

```text
$ xpg wwds
!run:../target/release/xpg wwds
```

Generate a password with 4 words followed by 2 lowercase letters:

```text
$ xpg wwwwcc
!run:../target/release/xpg wwwwcc
```

Generate a password with 4 words followed by 2 uppercase letters:

```text
$ xpg wwwwCC
!run:../target/release/xpg wwwwCC
```

Generate a password with 4 title case words:

```text
$ xpg TTTT
!run:../target/release/xpg TTTT
```

Generate a password with 4 uppercase words:

```text
$ xpg WWWW
!run:../target/release/xpg WWWW
```

Generate a password with 4 words followed by 5 "any" characters (lowercase letter, uppercase letter,
digit, or symbol)

```text
$ xpg wwwwaaaaa
!run:../target/release/xpg wwwwaaaaa
```

Generate a password with 3 words and at least 15 characters:

```text
$ xpg www -m 16
!run:../target/release/xpg www -m 15
```

Generate a password with 3 words and no more than 20 characters:

```text
$ xpg www -M 20
!run:../target/release/xpg www -M 20
```

Generate a password with 4 words and between 20 and 25 characters (increase attempts to help it
succeed):

```text
$ xpg www -m 20 -M 25 -a 1000
!run:../target/release/xpg www -m 20 -M 25 -a 1000
```

Generate a password with 2 words, 3 digits, 1 symbol, and exactly 16 characters (increase attempts
to help it succeed):

```text
$ xpg wwddds -l 16 -a 1000
!run:../target/release/xpg wwddds -l 16 -a 1000
```

Generate a password with a `1`, 2 title case words, and `!`:

```text
$ xpg '1TT!'
!run:../target/release/xpg '1TT!'
```

Generate a password with 1 symbol, 2 adjectives, 1 noun, and 2 digits:

```text
$ xpg 's{adj}{adj}{n}dd'
!run:../target/release/xpg 's{adj}{adj}{n}dd'
```

Generate a keychain-style password:

```text
$ xpg keychain
!run:../target/release/xpg keychain
```

Generate a code name:

```text
$ xpg codename
!run:../target/release/xpg codename
```

Generate a code name series:

```text
$ xpg codename-series
!run:../target/release/xpg codename-series
```

Generate a code name series with 20 code names:

```text
$ xpg codename-series -c 20
!run:../target/release/xpg codename-series -c 20
```

Generate a password with 20 "any" characters:

```text
$ xpg aaaaaaaaaaaaaaaaaaaa
!run:../target/release/xpg aaaaaaaaaaaaaaaaaaaa
```

Same as the previous example, but shorter:

```text
$ xpg a -m 20
!run:../target/release/xpg a -m 20
```

# Configuration

See [`config.json`] for the default configuration compiled into the binary
executable.

[`config.json`]: config.json

If another file is placed at any of the following locations or the path provided
via the `-C` option at *runtime*, it will be used instead.

```text
~/.config/xpg/config.json
/etc/xpg/config.json
```

## `WordKind`s

* Adjective / `{adj}`
    * Color\* / `{color}`
    * Nationality / `{nat}`
* Adverb / `{adv}`
* Conjunction / `{conj}`
* Interjection / `{i}`
* Noun / `{n}`
    * PluralNoun / `{n.pl}`
    * Pronoun / `{n.pro}`
    * SingularNoun / `{n.s}`
        * Astronomy / `{ast}`
            * Moon / `{moon}`
            * Planet / `{planet}`
        * Color\* / `{color}`
        * Day / `{day}`
        * Month / `{mon}`
        * ProperNoun / `{n.prop}`
            * Element / `{el}`
            * Name / `{name}`
                * FemaleName / `{fname}`
                * MaleName / `{mname}`
            * Place / `{place}`
                * City / `{city}`
                * Continent / `{cont}`
                * Country / `{country}`
                * UsState / `{us-state}`
            * Mythology / `{myth}`
                * GreekDeity / `{greekdeity}`
                * RomanDeity / `{romandeity}`
* Preposition / `{prep}`
* Verb / `{v}`
    * AuxiliaryVerb / `{v.aux}`
    * IntransitiveVerb / `{v.int}`
    * TransitiveVerb / `{v.tr}`
    * VerbPast / `{v.past}`

**Notes**

1. `WordKind` / `{sub}`

2. It is only necessary to set the most descriptive `WordKind`(s) since they are
   automatically included in the larger word kinds.

3. The `characters` object value must contain the `C`, `c`, and `d` keys with
   non-empty values.
   Additional keys/values can be added and used.

!inc:../CHANGELOG.md

# References

* Word list originated from Bart Busschots'
  [HSXKPasswd](https://www.bartbusschots.ie/s/publications/software/xkpasswd/)
  Perl module ([GitHub](https://github.com/bbusschots/hsxkpasswd),
  [CPAN: Crypt::HSXKPasswd](http://search.cpan.org/perldoc?Crypt%3A%3AHSXKPasswd)),
  specifically
  [lib/Crypt/HSXKPasswd/Dictionary/EN.pm@1d88564:38](https://github.com/bbusschots/hsxkpasswd/blob/1d88564d5bf74cf48025b372bcb635fc022962dd/lib/Crypt/HSXKPasswd/Dictionary/EN.pm#L38)

