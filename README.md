# About

xkcd-style password generator

![](t/fig/password_strength.png)

[xkcd #936: Password Strength](https://xkcd.com/936/)

# Usage

```text
$ xpg -h
xpg 0.18.1 <https://crates.io/crates/xpg>

xkcd-style password generator

Usage: xpg [OPTIONS] [PATTERNS]...

Arguments:
  [PATTERNS]...  Pattern(s) (see note 1) [default: `wwww`, `-L`: `{a}`]

Options:
  -s, --shuffle             Shuffle characters
  -c, --count <NUMBER>      Number of passwords (0:∞ ) [default: 1]
  -l, --length <NUMBER>     Length
  -m, --minimum <NUMBER>    Minimum length
  -M, --maximum <NUMBER>    Maximum length
  -a, --attempts <NUMBER>   Attempts (see note 2) [default: 10]
  -C, --config <PATH>       Configuration file
  -e, --extended            Merge extended words (see note 3)
  -A, --apply-case <STYLE>  Apply case style [possible values: upper, lower,
                            title, toggle, camel, pascal, upper-camel, snake,
                            constant, upper-snake, screaming-snake, kebab,
                            cobol, upper-kebab, train, flat, upper-flat,
                            alternating, random, pseudo-random, sentence]
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
xpg 0.18.1
```

*See also the [Configuration] section below.*

[Configuration]: #configuration

### Examples

```text
$ xpg
greenfingersboneclean
```

#### Generate 10 passwords

```text
$ xpg -c 10
landoughtworldplace
crossdishmarylandgrew
thusreadyshinewrong
songprovidewrongcompound
withinitselffiverhythm
pleasuredonecountsaid
purelikelydealgroup
lowerroombelievecows
austriastapletexastrain
belgiumbelfaststudentturn
```

#### Generate passwords forever

*or until Ctrl+C / SIGINT*

```text
$ xpg -c 0
differencetrianglewavebecame
whatfamousmeetingsouthern
botswanademandhungermatch
februarytemperaturespendcovered
distanceplantearlife
ropebasketenjoydoor
gateleavemeanteam
poemtorespokeraise
becausemontanasugarstreet
uranuswashalonesurprise
^C
```

#### Generate a password with 8 words

```text
$ xpg wwwwwwww
cellsacrossplainwomanmarsjapaneseladypossible
```

#### Generate 10 passwords with 8 words

```text
$ xpg wwwwwwww -c 10
tomorrowhorsecookbreadsimplehersgoodweek
evergrainstandlistencaughtthenfrenchkorea
happyeightclothguesssavethankproveyour
opensilentjapanunderworthsmileddinnerbirds
cakemarrymoneymadridforwarddinnersectioncity
toldwelcomeusuallydesignsleptcovereddiscoverbone
businesspoemshinelowerpositionhopeheaveninclude
jordanladyfliersteelactionstockdoorright
snowstarshowneverseemheavyfebruaryplants
furtherpleasegentlemanstateshapestrikesymbolsbelgium
```

#### Generate a password with 2 words followed by 3 digits

```text
$ xpg wwddd
closefather360
```

#### Generate a password with 2 words followed by 2 symbols

```text
$ xpg wwss
chartadvance)>
```

#### Generate a password with 2 words followed by a digit and a symbol

```text
$ xpg wwds
augustdream8,
```

#### Generate a password with 4 words followed by 2 lowercase letters

```text
$ xpg wwwwcc
systemnotematterlaborbj
```

#### Generate a password with 4 words followed by 2 uppercase letters

```text
$ xpg wwwwCC
drinkcriedfivepartyMK
```

#### Generate a password with 4 title case words

```text
$ xpg TTTT
RealBirdCentLetter
```

#### Generate a password with 4 uppercase words

```text
$ xpg WWWW
DOESMISTERMAYOREXCEPT
```

#### Generate a password with 4 words followed by 5 *any* characters

*An "any" character can be a lowercase letter, uppercase letter, digit, or
symbol.*

```text
$ xpg wwwwaaaaa
paidcellsaftergroup+u9z?
```

#### Generate a password with 3 words and at least 15 characters

```text
$ xpg www -m 15
pleasedoubleboard
```

#### Generate a password with 3 words and no more than 20 characters

```text
$ xpg www -M 20
staymoretuesday
```

#### Generate a password with 4 words and between 20 and 25 characters

```text
$ xpg www -m 20 -M 25 -a 1000
childhooddeterminetools
```

*See also [Usage] note 2.*

[Usage]: #usage

#### Generate a password with 2 words, 3 digits, 1 symbol, and exactly 16 characters

```text
$ xpg wwddds -l 16 -a 1000
equalsuppose702=
```

*See also [Usage] note 2.*

#### Generate a password with a `1`, 2 title case words, and `!`

```text
$ xpg '1TT!'
1ClothTrue!
```

#### Generate a password with 1 symbol, 2 adjectives, 1 noun, and 2 digits

```text
$ xpg 's{adj}{adj}{n}dd'
+futuresoftgalaxy36
```

#### Generate a password with 2 words and 1 extended word

```text
$ xpg 'ww{ext}'
hawaiistraightrhodeisland
```

#### Generate a password with 2 words and 1 title case extended word

```text
$ xpg 'ww{T:ext}'
shapesilverAutonoe
```

#### Generate a password with 2 words and 1 uppercase extended word

```text
$ xpg 'ww{W:ext}'
daughterresentVALETUDO
```

#### Generate 10 passwords with 4 title case words and extended words merged

```text
$ xpg -ec 10 TTTT
SometimesPartDirectFuture
RhodeislandWelcomeFactorsClock
AroundCrowdMinnesotaMiss
NonePasiphaeDoneOrange
DinnerThereChiefHundred
SignMercuryReportConsiderable
ReplyPaidMineGive
CommonSugarTotalOutside
EveryoneExampleRegionColombia
WestvirginiaHeightWisconsinRepresent
```

#### Generate a keychain-style password

```text
$ xpg keychain
ytqtia-eHpwhh-xhp3zx
```

#### Generate a code name

```text
$ xpg codename
THROUGH OXYGEN
```

#### Generate a code name series

```text
$ xpg codename-series
TWELVEMAJOR PLAINTIME
TWELVEMAJOR THIRTEENWIND
TWELVEMAJOR AFTERWORKERS
TWELVEMAJOR JAPANESEDOUBT
TWELVEMAJOR REALEFFECT
TWELVEMAJOR RICHEXCEPT
TWELVEMAJOR EARLYSHADE
TWELVEMAJOR KNOWNWISE
TWELVEMAJOR NINESHAPE
TWELVEMAJOR YOUNGHOUR
```

#### Generate a code name series with 20 code names

```text
$ xpg codename-series -c 20
PROUDHUNT PUBLICSIGHT
PROUDHUNT DECIMALSURFACE
PROUDHUNT WRITTENNOSE
PROUDHUNT NATURALBEING
PROUDHUNT CHANCESPEED
PROUDHUNT WORNSOUND
PROUDHUNT THEREWELCOME
PROUDHUNT LONGTHROW
PROUDHUNT GENERALCOWS
PROUDHUNT MOSTSEED
PROUDHUNT SAFEADVANCE
PROUDHUNT ABLENIGHT
PROUDHUNT MAJORCURRENT
PROUDHUNT THESEFIGURE
PROUDHUNT THROUGHWELL
PROUDHUNT THREETERMS
PROUDHUNT KNOWNGRASS
PROUDHUNT FIFTEENPAIN
PROUDHUNT DEARUNIT
PROUDHUNT PRETTYSTRANGER
```

#### Generate a haiku

```text
$ xpg haiku
Carefully english / America consider / Arrived soldier wash
```

#### Generate a haiku with syllable pattern

```text
$ xpg haiku+
Electricity (5) / Cattle state December halt (2,1,3,1) / Considerable (5)
```

#### Generate a condensed haiku with syllable pattern

```text
$ xpg haiku-
Along-friday-bridge(2,2,1)/Britain-wednesday-people-rich(2,2,2,1)/Electricity(5)
```

#### Generate a condensed haiku

```text
$ xpg haiku--
Considerable/Botswana-particular/Attempt-snow-observe
```

#### Generate a haiku with newlines

```text
$ xpg haiku++
Botswana some stay
Considerable true main
Till Ireland fool
```

#### Generate a haiku with syllable pattern and newlines

```text
$ xpg haiku+++
Material watch (4,1)
Actually instruments back (3,3,1)
Considerable (5)
```

#### Generate a haiku with extended words

```text
$ xpg -e haiku
North Carolina / South Carolina Spain speak / South Dakota Mark
```

#### Generate a password with 20 *any* characters

```text
$ xpg aaaaaaaaaaaaaaaaaaaa
]qM<b6=0qPFhR7Qx0ze>
```

#### Same as the previous example, but shorter

*This works because a character-only pattern is padded to the minimum length
with the last character.*

```text
$ xpg a -m 20
Mw~H*>u~_z}Dx1t<Mk-F
```

#### Generate a password with 5 lowercase, 1 uppercase, 1 digit, and 1 symbol

```text
$ xpg cccccCds
tvjmwV0(
```

#### Generate a password with 5 lowercase, 1 uppercase, 1 digit, and 1 symbol and shuffle the characters

```text
$ xpg -s cccccCds
vwCsao9(
```

#### Same as the previous example, but shorter

*This works because a character-only pattern is padded to the minimum length
with the last character.*

```text
$ xpg -sm 8 Cdsc
tlp&Fo2u
```

#### Generate 3 passwords each from a different pattern

```text
$ xpg wwww WWWW TTTT
yourselfsingunderdifference
INTOTORETHANINFORMATION
SeparateNoonShoeHalf
```

#### Apply upper casing

```text
$ xpg -A upper 'T T T T'
DOUBLE FIFTY JAPANESE RESENT
```

#### Apply lower casing

```text
$ xpg -A lower 'T T T T'
mind country lone charge
```

#### Apply title casing

```text
$ xpg -A title 'T T T T'
Rolled Charge Carry Mean
```

#### Apply toggle casing

```text
$ xpg -A toggle 'T T T T'
nOUN sURPRISE gIBRALTAR eGYPT
```

#### Apply camel casing

```text
$ xpg -A camel 'T T T T'
knewDifficultShoesSeparate
```

#### Apply pascal casing

```text
$ xpg -A pascal 'T T T T'
WheelSoldierEnglandPrinted
```

#### Apply upper-camel casing

```text
$ xpg -A upper-camel 'T T T T'
DirectSilentPartyBurning
```

#### Apply snake casing

```text
$ xpg -A snake 'T T T T'
determine_attempt_total_last
```

#### Apply upper-snake casing

```text
$ xpg -A upper-snake 'T T T T'
SOUTHERN_DOUBT_PRICE_BUILD
```

#### Apply screaming-snake casing

```text
$ xpg -A screaming-snake 'T T T T'
AROUND_STAPLE_DRINK_SOUTHERN
```

#### Apply kebab casing

```text
$ xpg -A kebab 'T T T T'
hall-southern-loud-presidents
```

#### Apply cobol casing

```text
$ xpg -A cobol 'T T T T'
REALIZE-DRINK-COMPOUND-PARTICULAR
```

#### Apply upper-kebab casing

```text
$ xpg -A upper-kebab 'T T T T'
THANK-JUDGE-DISCOVER-DECEMBER
```

#### Apply train casing

```text
$ xpg -A train 'T T T T'
Over-Egypt-Angry-Half
```

#### Apply flat casing

```text
$ xpg -A flat 'T T T T'
bookshipwhichopinion
```

#### Apply upper-flat casing

```text
$ xpg -A upper-flat 'T T T T'
THINKRIDDENDOCTORYELLOW
```

#### Apply alternating casing

```text
$ xpg -A alternating 'T T T T'
uSuAl EqUaTiOn PuShEd BaBy
```

#### Apply random casing

```text
$ xpg -A random 'T T T T'
gRew NEEDlE EqUAtioN PhRASE
```

#### Apply pseudo-random casing

```text
$ xpg -A pseudo-random 'T T T T'
fInLaND cOmmON wAve HUrRy
```

#### Generate a password with words with 1, 2, 3, 4, and 5 syllables

```text
$ xpg '{a:1}-{W:a:2}-{T:a:3}-{olympian:4}-{T:ext:5}'
sleep-TODAY-Mercury-aphrodite-Southamerica
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

