# About

xkcd-style password generator

![](t/fig/password_strength.png)

[xkcd #936: Password Strength](https://xkcd.com/936/)

# Usage

```text
$ xpg -h
xpg 0.17.3 <https://crates.io/crates/xpg>

xkcd-style password generator

Usage: xpg [OPTIONS] [PATTERNS]...

Arguments:
  [PATTERNS]...  Pattern(s) (see note 1) [default: `wwww`, `-L`: `{a}`]

Options:
  -c, --count <NUMBER>      Number of passwords (0:∞ ) [default: 1]
  -l, --length <NUMBER>     Length
  -m, --minimum <NUMBER>    Minimum length
  -M, --maximum <NUMBER>    Maximum length
  -a, --attempts <NUMBER>   Attempts (see note 2) [default: 10]
  -C, --config <PATH>       Configuration file
  -e, --extended            Merge extended words (see note 3)
  -A, --apply-case <STYLE>  Apply case style [possible values: upper, lower,
                            title, toggle, camel, pascal, upper-camel, snake,
                            upper-snake, screaming-snake, kebab, cobol,
                            upper-kebab, train, flat, upper-flat, alternating,
                            random, pseudo-random]
  -L, --list                List words in `{sub}`(s)
  -d, --dump-config         Print configuration
  -r, --readme              Print readme
  -h, --help                Print help
  -V, --version             Print version

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
        * `haiku--` | `haiku-condensed`
        * `haiku-` | `haiku-with-syllables-condensed`
        * `haiku`
        * `haiku+` | `haiku-with-syllables`
        * `haiku++` | `haiku-full`
        * `haiku+++` | `haiku-full-with-syllables`

2. Ten attempts are made to satisfy the given length requirements by default;
   use `-a N` and set `N` to the number of attempts to increase chances.
   It will exit with an error if unable to satisfy requirements; some
   configurations will always produce an error.

3. Extended words are only available in exclusively extended word kinds by
   default; however, with `-e`, they are merged with regular word kinds; use
   with the `-L` option to see its effect.
```

```text
$ xpg -V
xpg 0.17.3
```

*See also the [Configuration] section below.*

[Configuration]: #configuration

### Examples

```text
$ xpg
largeamongindiabuilt
```

#### Generate 10 passwords

```text
$ xpg -c 10
duskusualmelodynotice
airplanemeetfactoriesdate
doublefillpaidcarefully
theirstateplantsomeone
grasscollegemonthhere
oceankindsightshould
slowlyamsterdamkingneck
evenstrikegoesmonth
besideshallbringsand
presidentohiosafeenjoy
```

#### Generate passwords forever

*or until Ctrl+C / SIGINT*

```text
$ xpg -c 0
discoveredclockholdvoice
shopsingsettleequal
eitherstatereallyjourney
nightbrothermaybeexercise
playeverhusbandseat
minuteswedgedeterminealone
differentcourseedgegoodbye
informationanswersuddenfence
shorelatersubjectforget
ringneptunehappenedsafe
^C
```

#### Generate a password with 8 words

```text
$ xpg wwwwwwww
westdollarslevelpositionlistshouldersignpoland
```

#### Generate 10 passwords with 8 words

```text
$ xpg wwwwwwww -c 10
factorieskentuckyshortsentsteelwhitedailywash
copylaterloststrangerpartialmarchgroupfront
voicemeatgiveairplanehusbandtroublehersdone
manymembersagreealonewentjoinedlifthalt
sistercouldparisspainstillairplanetheyarmy
dollarplansightoppositereachenteredfellowprocess
grouplikelynecessarycoveredrushremainchooseuntil
londonstarsintoelectricflierherssilvercalifornia
townjudgesuchtrademinutesallowwonderstep
fifthgatenorwaywatchvisitwavegeneralcattle
```

#### Generate a password with 2 words followed by 3 digits

```text
$ xpg wwddd
basketsend049
```

#### Generate a password with 2 words followed by 2 symbols

```text
$ xpg wwss
verbfollow.(
```

#### Generate a password with 2 words followed by a digit and a symbol

```text
$ xpg wwds
industrykiss7(
```

#### Generate a password with 4 words followed by 2 lowercase letters

```text
$ xpg wwwwcc
lonetailscoreleavepo
```

#### Generate a password with 4 words followed by 2 uppercase letters

```text
$ xpg wwwwCC
wateryourincludeableJM
```

#### Generate a password with 4 title case words

```text
$ xpg TTTT
DemandNothingRatherBanker
```

#### Generate a password with 4 uppercase words

```text
$ xpg WWWW
FORGETDRAWINGPOEMSAFETY
```

#### Generate a password with 4 words followed by 5 *any* characters

*An "any" character can be a lowercase letter, uppercase letter, digit, or
symbol.*

```text
$ xpg wwwwaaaaa
irelandcubatesttoolsck#fT
```

#### Generate a password with 3 words and at least 15 characters

```text
$ xpg www -m 15
insteadshiptwelve
```

#### Generate a password with 3 words and no more than 20 characters

```text
$ xpg www -M 20
laterstoodsight
```

#### Generate a password with 4 words and between 20 and 25 characters

```text
$ xpg www -m 20 -M 25 -a 1000
methodwelcomerepresent
```

*See also [Usage] note 2.*

[Usage]: #usage

#### Generate a password with 2 words, 3 digits, 1 symbol, and exactly 16 characters

```text
$ xpg wwddds -l 16 -a 1000
beautydesert296*
```

*See also [Usage] note 2.*

#### Generate a password with a `1`, 2 title case words, and `!`

```text
$ xpg '1TT!'
1ChinaDried!
```

#### Generate a password with 1 symbol, 2 adjectives, 1 noun, and 2 digits

```text
$ xpg 's{adj}{adj}{n}dd'
!thindailydouble03
```

#### Generate a password with 2 words and 1 extended word

```text
$ xpg 'ww{ext}'
friendwritepersephone
```

#### Generate a password with 2 words and 1 title case extended word

```text
$ xpg 'ww{T:ext}'
ropebirdAphrodite
```

#### Generate a password with 2 words and 1 uppercase extended word

```text
$ xpg 'ww{W:ext}'
daughterblackZEUS
```

#### Generate 10 passwords with 4 title case words and extended words merged

```text
$ xpg -ec 10 TTTT
SquareDespinaClothIdaho
SimilarLeastLookLeda
HawaiiEggsPartyLaughed
ThinkCenturyDollarVowel
EntireGeorgiaNothingQueen
MouthWellHumanJumped
ShoreKaleHopeTell
KitchenDollarHelloSpeak
WhetherWifeMainTrip
AuntNecessaryCountryFelt
```

#### Generate a keychain-style password

```text
$ xpg keychain
phxxgn-zinSje-qdt3bl
```

#### Generate a code name

```text
$ xpg codename
HEAVY MASTER
```

#### Generate a code name series

```text
$ xpg codename-series
WIDEWHOLE NEARPOWER
WIDEWHOLE LIKELYRIVER
WIDEWHOLE DIFFICULTNIGHT
WIDEWHOLE YOUNGDUSK
WIDEWHOLE NORTHERNHALL
WIDEWHOLE ABOUTFIRST
WIDEWHOLE FURTHERWEAK
WIDEWHOLE NEITHEROTHER
WIDEWHOLE REALLYCHILDREN
WIDEWHOLE LESSSTOP
```

#### Generate a code name series with 20 code names

```text
$ xpg codename-series -c 20
FIRMLONG NEXTCOUNTRY
FIRMLONG QUITESOUTH
FIRMLONG HEAVYFLOOR
FIRMLONG ENGLISHBRIDGE
FIRMLONG FAIRCOTTON
FIRMLONG CLEANPLAN
FIRMLONG CORRECTPRESIDENT
FIRMLONG SHARPGROUND
FIRMLONG NEVERDESIRE
FIRMLONG SOFTNEED
FIRMLONG SAFEFACTORS
FIRMLONG SILENTSHINE
FIRMLONG WORNCHANCE
FIRMLONG SIMPLEBOTTOM
FIRMLONG PRINTEDSTAR
FIRMLONG MODERNSOLDIERS
FIRMLONG PUBLICHELP
FIRMLONG INDIANWHEEL
FIRMLONG THICKDEEP
FIRMLONG MANYCROPS
```

#### Generate a haiku

```text
$ xpg haiku
Japanese army / Electricity there game / Century nothing
```

#### Generate a haiku with syllable pattern

```text
$ xpg haiku+
Sense another book (1,3,1) / Process provide school Peru (2,2,1,2) / Face Belgium sent east (1,2,1,1)
```

#### Generate a condensed haiku with syllable pattern

```text
$ xpg haiku-
Electricity(5)/Busy-scene-lady-song-soft(2,1,2,1,1)/Considerable(5)
```

#### Generate a condensed haiku

```text
$ xpg haiku--
Thrown-Alabama/California-either-stock/Electricity
```

#### Generate a haiku with newlines

```text
$ xpg haiku++
Shot adjective roll
Market nothing bright desert
Broad company cloth
```

#### Generate a haiku with syllable pattern and newlines

```text
$ xpg haiku+++
Music women raised (2,2,1)
Moment electricity (2,5)
Jerusalem their (4,1)
```

#### Generate a haiku with extended words

```text
$ xpg -e haiku
Philophrosyne / Considerable score what / Electricity
```

#### Generate a password with 20 *any* characters

```text
$ xpg aaaaaaaaaaaaaaaaaaaa
=S/a1eHg8;LTYh11xO{4
```

#### Same as the previous example, but shorter

*This works because a character-only pattern is padded to the minimum length
with the last character.*

```text
$ xpg a -m 20
5zg9h9X_Gg^VDKOG47em
```

#### Generate 3 passwords each from a different pattern

```text
$ xpg wwww WWWW TTTT
auntclockveryseen
ITSELFPARTIALFROMHEAR
PressInterestSeptemberClear
```

#### Apply upper casing

```text
$ xpg -A upper 'T T T T'
EXPRESS SHINE SLEEP LIFE
```

#### Apply lower casing

```text
$ xpg -A lower 'T T T T'
finger kiss partial probable
```

#### Apply title casing

```text
$ xpg -A title 'T T T T'
Village Material Keep Jamaica
```

#### Apply toggle casing

```text
$ xpg -A toggle 'T T T T'
fENCE aNGRY pOEM vALUE
```

#### Apply camel casing

```text
$ xpg -A camel 'T T T T'
coverGreeceNorthernPortugal
```

#### Apply pascal casing

```text
$ xpg -A pascal 'T T T T'
SuggestedSecondAmountDeep
```

#### Apply upper-camel casing

```text
$ xpg -A upper-camel 'T T T T'
ExactlyConsiderBreakLaughed
```

#### Apply snake casing

```text
$ xpg -A snake 'T T T T'
really_season_farmers_insects
```

#### Apply upper-snake casing

```text
$ xpg -A upper-snake 'T T T T'
RIDDEN_CROSS_CHILDREN_ISLAND
```

#### Apply screaming-snake casing

```text
$ xpg -A screaming-snake 'T T T T'
BIRD_NOTHING_CUBA_BEST
```

#### Apply kebab casing

```text
$ xpg -A kebab 'T T T T'
angry-norway-swim-drawing
```

#### Apply cobol casing

```text
$ xpg -A cobol 'T T T T'
DICTIONARY-EVERY-SCALE-VALLEY
```

#### Apply upper-kebab casing

```text
$ xpg -A upper-kebab 'T T T T'
ROAD-TELL-RATHER-LISTEN
```

#### Apply train casing

```text
$ xpg -A train 'T T T T'
Behind-Name-Region-Havana
```

#### Apply flat casing

```text
$ xpg -A flat 'T T T T'
presidentsthinghardnumeral
```

#### Apply upper-flat casing

```text
$ xpg -A upper-flat 'T T T T'
FEETCOOLNEVADABABY
```

#### Apply alternating casing

```text
$ xpg -A alternating 'T T T T'
nOtE sAfEtY sToP sInGlE
```

#### Apply random casing

```text
$ xpg -A random 'T T T T'
PLAneT sumMEr TOok THIS
```

#### Apply pseudo-random casing

```text
$ xpg -A pseudo-random 'T T T T'
ReAlIzE bEgIn pERsoN sEeM
```

#### Generate a password with words with 1, 2, 3, 4, and 5 syllables

```text
$ xpg '{a:1}-{W:a:2}-{T:a:3}-{olympian:4}-{T:ext:5}'
bird-FOREST-Division-aphrodite-Northamerica
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
1403
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

