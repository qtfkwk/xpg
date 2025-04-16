# About

xkcd-style password generator

![](t/fig/password_strength.png)

[xkcd #936: Password Strength](https://xkcd.com/936/)

# Usage

```text
$ xpg -h
xpg 0.18.2 <https://crates.io/crates/xpg>

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
xpg 0.18.2
```

*See also the [Configuration] section below.*

[Configuration]: #configuration

### Examples

```text
$ xpg
greecemothergravejoined
```

#### Generate 10 passwords

```text
$ xpg -c 10
pulledprocessmomentgeneral
untildetailsfriendsattempt
horsewifestopmatter
burningchangedarktrade
datephraseclothsettle
aprilbottombeatgrave
againstbusinesslaughtergone
angertillhappensubject
characterinsidecountryloud
birdbothquitetwelve
```

#### Generate passwords forever

*or until Ctrl+C / SIGINT*

```text
$ xpg -c 0
morningstartpoundsform
strangeschoolmembernecessary
sorrytowntimealso
againstcoldmeetturn
periodworkersbroughtenjoy
necessaryindiagraverest
goodalaskasundayguess
heavyveryreachterms
friendburnthancanada
capitalyourreadyrush
^C
```

#### Generate a password with 8 words

```text
$ xpg wwwwwwww
factorscovervenusoftencorrectcallcostgreen
```

#### Generate 10 passwords with 8 words

```text
$ xpg wwwwwwww -c 10
determinenearlycottonropeinformationhuntingsignalyesterday
becameplainspokenetherlandsrussiafillelselift
mountainpartygibraltartiedclothwarmexperimenttill
leftlaughinformationarmyunderdoesthentree
aroundoceandatesomeonewholecropsrepeatednoun
smalldancesaturdayliftedscenedescribenoonobserve
valleyearlysweetrightfinishentirecompletescience
cattleorderlyshoutedjumpedswedencirclefoothorse
probablesuddencareshoutdifferenceseptemberconditionstherefore
nextraisedfractiongreenthoughttoolsvalueeven
```

#### Generate a password with 2 words followed by 3 digits

```text
$ xpg wwddd
begandown355
```

#### Generate a password with 2 words followed by 2 symbols

```text
$ xpg wwss
aboutstars~{
```

#### Generate a password with 2 words followed by a digit and a symbol

```text
$ xpg wwds
damascusthin7<
```

#### Generate a password with 4 words followed by 2 lowercase letters

```text
$ xpg wwwwcc
guardthoughtemperatureslowlycb
```

#### Generate a password with 4 words followed by 2 uppercase letters

```text
$ xpg wwwwCC
whosecasefijiwaveWI
```

#### Generate a password with 4 title case words

```text
$ xpg TTTT
LakeSingaporeAustraliaEscape
```

#### Generate a password with 4 uppercase words

```text
$ xpg WWWW
MISTERCOUNTRYRUSSIATRIED
```

#### Generate a password with 4 words followed by 5 *any* characters

*An "any" character can be a lowercase letter, uppercase letter, digit, or
symbol.*

```text
$ xpg wwwwaaaaa
forgetmoleculesperustatemwRma
```

#### Generate a password with 3 words and at least 15 characters

```text
$ xpg www -m 15
choosedegreetraining
```

#### Generate a password with 3 words and no more than 20 characters

```text
$ xpg www -M 20
russiarisecool
```

#### Generate a password with 4 words and between 20 and 25 characters

```text
$ xpg www -m 20 -M 25 -a 1000
withoutthemselvescase
```

*See also [Usage] note 2.*

[Usage]: #usage

#### Generate a password with 2 words, 3 digits, 1 symbol, and exactly 16 characters

```text
$ xpg wwddds -l 16 -a 1000
behindchance208=
```

*See also [Usage] note 2.*

#### Generate a password with a `1`, 2 title case words, and `!`

```text
$ xpg '1TT!'
1DivideOrder!
```

#### Generate a password with 1 symbol, 2 adjectives, 1 noun, and 2 digits

```text
$ xpg 's{adj}{adj}{n}dd'
[deepsuddenwhere89
```

#### Generate a password with 2 words and 1 extended word

```text
$ xpg 'ww{ext}'
beganforeverorthosie
```

#### Generate a password with 2 words and 1 title case extended word

```text
$ xpg 'ww{T:ext}'
darefailIapetus
```

#### Generate a password with 2 words and 1 uppercase extended word

```text
$ xpg 'ww{W:ext}'
sharpmatchSAO
```

#### Generate 10 passwords with 4 title case words and extended words merged

```text
$ xpg -ec 10 TTTT
WyomingConsiderableCarrySharp
CallistoVerbMinuteDusk
CakeFloorBrazilFood
ChildhoodNothingApolloSound
SpaceChairDistanceAcross
BurningMilkShoreFeel
HerseShakeRiseBest
FloorForeignCrowdBlack
FractionFranceOrderAmount
BeingBuildRiddenStaple
```

#### Generate a keychain-style password

```text
$ xpg keychain
suqvJe-kwz8mf-igtsqb
```

#### Generate a code name

```text
$ xpg codename
SAFE BIRDS
```

#### Generate a code name series

```text
$ xpg codename-series
FRENCHMERCURY LATERCHANCE
FRENCHMERCURY DECIDEDSHIP
FRENCHMERCURY PARTIALTUESDAY
FRENCHMERCURY DISTANTFAIR
FRENCHMERCURY PRETTYLETTER
FRENCHMERCURY LEASTTEAM
FRENCHMERCURY THINLIAR
FRENCHMERCURY JAPANESEMONTH
FRENCHMERCURY SUCHDEEP
FRENCHMERCURY MAJORSOUND
```

#### Generate a code name series with 20 code names

```text
$ xpg codename-series -c 20
SIMPLENOTICE OPENINSIDE
SIMPLENOTICE WILDGROUND
SIMPLENOTICE SHALLTRIANGLE
SIMPLENOTICE CORRECTSHOW
SIMPLENOTICE FEELINGWEIGHT
SIMPLENOTICE SICKFIRE
SIMPLENOTICE EXCITINGLIGHT
SIMPLENOTICE FAMOUSWEDNESDAY
SIMPLENOTICE LEFTCAUSE
SIMPLENOTICE SILENTFORTY
SIMPLENOTICE EASYJUPITER
SIMPLENOTICE DEVELOPEDDUSK
SIMPLENOTICE TRUEMOLECULES
SIMPLENOTICE HUGEDROP
SIMPLENOTICE FULLFOOD
SIMPLENOTICE WHOLEWILD
SIMPLENOTICE BROKEPLAY
SIMPLENOTICE CURRENTSTREET
SIMPLENOTICE COOLCORN
SIMPLENOTICE INDIANCHARGE
```

#### Generate a haiku

```text
$ xpg haiku
Animal engine / Side Bulgaria upon / Shape experiment
```

#### Generate a haiku with syllable pattern

```text
$ xpg haiku+
Saturday hair round (3,1,1) / Burning second solution (2,2,3) / Scotland slowly seeds (2,2,1)
```

#### Generate a condensed haiku with syllable pattern

```text
$ xpg haiku-
Considerable(5)/Anything-Arizona(3,4)/Short-orderly-church(1,3,1)
```

#### Generate a condensed haiku

```text
$ xpg haiku--
English-galaxy/Sudden-history-skin-cloud/California-reach
```

#### Generate a haiku with newlines

```text
$ xpg haiku++
Similar winter
February honor bill
Wire Amsterdam strong
```

#### Generate a haiku with syllable pattern and newlines

```text
$ xpg haiku+++
Material wise (4,1)
Electricity name walk (5,1,1)
Period paid said (3,1,1)
```

#### Generate a haiku with extended words

```text
$ xpg -e haiku
Harpalyke plain / Difference Triton garden / Forward titania
```

#### Generate a password with 20 *any* characters

```text
$ xpg aaaaaaaaaaaaaaaaaaaa
P%f*KDk]5>a]J$DCtZdE
```

#### Same as the previous example, but shorter

*This works because a character-only pattern is padded to the minimum length
with the last character.*

```text
$ xpg a -m 20
ZG1K0vb=&^n)#R9J2h?=
```

#### Generate a password with 5 lowercase, 1 uppercase, 1 digit, and 1 symbol

```text
$ xpg cccccCds
cqduoS3^
```

#### Generate a password with 5 lowercase, 1 uppercase, 1 digit, and 1 symbol and shuffle the characters

```text
$ xpg -s cccccCds
7S?axqcx
```

#### Same as the previous example, but shorter

*This works because a character-only pattern is padded to the minimum length
with the last character.*

```text
$ xpg -sm 8 Cdsc
Qz=wx8po
```

#### Generate 3 passwords each from a different pattern

```text
$ xpg wwww WWWW TTTT
strikenovemberwaterniece
DEEPUSUALBELGIUMBRITAIN
CoverDanceCaughtDiscovered
```

#### Apply upper casing

```text
$ xpg -A upper 'T T T T'
DESIGN COST SHOULDER CALL
```

#### Apply lower casing

```text
$ xpg -A lower 'T T T T'
says galaxy ahead discovered
```

#### Apply title casing

```text
$ xpg -A title 'T T T T'
Cool Represent Asia Apple
```

#### Apply toggle casing

```text
$ xpg -A toggle 'T T T T'
fISH tHIS sHALL fRIDAY
```

#### Apply camel casing

```text
$ xpg -A camel 'T T T T'
meanMaybeHusbandMatch
```

#### Apply pascal casing

```text
$ xpg -A pascal 'T T T T'
MainRollSweetWith
```

#### Apply upper-camel casing

```text
$ xpg -A upper-camel 'T T T T'
FireNoseSailFeed
```

#### Apply snake casing

```text
$ xpg -A snake 'T T T T'
dublin_true_burning_happen
```

#### Apply upper-snake casing

```text
$ xpg -A upper-snake 'T T T T'
THEN_INSTRUMENTS_STUDY_SENSE
```

#### Apply screaming-snake casing

```text
$ xpg -A screaming-snake 'T T T T'
CURRENT_FOOT_LAUGHED_MINE
```

#### Apply kebab casing

```text
$ xpg -A kebab 'T T T T'
anything-front-straight-soldiers
```

#### Apply cobol casing

```text
$ xpg -A cobol 'T T T T'
WHERE-FIFTEEN-EDGE-ROOM
```

#### Apply upper-kebab casing

```text
$ xpg -A upper-kebab 'T T T T'
MAYBE-SLEEP-ANYTHING-PLUTO
```

#### Apply train casing

```text
$ xpg -A train 'T T T T'
Gave-Easy-Color-Proud
```

#### Apply flat casing

```text
$ xpg -A flat 'T T T T'
donechildhoodaheadpluto
```

#### Apply upper-flat casing

```text
$ xpg -A upper-flat 'T T T T'
PARISSTRAIGHTJULYANYTHING
```

#### Apply alternating casing

```text
$ xpg -A alternating 'T T T T'
lEvEl PaPeR cOlOmBiA tOtAl
```

#### Apply random casing

```text
$ xpg -A random 'T T T T'
faIr MErCURy WOmaN rEAd
```

#### Apply pseudo-random casing

```text
$ xpg -A pseudo-random 'T T T T'
FrEsh CoTtON hiSToRy dOuBT
```

#### Generate a password with words with 1, 2, 3, 4, and 5 syllables

```text
$ xpg '{a:1}-{W:a:2}-{T:a:3}-{olympian:4}-{T:ext:5}'
wheat-HUNDRED-Korea-aphrodite-Southamerica
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

