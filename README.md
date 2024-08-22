# About

xkcd-style password generator

![](t/fig/password_strength.png)

[xkcd #936: Password Strength](https://xkcd.com/936/)

# Usage

```text
$ xpg -h
xpg 0.15.1 <https://crates.io/crates/xpg>

xkcd-style password generator

Usage: xpg [OPTIONS] [PATTERNS]...

Arguments:
  [PATTERNS]...  Pattern(s) (see note 1) [default: `wwww`, `-L`: `{a}`]

Options:
  -c, --count <NUMBER>     Number of passwords (0:∞ ) [default: 1]
  -l, --length <NUMBER>    Length
  -m, --minimum <NUMBER>   Minimum length
  -M, --maximum <NUMBER>   Maximum length
  -a, --attempts <NUMBER>  Attempts (see note 2) [default: 10]
  -C, --config <PATH>      Configuration file
  -e, --extended           Merge extended words (see note 3)
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
xpg 0.15.1
```

*See also the [Configuration] section below.*

[Configuration]: #configuration

### Examples

```text
$ xpg
copyoslobecomespace
```

#### Generate 10 passwords

```text
$ xpg -c 10
historydesignbutterbeautiful
moscowchanceshoesdrink
eggschangewhethershare
pleasefilledjordannight
tillstaplecopyquite
speedaustriadaresilver
indiansharplowerliar
gaveteartakenmany
simplehomeimportantchair
facewhatdelightdesire
```

#### Generate passwords forever

*or until Ctrl+C / SIGINT*

```text
$ xpg -c 0
inchesdelawareloneears
waveorderplanesing
britainthemrepeatedentered
roundkitchencarrysold
collegemachinecowsonce
leadertheremountainplural
shipfamilyhalfpage
gavethousandblowhawaii
clothesanswerwondermatch
oxygenspokesugarrome
^C
```

#### Generate a password with 8 words

```text
$ xpg wwwwwwww
dategeneralpreparedquietgoneenteredbecomefell
```

#### Generate 10 passwords with 8 words

```text
$ xpg wwwwwwww -c 10
easyfeelthanbrothertubelistmakedaily
goldpaintsaturdaynountroublecopywithskin
firstgamedoctorpickeverlasttellminute
fifthhallequationwheelspiecehowevereasystorm
walesstandcolourmonthsviewoverstudyalthough
allowsnowunderlineplanetbotswanaspokecolourfamous
melodyraiseslowlyfavormuchfinlandincludelord
irelandbringfinallycorrectfootrememberropegain
glassleavecrossbottomdropfieldstormbank
spotrosemountaingermanyexperiencestillfullinclude
```

#### Generate a password with 2 words followed by 3 digits

```text
$ xpg wwddd
memberyesterday429
```

#### Generate a password with 2 words followed by 2 symbols

```text
$ xpg wwss
largefact.#
```

#### Generate a password with 2 words followed by a digit and a symbol

```text
$ xpg wwds
helpsuggested2_
```

#### Generate a password with 4 words followed by 2 lowercase letters

```text
$ xpg wwwwcc
killguideniecehungerpf
```

#### Generate a password with 4 words followed by 2 uppercase letters

```text
$ xpg wwwwCC
remainlegsloneperiodJA
```

#### Generate a password with 4 title case words

```text
$ xpg TTTT
BehindCharacterRootSail
```

#### Generate a password with 4 uppercase words

```text
$ xpg WWWW
STRANGENEVADAPERIODSCIENCE
```

#### Generate a password with 4 words followed by 5 *any* characters

*An "any" character can be a lowercase letter, uppercase letter, digit, or
symbol.*

```text
$ xpg wwwwaaaaa
appleunderlineneptunemodernoNd/0
```

#### Generate a password with 3 words and at least 15 characters

```text
$ xpg www -m 15
preparedjudgeinformation
```

#### Generate a password with 3 words and no more than 20 characters

```text
$ xpg www -M 20
studentpulledwent
```

#### Generate a password with 4 words and between 20 and 25 characters

```text
$ xpg www -m 20 -M 25 -a 1000
berlinfuturenorthern
```

*See also [Usage] note 2.*

[Usage]: #usage

#### Generate a password with 2 words, 3 digits, 1 symbol, and exactly 16 characters

```text
$ xpg wwddds -l 16 -a 1000
peruneighbor579+
```

*See also [Usage] note 2.*

#### Generate a password with a `1`, 2 title case words, and `!`

```text
$ xpg '1TT!'
1GoodbyeInformation!
```

#### Generate a password with 1 symbol, 2 adjectives, 1 noun, and 2 digits

```text
$ xpg 's{adj}{adj}{n}dd'
+slowelsewild27
```

#### Generate a password with 2 words and 1 extended word

```text
$ xpg 'ww{ext}'
billwelloregon
```

#### Generate a password with 2 words and 1 title case extended word

```text
$ xpg 'ww{T:ext}'
cornscienceGalatea
```

#### Generate a password with 2 words and 1 uppercase extended word

```text
$ xpg 'ww{W:ext}'
hawaiidenmarkJANUARY
```

#### Generate 10 passwords with 4 title case words and extended words merged

```text
$ xpg -ec 10 TTTT
MarkAthenaTearPrice
ConsiderHimselfTakenTall
WashWantsParkBrother
AmsterdamNameOregonMatter
ThalassaBurningFootThrew
TogetherEngineUncleKenya
ShapeBeanSectionLittle
FineArrivedHundredAlso
LittleSummerColombiaShade
WeekSpeakPlanTemperature
```

#### Generate a keychain-style password

```text
$ xpg keychain
crxoor-uastQc-u5tfco
```

#### Generate a code name

```text
$ xpg codename
BUILT COLUMN
```

#### Generate a code name series

```text
$ xpg codename-series
WHICHTRADE SAFEMARKET
WHICHTRADE IRONMANNER
WHICHTRADE NOTHINGSTUDENT
WHICHTRADE SHALLOXYGEN
WHICHTRADE HAPPYFLAT
WHICHTRADE FEELINGPLANTS
WHICHTRADE ONLYPOINT
WHICHTRADE LONESHAKE
WHICHTRADE VARIOUSDESIRE
WHICHTRADE SHARPDRIVE
```

#### Generate a code name series with 20 code names

```text
$ xpg codename-series -c 20
EXPRESSDELIGHT VARIOUSYESTERDAY
EXPRESSDELIGHT USUALPART
EXPRESSDELIGHT NEARGALAXY
EXPRESSDELIGHT THROWNWEDNESDAY
EXPRESSDELIGHT DEEPSQUARE
EXPRESSDELIGHT WHATSPRING
EXPRESSDELIGHT TOGETHERVERB
EXPRESSDELIGHT SMALLSHOE
EXPRESSDELIGHT GLASSHUSBAND
EXPRESSDELIGHT AWAYGLASS
EXPRESSDELIGHT THIRDSUIT
EXPRESSDELIGHT NEVERHUNTING
EXPRESSDELIGHT FAIRWEEK
EXPRESSDELIGHT TWELVEQUESTIONS
EXPRESSDELIGHT MANYCROPS
EXPRESSDELIGHT IMPORTANTEUROPE
EXPRESSDELIGHT ROUNDDESIRE
EXPRESSDELIGHT REALLYPARAGRAPH
EXPRESSDELIGHT DAILYSTORY
EXPRESSDELIGHT MADEHAND
```

#### Generate a haiku

```text
$ xpg haiku
Damascus build touch / Especially opposite / Electricity
```

#### Generate a haiku with syllable pattern

```text
$ xpg haiku+
Roll Bulgaria (1,4) / Everyone Fiji fifty (3,2,2) / Cents thirteen spend feed (1,2,1,1)
```

#### Generate a condensed haiku with syllable pattern

```text
$ xpg haiku-
Considerable(5)/Dictionary-paragraph(4,3)/Material-view(4,1)
```

#### Generate a condensed haiku

```text
$ xpg haiku--
Considerable/Arizona-fruit-word-spent/Electricity
```

#### Generate a haiku with newlines

```text
$ xpg haiku++
Cells Tokyo wagon
Egypt once forever fight
Colombia seat
```

#### Generate a haiku with syllable pattern and newlines

```text
$ xpg haiku+++
Particular phrase (4,1)
California help member (4,1,2)
India thirteen (3,2)
```

#### Generate a haiku with extended words

```text
$ xpg -e haiku
Upon service sell / Thelxinoe paragraph / Matter window grain
```

#### Generate a password with 20 *any* characters

```text
$ xpg aaaaaaaaaaaaaaaaaaaa
&>ZQ03o7I0!mu!LhxNpv
```

#### Same as the previous example, but shorter

*This works because a character-only pattern is padded to the minimum length
with the last character.*

```text
$ xpg a -m 20
MU*{PfY5Avgn%u?&O/*a
```

#### Generate 3 passwords each from a different pattern

```text
$ xpg wwww WWWW TTTT
butterrealizeriddencharge
DEVICEFOOLBRIGHTSPAIN
OsloCarefullyWinterNeedle
```

#### Generate a password with words with 1, 2, 3, 4, and 5 syllables

```text
$ xpg '{a:1}-{W:a:2}-{T:a:3}-{olympian:4}-{T:ext:5}'
crops-MARKET-President-dionysus-Southcarolina
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

