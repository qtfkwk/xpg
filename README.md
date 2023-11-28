# About

xkcd-style password generator

![](t/fig/password_strength.png)

[xkcd #936: Password Strength](https://xkcd.com/936/)

# Usage

```text
$ xpg -h
xpg 0.11.0 <https://crates.io/crates/xpg>

xkcd-style password generator

Usage: xpg [OPTIONS] [PATTERNS]...

Arguments:
  [PATTERNS]...  Pattern(s) (see note 1) [default: `wwww`, `-L`: `{a}`]

Options:
  -c, --count <NUMBER>     Number of passwords (0:∞ ) [default: 1]
  -l, --length <NUMBER>    Length
  -m, --minimum <NUMBER>   Minimum length
  -M, --maximum <NUMBER>   Maximum length
  -a, --attempts <NUMBER>  Attempts [default: 10]
  -C, --config <PATH>      Configuration file
  -e, --extended           Merge extended words (see note 2)
  -L, --list               List words in `{sub}`(s)
  -d, --dump-config        Print configuration
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
        * `haiku`
        * `haiku+`
        * `haiku-`
        * `haiku--`
        * `haiku++`
        * `haiku+++`

2. By default, extended words are only available in exclusively extended word
   kinds; however, with `-e`, they are merged with regular word kinds; use with
   the `-L` option to see its effect.
```

```text
$ xpg -V
xpg 0.11.0
```

*See also the [Configuration] section below.*

[Configuration]: #configuration

### Examples

```text
$ xpg
soilpromisebattlebones
```

#### Generate 10 passwords

```text
$ xpg -c 10
livewouldreallygreen
decidedstationfrenchchile
lesscoastbearriver
softcongobelgiumthirteen
mightserviceuponwife
darelisbonriddenbelieve
fiftynecessaryfindwindow
broadwithtalkiron
theirloudaugustthese
dreamfromnationrequire
```

#### Generate passwords forever

*or until Ctrl+C / SIGINT*

```text
$ xpg -c 0
knowlivereallylose
thankfellowwantbranches
shouldmeattradehurt
necessarycattlehighnation
shownlaughdetailsbanker
plantoldhuntingdevice
centurymusicangrybanker
untilrepeatedjupiterclothes
fellmountainjerusalemchair
understanduranusmondaydried
^C
```

#### Generate a password with 8 words

```text
$ xpg wwwwwwww
perhapsminuteswagonskingravebeganplainshort
```

#### Generate 10 passwords with 8 words

```text
$ xpg wwwwwwww -c 10
daughterwhenosloirelandshirtperudarktone
whichdreamcasespeedshotcloudresenthours
alwaysrushcannotcropsthinkseveraldollarsfront
verbbellleftblueplainquartergathercorrect
clotheslikedetermineadvancetoldtheremaltaonly
takefreescoremuchwashwornchangerolled
thesenieceislandturnenginehappenstudentsheld
considerablepushpleasenumberleftlanguagedreamfood
streethimselfhorseproveshineitalyreadyengine
northernsimilardevelopedsettlestrengthfellbreadgate
```

#### Generate a password with 2 words followed by 3 digits

```text
$ xpg wwddd
teacherincrease900
```

#### Generate a password with 2 words followed by 2 symbols

```text
$ xpg wwss
sharesurface@]
```

#### Generate a password with 2 words followed by a digit and a symbol

```text
$ xpg wwds
roombehind3%
```

#### Generate a password with 4 words followed by 2 lowercase letters

```text
$ xpg wwwwcc
includecakeladychildrentd
```

#### Generate a password with 4 words followed by 2 uppercase letters

```text
$ xpg wwwwCC
ablesingwednesdaybasketAM
```

#### Generate a password with 4 title case words

```text
$ xpg TTTT
AroundLevelSilentRiver
```

#### Generate a password with 4 uppercase words

```text
$ xpg WWWW
THEYMODERNINTERESTCHILDHOOD
```

#### Generate a password with 4 words followed by 5 "any" characters

*An "any" character can be a lowercase letter, uppercase letter, digit, or
symbol.*

```text
$ xpg wwwwaaaaa
whiteopiniondesigncanadalSAgU
```

#### Generate a password with 3 words and at least 15 characters

```text
$ xpg www -m 15
jumpgeneralfinland
```

#### Generate a password with 3 words and no more than 20 characters

```text
$ xpg www -M 20
readlonenumber
```

#### Generate a password with 4 words and between 20 and 25 characters

*Increase attempts to help it succeed.*

```text
$ xpg www -m 20 -M 25 -a 1000
whomdaughterglossary
```

#### Generate a password with 2 words, 3 digits, 1 symbol, and exactly 16 characters

*Increase attempts to help it succeed.*

```text
$ xpg wwddds -l 16 -a 1000
brightdevice068^
```

#### Generate a password with a `1`, 2 title case words, and `!`

```text
$ xpg '1TT!'
1GrainJerusalem!
```

#### Generate a password with 1 symbol, 2 adjectives, 1 noun, and 2 digits

```text
$ xpg 's{adj}{adj}{n}dd'
)theirshouldknown76
```

#### Generate a password with 2 words and 1 extended word

```text
$ xpg 'ww{ext}'
hallhoweveroceania
```

#### Generate a password with 2 words and 1 title case extended word

```text
$ xpg 'ww{T:ext}'
colordoubtPhilophrosyne
```

#### Generate a password with 2 words and 1 uppercase extended word

```text
$ xpg 'ww{W:ext}'
severalsuddenOBERON
```

#### Generate 10 passwords with 4 title case words and extended words merged

```text
$ xpg -ec 10 TTTT
ElectricTallMouthPlanet
InformationTinyNeedleIreland
VenusAdrasteaWorthIndustry
HeatPointStrangeThree
CauseThinkAgainstMust
BesideHungerHumanLend
HappenedFromFrontPain
FeltColourGroundAfrica
BerlinNevadaRushJupiter
BlueWholeProveLong
```

#### Generate a keychain-style password

```text
$ xpg keychain
y7dbml-uOyxyq-edsssr
```

#### Generate a code name

```text
$ xpg codename
DAILY SUPPLY
```

#### Generate a code name series

```text
$ xpg codename-series
PRETTYMONTH HUGEWATER
PRETTYMONTH FRONTBUILDING
PRETTYMONTH MODERNLIAR
PRETTYMONTH FIFTEENCHIEF
PRETTYMONTH WRONGBALL
PRETTYMONTH LARGESUFFIX
PRETTYMONTH SICKSTAND
PRETTYMONTH SEPARATEVALUE
PRETTYMONTH EIGHTDEAL
PRETTYMONTH PARTIALSMOKE
```

#### Generate a code name series with 20 code names

```text
$ xpg codename-series -c 20
THREETOWN FIFTEENCHILDREN
THREETOWN STRONGMOMENT
THREETOWN HEAVYGRAVE
THREETOWN EVERYBOARD
THREETOWN BEARCHECK
THREETOWN ROUNDQUARTER
THREETOWN RATHERPAIN
THREETOWN ONCETAIL
THREETOWN VERYROOM
THREETOWN MOSTELEMENTS
THREETOWN SUREMEAN
THREETOWN NINEDESERT
THREETOWN READYCOUNT
THREETOWN COLDSTONE
THREETOWN MAJORINCHES
THREETOWN BUILTFACTORIES
THREETOWN SWEETWEATHER
THREETOWN DARKHUMAN
THREETOWN FOREIGNCAPITAL
THREETOWN IRONLEVEL
```

#### Generate a haiku

```text
$ xpg haiku
Copenhagen shoes / Settle meet damascus part / Alabama sand
```

#### Generate a haiku with syllable pattern

```text
$ xpg haiku+
Temperature write (4,1) / Father jerusalem wheat (2,4,1) / Experience cook (4,1)
```

#### Generate a condensed haiku with syllable pattern

```text
$ xpg haiku-
Experience-thin(4,1)/Underline-party-upon(3,2,2)/Electricity(5)
```

#### Generate a condensed haiku

```text
$ xpg haiku--
America-sort/Division-serve-sure-tiny/Sweet-dictionary
```

#### Generate a haiku with newlines

```text
$ xpg haiku++
Orderly check glass
Power yard necessary
Jerusalem foot
```

#### Generate a haiku with syllable pattern and newlines

```text
$ xpg haiku+++
Experience field (4,1)
Difference stand shall match inch (3,1,1,1,1)
Particular proud (4,1)
```

#### Generate a password with 20 "any" characters

```text
$ xpg aaaaaaaaaaaaaaaaaaaa
kYCROEE;1rd#IDct?W}I
```

#### Same as the previous example, but shorter

*This works because a character-only pattern is padded to the minimum length
with the last character.*

```text
$ xpg a -m 20
Uy-V[soM!%e-]7H05%WW
```

#### Generate 3 passwords each from a different pattern

```text
$ xpg wwww WWWW TTTT
wedgewinterhappenedgray
FIELDWELCOMETRADESMOKE
SureColorWhichGarden
```

#### Generate a password with words with 1, 2, 3, 4, and 5 syllables

```text
$ xpg '{a:1}-{W:a:2}-{T:a:3}-{olympian:4}-{T:ext:5}'
salt-DETAILS-Important-aphrodite-Philophrosyne
```

#### List the words in Color / `{color}`

```text
$ xpg -L '{color}'
black
blue
brown
gray
green
white
yellow
```

#### List the words in Color / `{color}` in uppercase

```text
$ xpg -L '{W:color}'
BLACK
BLUE
BROWN
GRAY
GREEN
WHITE
YELLOW
```

#### List the words in Color / `{color}` in title case

```text
$ xpg -L '{T:color}'
Black
Blue
Brown
Gray
Green
White
Yellow
```

#### Count the words in Color / `{color}`

```text
$ xpg -L '{color}' |wc -l
7
```

#### List the words in Color / `{color}` and Element / `{W:el}`

```text
$ xpg -L '{color}' '{W:el}'
black
blue
brown
gray
green
white
yellow
GOLD
IRON
LEAD
MERCURY
OXYGEN
SILVER
```

#### Count words

```text
$ xpg -L |wc -l
1257
```

#### Count words with extended

```text
$ xpg -Le |wc -l
1400
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
* 0.8.1 (2023-11-25): fix `-C` option
* 0.9.0 (2023-11-26): add extended words; enable multiple patterns; add `-e` and
  `-L` options; make `-d` dump the loaded not default configuration
* 0.10.0 (2023-11-27): add syllables and `haiku*` special patterns; fix the
  codename series algorithm; make olympian/chthonic kinds extended; update
  dependencies
* 0.11.0 (2023-11-28): add bat pager on \*nix

# References

* Word list originated from Bart Busschots'
  [HSXKPasswd](https://www.bartbusschots.ie/s/publications/software/xkpasswd/)
  Perl module ([GitHub](https://github.com/bbusschots/hsxkpasswd),
  [CPAN: Crypt::HSXKPasswd](http://search.cpan.org/perldoc?Crypt%3A%3AHSXKPasswd)),
  specifically
  [lib/Crypt/HSXKPasswd/Dictionary/EN.pm@1d88564:38](https://github.com/bbusschots/hsxkpasswd/blob/1d88564d5bf74cf48025b372bcb635fc022962dd/lib/Crypt/HSXKPasswd/Dictionary/EN.pm#L38)

