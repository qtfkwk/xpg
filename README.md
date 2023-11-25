# About

xkcd-style password generator

![](t/fig/password_strength.png)

[xkcd #936: Password Strength](https://xkcd.com/936/)

# Usage

```text
$ xpg -h
xpg 0.8.0 <https://crates.io/crates/xpg>

xkcd-style password generator

Usage: xpg [OPTIONS] [PATTERN]

Arguments:
  [PATTERN]  Pattern (see note 1 below) [default: wwww]

Options:
  -c, --count <NUMBER>     Number of passwords (0:∞ ) [default: 1]
  -l, --length <NUMBER>    Length
  -m, --minimum <NUMBER>   Minimum length
  -M, --maximum <NUMBER>   Maximum length
  -a, --attempts <NUMBER>  Attempts [default: 10]
  -C, --config <PATH>      Configuration file
  -d, --default-config     Print default configuration
  -r, --readme             Print readme
  -h, --help               Print help
  -V, --version            Print version

---

Notes

1. Patterns
    * Words
        * `W`: WORD
        * `w`: word
        * `T`: Word
        * `k`: `shuffle(cccc(c|d)(C|c))`
        * `{sub}`: see `WordKind`s section in readme
    * Characters
        * `C`: A-Z
        * `c`: a-z
        * `d`: 0-9
        * `s`: `~!@#$%^&*-_=+;:,./?()[]{}<>`
        * `a`: `C` | `c` | `d` | `s`
    * Special
        * `keychain`
        * `codename`
        * `codename-series`
```

```text
$ xpg -V
xpg 0.8.0
```

*See also the [Configuration] section below.*

[Configuration]: #configuration

### Examples

```text
$ xpg
unittravelbelfastdance
```

Generate 10 passwords:

```text
$ xpg -c 10
kenyawentvillagesystem
statementofferseatgermany
sweetsoftpaintroad
futuresystemmontanagone
sundaytoolspicturebicycle
passedrecordinstrumentsoffice
osloscotlandlegsbeauty
watchlargearmyridden
cornerdesirehusbandsent
laborsymbolsconditionstrain
```

Generate passwords forever (or until Ctrl+C / SIGINT):

```text
$ xpg -c 0
rometouchslowfriday
goodlearnunderlinereached
greeceflierconsiderriver
plutotwelvenationlone
spokeaustraliashotgovern
alsosafeshoutedeight
remainalsomeatcalifornia
dishprobablymadequarter
spellworkersproudaway
rainwavenationguess
^C
```

Generate a password with 8 words:

```text
$ xpg wwwwwwww
poleunderstandpositionactionsystemcolombiaoppositegive
```

Generate 10 passwords with 8 words:

```text
$ xpg wwwwwwww -c 10
spendboatyearoppositemontanacleanstartafraid
escapeinsectsawayuponsoilsucceedgoesinches
asiasortsizenearprovetreeenoughskin
centbodyvalleylosepricefeltcannotsyllables
shoutedohionoselegspieceportugalsimilarthick
electricitycountpositioncenturypresidentshandtripcotton
instrumentsthrowncirclemattermeattherelanguagegold
worldseedseatfirstkingservicegreatshake
somethingstaplearounddollarevereverythingeitherchildren
aroundsciencecapitalcarefullyveryjourneyhollandstaple
```

Generate a password with 2 words followed by 3 digits:

```text
$ xpg wwddd
excitingpeace726
```

Generate a password with 2 words followed by 2 symbols:

```text
$ xpg wwss
thesemain*/
```

Generate a password with 2 words followed by a digit and a symbol:

```text
$ xpg wwds
suchproblem4_
```

Generate a password with 4 words followed by 2 lowercase letters:

```text
$ xpg wwwwcc
buildingmisterlaughperiodoa
```

Generate a password with 4 words followed by 2 uppercase letters:

```text
$ xpg wwwwCC
presidentssomeonepatternsentenceVX
```

Generate a password with 4 title case words:

```text
$ xpg TTTT
OxygenSoldBrotherNose
```

Generate a password with 4 uppercase words:

```text
$ xpg WWWW
AUSTRALIAFURTHERDESERTPOSITION
```

Generate a password with 4 words followed by 5 "any" characters (lowercase letter, uppercase letter,
digit, or symbol)

```text
$ xpg wwwwaaaaa
squareprovideshakeroomE*?pw
```

Generate a password with 3 words and at least 15 characters:

```text
$ xpg www -m 16
fingersbelliron
```

Generate a password with 3 words and no more than 20 characters:

```text
$ xpg www -M 20
perfectheldanything
```

Generate a password with 4 words and between 20 and 25 characters (increase attempts to help it
succeed):

```text
$ xpg www -m 20 -M 25 -a 1000
pureespeciallypretty
```

Generate a password with 2 words, 3 digits, 1 symbol, and exactly 16 characters (increase attempts
to help it succeed):

```text
$ xpg wwddds -l 16 -a 1000
observebirds823{
```

Generate a password with a `1`, 2 title case words, and `!`:

```text
$ xpg '1TT!'
1MovementSignal!
```

Generate a password with 1 symbol, 2 adjectives, 1 noun, and 2 digits:

```text
$ xpg 's{adj}{adj}{n}dd'
$excitingverytrack95
```

Generate a keychain-style password:

```text
$ xpg keychain
yymodw-xfvan6-txLzrm
```

Generate a code name:

```text
$ xpg codename
FIRM POINT
```

Generate a code name series:

```text
$ xpg codename-series
AWAYTUBE GONENIGHT
AWAYTUBE NEITHERBIRDS
AWAYTUBE BRITISHMORNING
AWAYTUBE PARTIALBONE
AWAYTUBE MAINEIGHT
AWAYTUBE PASSEDSHOES
AWAYTUBE LEFTNATION
AWAYTUBE THERESHADE
AWAYTUBE LOWERMOMENT
AWAYTUBE DIFFICULTPAIN
```

Generate a code name series with 20 code names:

```text
$ xpg codename-series -c 20
STRANGESCIENCE SIMILARGROUP
STRANGESCIENCE CLEARPARAGRAPH
STRANGESCIENCE THOUSANDFAVOR
STRANGESCIENCE BRITISHWORTH
STRANGESCIENCE FRENCHTELL
STRANGESCIENCE PRETTYSOUTH
STRANGESCIENCE RICHQUESTIONS
STRANGESCIENCE HEADARMS
STRANGESCIENCE EASYFUTURE
STRANGESCIENCE SICKSUFFIX
STRANGESCIENCE BRITISHBOARD
STRANGESCIENCE BROKENWIND
STRANGESCIENCE JUSTHUNDRED
STRANGESCIENCE FASTPARTIAL
STRANGESCIENCE GENERALLIST
STRANGESCIENCE OUTEROCTOBER
STRANGESCIENCE LEFTPOINT
STRANGESCIENCE LEADISLAND
STRANGESCIENCE PROBABLESOUND
STRANGESCIENCE SWEETPROCESS
```

Generate a password with 20 "any" characters:

```text
$ xpg aaaaaaaaaaaaaaaaaaaa
.ZghZ8XdBSbpOlJq$?>^
```

Same as the previous example, but shorter:

```text
$ xpg a -m 20
zf_?R]j,4)L#{nq<EC>S
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

# Changelog

* 0.1.0 (2019-09-08): store wordlist as constant and operate on it without
  copying; select N words without repetition via `choose_multiple`; tests;
  benchmark various implementations; `xpg` function; `xpg!` macro to enable
  optional word count argument; command line interface via clap
* 0.1.1 (2019-09-08): minor fixes
* 0.1.2 (2019-09-08): minor fixes
* 0.2.0 (2019-09-09): expose `xpg!` macro; improve documentation
* 0.3.0 (2022-10-01): update dependencies; add digits and symbols options
* 0.3.1 (2022-10-01): fix readme
* 0.4.0 (2022-10-02): add --lowercase, --uppercase, and --any options; enable
  --words option to be zero if --any option is greater than zero; enable
  infinite passwords via `-c 0`
* 0.5.0 (2022-10-13): add --keychain and --code-name options
* 0.6.0 (2023-11-22): remove num and factorial crate dependencies; improve
  permutations function; update dependencies; apply clippy fixes and cargo fmt;
  reorganize
* 0.6.1 (2023-11-22): fix readme
* 0.7.0 (2023-11-24): complete redesign
* 0.8.0 (2023-11-25): remove benchmark; simplify cli options; add `-r` and `-C`
  options; restore `-c 0` functionality; add `codename-series` special pattern;
  add word kind subpatterns; enable external runtime configuration via new
  `Config` struct

# References

* Word list originated from Bart Busschots'
  [HSXKPasswd](https://www.bartbusschots.ie/s/publications/software/xkpasswd/)
  Perl module ([GitHub](https://github.com/bbusschots/hsxkpasswd),
  [CPAN: Crypt::HSXKPasswd](http://search.cpan.org/perldoc?Crypt%3A%3AHSXKPasswd)),
  specifically
  [lib/Crypt/HSXKPasswd/Dictionary/EN.pm@1d88564:38](https://github.com/bbusschots/hsxkpasswd/blob/1d88564d5bf74cf48025b372bcb635fc022962dd/lib/Crypt/HSXKPasswd/Dictionary/EN.pm#L38)

