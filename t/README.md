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

#### Generate 10 passwords

```text
$ xpg -c 10
!run:../target/release/xpg -c 10
```

#### Generate passwords forever

*or until Ctrl+C / SIGINT*

```text
$ xpg -c 0
!run:../target/release/xpg -c 10
^C
```

#### Generate a password with 8 words

```text
$ xpg wwwwwwww
!run:../target/release/xpg wwwwwwww
```

#### Generate 10 passwords with 8 words

```text
$ xpg wwwwwwww -c 10
!run:../target/release/xpg wwwwwwww -c 10
```

#### Generate a password with 2 words followed by 3 digits

```text
$ xpg wwddd
!run:../target/release/xpg wwddd
```

#### Generate a password with 2 words followed by 2 symbols

```text
$ xpg wwss
!run:../target/release/xpg wwss
```

#### Generate a password with 2 words followed by a digit and a symbol

```text
$ xpg wwds
!run:../target/release/xpg wwds
```

#### Generate a password with 4 words followed by 2 lowercase letters

```text
$ xpg wwwwcc
!run:../target/release/xpg wwwwcc
```

#### Generate a password with 4 words followed by 2 uppercase letters

```text
$ xpg wwwwCC
!run:../target/release/xpg wwwwCC
```

#### Generate a password with 4 title case words

```text
$ xpg TTTT
!run:../target/release/xpg TTTT
```

#### Generate a password with 4 uppercase words

```text
$ xpg WWWW
!run:../target/release/xpg WWWW
```

#### Generate a password with 4 words followed by 5 *any* characters

*An "any" character can be a lowercase letter, uppercase letter, digit, or
symbol.*

```text
$ xpg wwwwaaaaa
!run:../target/release/xpg wwwwaaaaa
```

#### Generate a password with 3 words and at least 15 characters

```text
$ xpg www -m 15
!run:../target/release/xpg www -m 15
```

#### Generate a password with 3 words and no more than 20 characters

```text
$ xpg www -M 20
!run:../target/release/xpg www -M 20
```

#### Generate a password with 4 words and between 20 and 25 characters

```text
$ xpg www -m 20 -M 25 -a 1000
!run:../target/release/xpg www -m 20 -M 25 -a 1000
```

*See also [Usage] note 2.*

[Usage]: #usage

#### Generate a password with 2 words, 3 digits, 1 symbol, and exactly 16 characters

```text
$ xpg wwddds -l 16 -a 1000
!run:../target/release/xpg wwddds -l 16 -a 1000
```

*See also [Usage] note 2.*

#### Generate a password with a `1`, 2 title case words, and `!`

```text
$ xpg '1TT!'
!run:../target/release/xpg '1TT!'
```

#### Generate a password with 1 symbol, 2 adjectives, 1 noun, and 2 digits

```text
$ xpg 's{adj}{adj}{n}dd'
!run:../target/release/xpg 's{adj}{adj}{n}dd'
```

#### Generate a password with 2 words and 1 extended word

```text
$ xpg 'ww{ext}'
!run:../target/release/xpg 'ww{ext}'
```

#### Generate a password with 2 words and 1 title case extended word

```text
$ xpg 'ww{T:ext}'
!run:../target/release/xpg 'ww{T:ext}'
```

#### Generate a password with 2 words and 1 uppercase extended word

```text
$ xpg 'ww{W:ext}'
!run:../target/release/xpg 'ww{W:ext}'
```

#### Generate 10 passwords with 4 title case words and extended words merged

```text
$ xpg -ec 10 TTTT
!run:../target/release/xpg -ec 10 TTTT
```

#### Generate a keychain-style password

```text
$ xpg keychain
!run:../target/release/xpg keychain
```

#### Generate a code name

```text
$ xpg codename
!run:../target/release/xpg codename
```

#### Generate a code name series

```text
$ xpg codename-series
!run:../target/release/xpg codename-series
```

#### Generate a code name series with 20 code names

```text
$ xpg codename-series -c 20
!run:../target/release/xpg codename-series -c 20
```

#### Generate a haiku

```text
$ xpg haiku
!run:../target/release/xpg haiku
```

#### Generate a haiku with syllable pattern

```text
$ xpg haiku+
!run:../target/release/xpg haiku+
```

#### Generate a condensed haiku with syllable pattern

```text
$ xpg haiku-
!run:../target/release/xpg haiku-
```

#### Generate a condensed haiku

```text
$ xpg haiku--
!run:../target/release/xpg haiku--
```

#### Generate a haiku with newlines

```text
$ xpg haiku++
!run:../target/release/xpg haiku++
```

#### Generate a haiku with syllable pattern and newlines

```text
$ xpg haiku+++
!run:../target/release/xpg haiku+++
```

#### Generate a haiku with extended words

```text
$ xpg -e haiku
!run:../target/release/xpg -e haiku
```

#### Generate a password with 20 *any* characters

```text
$ xpg aaaaaaaaaaaaaaaaaaaa
!run:../target/release/xpg aaaaaaaaaaaaaaaaaaaa
```

#### Same as the previous example, but shorter

*This works because a character-only pattern is padded to the minimum length
with the last character.*

```text
$ xpg a -m 20
!run:../target/release/xpg a -m 20
```

#### Generate a password with 5 lowercase, 1 uppercase, 1 digit, and 1 symbol

```text
$ xpg cccccCds
!run:../target/release/xpg cccccCds
```

#### Generate a password with 5 lowercase, 1 uppercase, 1 digit, and 1 symbol and shuffle the characters

```text
$ xpg -s cccccCds
!run:../target/release/xpg -s cccccCds
```

#### Same as the previous example, but shorter

*This works because a character-only pattern is padded to the minimum length
with the last character.*

```text
$ xpg -sm 8 Cdsc
!run:../target/release/xpg -sm 8 Cdsc
```

#### Generate 3 passwords each from a different pattern

```text
$ xpg wwww WWWW TTTT
!run:../target/release/xpg wwww WWWW TTTT
```

#### Apply upper casing

```text
$ xpg -A upper 'T T T T'
!run:../target/release/xpg -A upper 'T T T T'
```

#### Apply lower casing

```text
$ xpg -A lower 'T T T T'
!run:../target/release/xpg -A lower 'T T T T'
```

#### Apply title casing

```text
$ xpg -A title 'T T T T'
!run:../target/release/xpg -A title 'T T T T'
```

#### Apply toggle casing

```text
$ xpg -A toggle 'T T T T'
!run:../target/release/xpg -A toggle 'T T T T'
```

#### Apply camel casing

```text
$ xpg -A camel 'T T T T'
!run:../target/release/xpg -A camel 'T T T T'
```

#### Apply pascal casing

```text
$ xpg -A pascal 'T T T T'
!run:../target/release/xpg -A pascal 'T T T T'
```

#### Apply upper-camel casing

```text
$ xpg -A upper-camel 'T T T T'
!run:../target/release/xpg -A upper-camel 'T T T T'
```

#### Apply snake casing

```text
$ xpg -A snake 'T T T T'
!run:../target/release/xpg -A snake 'T T T T'
```

#### Apply upper-snake casing

```text
$ xpg -A upper-snake 'T T T T'
!run:../target/release/xpg -A upper-snake 'T T T T'
```

#### Apply screaming-snake casing

```text
$ xpg -A screaming-snake 'T T T T'
!run:../target/release/xpg -A screaming-snake 'T T T T'
```

#### Apply kebab casing

```text
$ xpg -A kebab 'T T T T'
!run:../target/release/xpg -A kebab 'T T T T'
```

#### Apply cobol casing

```text
$ xpg -A cobol 'T T T T'
!run:../target/release/xpg -A cobol 'T T T T'
```

#### Apply upper-kebab casing

```text
$ xpg -A upper-kebab 'T T T T'
!run:../target/release/xpg -A upper-kebab 'T T T T'
```

#### Apply train casing

```text
$ xpg -A train 'T T T T'
!run:../target/release/xpg -A train 'T T T T'
```

#### Apply flat casing

```text
$ xpg -A flat 'T T T T'
!run:../target/release/xpg -A flat 'T T T T'
```

#### Apply upper-flat casing

```text
$ xpg -A upper-flat 'T T T T'
!run:../target/release/xpg -A upper-flat 'T T T T'
```

#### Apply alternating casing

```text
$ xpg -A alternating 'T T T T'
!run:../target/release/xpg -A alternating 'T T T T'
```

#### Apply random casing

```text
$ xpg -A random 'T T T T'
!run:../target/release/xpg -A random 'T T T T'
```

#### Apply pseudo-random casing

```text
$ xpg -A pseudo-random 'T T T T'
!run:../target/release/xpg -A pseudo-random 'T T T T'
```

#### Generate a password with words with 1, 2, 3, 4, and 5 syllables

```text
$ xpg '{a:1}-{W:a:2}-{T:a:3}-{olympian:4}-{T:ext:5}'
!run:../target/release/xpg '{a:1}-{W:a:2}-{T:a:3}-{olympian:4}-{T:ext:5}'
```

#### List the words in Color / `{color}`

```text
$ xpg -L '{color}'
!run:../target/release/xpg -L '{color}'
```

#### List the words in Color / `{color}` in uppercase

```text
$ xpg -L '{W:color}'
!run:../target/release/xpg -L '{W:color}'
```

#### List the words in Color / `{color}` in title case

```text
$ xpg -L '{T:color}'
!run:../target/release/xpg -L '{T:color}'
```

#### Count the words in Color / `{color}`

```text
$ xpg -L '{color}' |wc -l
!run:../target/release/xpg -L '{color}' |wc -l
```

#### List the words in Color / `{color}` and Element / `{W:el}`

```text
$ xpg -L '{color}' '{W:el}'
!run:../target/release/xpg -L '{color}' '{W:el}'
```

#### Count words

```text
$ xpg -L |wc -l
!run:../target/release/xpg -L |wc -l
```

#### Count words with extended

```text
$ xpg -Le |wc -l
!run:../target/release/xpg -Le |wc -l
```

# Configuration

See [`config.json`] or use `-d` for the default configuration compiled into the
binary executable.

[`config.json`]: config.json

If another file is placed at any of the following locations or the path provided
via the `-C` option at *runtime*, it will be used instead.

* Linux:
  `/home/username/.config/xpg/config.json`
* Windows:
  `C:\Users\username\AppData\Roaming\qtfkwk\xpg\config\config.json`
* macOS:
  `/Users/username/Library/Application Support/com.qtfkwk.xpg/config.json`

## `WordKind`s

* AllExtended / `{a.ext}`
    * All / `{a}`
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
                        * GreekMyth / `{greekmyth}`
                            * Olympian / `{olympian}`
                            * Chthonic / `{chthonic}`
                        * RomanMyth / `{romanmyth}`
        * Preposition / `{prep}`
        * Verb / `{v}`
            * AuxiliaryVerb / `{v.aux}`
            * IntransitiveVerb / `{v.int}`
            * TransitiveVerb / `{v.tr}`
            * VerbPast / `{v.past}`
    * Extended / `{ext}`
        * Noun / `{n}`
            * SingularNoun / `{n.s}`
                * Astronomy / `{ast}`
                    * Moon / `{moon}`
                        * MarsMoon / `{marsmoon}`
                        * JupiterMoon / `{jupitermoon}`
                        * SaturnMoon / `{saturnmoon}`
                        * UranusMoon / `{uranusmoon}`
                        * NeptuneMoon / `{neptunemoon}`

**Notes**

1. WordKind / `{sub}`

2. It is only necessary to set the most descriptive `WordKind`(s) since they are
   automatically included in the larger word kinds.

3. The `characters` object value must contain the `C`, `c`, and `d` keys with
   non-empty values.
   Additional keys/values can be added and used.

4. `{sub}`s produce lowercase words by default, but if prefixed with `W:` or
   `T:` words will be uppercase or title case, respectively: i.e. `{W:sub}` or
   `{T:sub}` (replace `sub` with an actual `{sub}` name above).

5. `{sub}`s include all words by default, but if suffixed with `:N` where `N` is
   a number, it will only include words with that number of syllables: i.e.
   `{sub:N}` (replace `sub` with an actual `{sub}` name above and `N` with the
   desired number of syllables ~1-5).

# Changelog

Please find the [`CHANGELOG.md`] in the [repository].

[`CHANGELOG.md`]: https://github.com/qtfkwk/xpg/blob/master/CHANGELOG.md
[repository]: https://github.com/qtfkwk/xpg

# References

* Word list originated from Bart Busschots'
  [HSXKPasswd](https://www.bartbusschots.ie/s/publications/software/xkpasswd/)
  Perl module ([GitHub](https://github.com/bbusschots/hsxkpasswd),
  [CPAN: Crypt::HSXKPasswd](http://search.cpan.org/perldoc?Crypt%3A%3AHSXKPasswd)),
  specifically
  [lib/Crypt/HSXKPasswd/Dictionary/EN.pm@1d88564:38](https://github.com/bbusschots/hsxkpasswd/blob/1d88564d5bf74cf48025b372bcb635fc022962dd/lib/Crypt/HSXKPasswd/Dictionary/EN.pm#L38)

