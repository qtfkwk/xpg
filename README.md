# About

xkcd-style password generator

![](t/fig/password_strength.png)

[xkcd #936: Password Strength](https://xkcd.com/936/)

# Usage

```text
$ xpg -h
xpg 0.19.0 <https://crates.io/crates/xpg>

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
xpg 0.19.0
```

*See also the [Configuration] section below.*

[Configuration]: #configuration

### Examples

```text
$ xpg
blackmoleculesfoolheld
```

#### Generate 10 passwords

```text
$ xpg -c 10
floridanetherlandsstillguess
mayorseedsenoughgrew
withsometimesjumpedneed
trackflowersverbcost
seedpleasantrestfive
fruitnorthernappearhawaii
leaderinsteadhaveloud
themselvesaugustwheelsscene
givessettledexercisethem
whitefancystarthome
```

#### Generate passwords forever

*or until Ctrl+C / SIGINT*

```text
$ xpg -c 0
feetvermontdestroynotice
mexicoseptemberfinesignal
savepreparedraiseworkers
answerwifebottledinner
sufferpaintsignaloxygen
explainironwhichname
carryfacearizonacried
girljourneysomeonestar
dealwritetrustscene
incheswhatbehindboard
^C
```

#### Generate a password with 8 words

```text
$ xpg wwwwwwww
throughcongoyesterdayscalestudylightweightover
```

#### Generate 10 passwords with 8 words

```text
$ xpg wwwwwwww -c 10
trackequalexpectflowroundspecialchildcause
observeneptunereplysleptoutsidecriedhopeshoe
kissslowwornmarkbookgavewednesdayshake
industrycarefullydutypartysurfacehandstudentbelieve
deadcausebattlefactorstalkcapitalexperimenttruck
succeeditalybelieveoctoberyourselfneighborwindowsmoke
fellowparticularthereforelossentirevenussubjectshop
pageorderfactorscontrolhorsedetailstrieschoose
rulegiveplaincountryclothescopenhagenlegsaustralia
galaxyamericabrazilonlyyourselfeffectshoulderfear
```

#### Generate a password with 2 words followed by 3 digits

```text
$ xpg wwddd
singlemonth871
```

#### Generate a password with 2 words followed by 2 symbols

```text
$ xpg wwss
clockpain]]
```

#### Generate a password with 2 words followed by a digit and a symbol

```text
$ xpg wwds
worldindia2<
```

#### Generate a password with 4 words followed by 2 lowercase letters

```text
$ xpg wwwwcc
squaresilentenoughwashcc
```

#### Generate a password with 4 words followed by 2 uppercase letters

```text
$ xpg wwwwCC
patternrealizeexperiencejapanJS
```

#### Generate a password with 4 title case words

```text
$ xpg TTTT
LaughedEachControlBill
```

#### Generate a password with 4 uppercase words

```text
$ xpg WWWW
GERMANYSPREADWHEELSCAPTAIN
```

#### Generate a password with 4 words followed by 5 *any* characters

*An "any" character can be a lowercase letter, uppercase letter, digit, or
symbol.*

```text
$ xpg wwwwaaaaa
devicehomegaveworkersly)ET
```

#### Generate a password with 3 words and at least 15 characters

```text
$ xpg www -m 15
europegreenweak
```

#### Generate a password with 3 words and no more than 20 characters

```text
$ xpg www -M 20
arizonaraisecents
```

#### Generate a password with 4 words and between 20 and 25 characters

```text
$ xpg www -m 20 -M 25 -a 1000
ringincreaseamsterdam
```

*See also [Usage] note 2.*

[Usage]: #usage

#### Generate a password with 2 words, 3 digits, 1 symbol, and exactly 16 characters

```text
$ xpg wwddds -l 16 -a 1000
chartsuppose909!
```

*See also [Usage] note 2.*

#### Generate a password with a `1`, 2 title case words, and `!`

```text
$ xpg '1TT!'
1StateBall!
```

#### Generate a password with 1 symbol, 2 adjectives, 1 noun, and 2 digits

```text
$ xpg 's{adj}{adj}{n}dd'
+separatetruestar38
```

#### Generate a password with 2 words and 1 extended word

```text
$ xpg 'ww{ext}'
ruletrainingorthosie
```

#### Generate a password with 2 words and 1 title case extended word

```text
$ xpg 'ww{T:ext}'
toldconditionTitania
```

#### Generate a password with 2 words and 1 uppercase extended word

```text
$ xpg 'ww{W:ext}'
wheelsimportantHELIKE
```

#### Generate 10 passwords with 4 title case words and extended words merged

```text
$ xpg -ec 10 TTTT
SwedenSlowMeetTest
SisterLendHeavenReady
GirlMotherPublicReceived
DelightJanuarySendMneme
TravelBelieveProbableWorkers
BeganWalesShotRepresent
EverHeavyEggsRock
ShareLifeStationAddition
SleepMatterDeepWithin
SignAoedeFearWritten
```

#### Generate a keychain-style password

```text
$ xpg keychain
lwidnX-bmcbwu-tkegp2
```

#### Generate a code name

```text
$ xpg codename
SEEN UNIT
```

#### Generate a code name series

```text
$ xpg codename-series
SIMILARSTORY EQUALSOLDIERS
SIMILARSTORY GRAVEGAIN
SIMILARSTORY OUTSIDETRAIN
SIMILARSTORY THOSEPLAN
SIMILARSTORY STRAIGHTTRAINING
SIMILARSTORY JAPANESEWOMAN
SIMILARSTORY BRITISHSECTION
SIMILARSTORY AFTERHURRY
SIMILARSTORY FIFTYHEAD
SIMILARSTORY FULLDOUBT
```

#### Generate a code name series with 20 code names

```text
$ xpg codename-series -c 20
FOREVERBREAD CLEANCOAT
FOREVERBREAD BOTTOMSHOW
FOREVERBREAD IRONSTEEL
FOREVERBREAD SILENTBATTERY
FOREVERBREAD USUALLYNECK
FOREVERBREAD LIVEPASS
FOREVERBREAD EVENKIND
FOREVERBREAD GENERALTRIP
FOREVERBREAD BROADEDGE
FOREVERBREAD SEVENMATERIAL
FOREVERBREAD SICKWEST
FOREVERBREAD EACHTEMPERATURE
FOREVERBREAD SMALLBEING
FOREVERBREAD EASTTROUBLE
FOREVERBREAD BRITISHJOURNEY
FOREVERBREAD YOURGRAVE
FOREVERBREAD FINALLYASIA
FOREVERBREAD FIFTYSTATE
FOREVERBREAD NEXTFOOL
FOREVERBREAD HOWEVERHEAT
```

#### Generate a haiku

```text
$ xpg haiku
Considerable / Gentleman Jerusalem / Colombia home
```

#### Generate a haiku with syllable pattern

```text
$ xpg haiku+
Bear mail farmers feet (1,1,2,1) / Therefore wild Arizona (2,1,4) / Information last (4,1)
```

#### Generate a condensed haiku with syllable pattern

```text
$ xpg haiku-
Considerable(5)/Therefore-interest-kitchen(2,3,2)/Electricity(5)
```

#### Generate a condensed haiku

```text
$ xpg haiku--
Considerable/Europe-arrive-coast-business/Lisbon-pull-suffix
```

#### Generate a haiku with newlines

```text
$ xpg haiku++
Pushed gentleman could
February Finland soft
Track gather game rise
```

#### Generate a haiku with syllable pattern and newlines

```text
$ xpg haiku+++
Considerable (5)
Electricity river (5,2)
Carefully told care (3,1,1)
```

#### Generate a haiku with extended words

```text
$ xpg -e haiku
North America / North Carolina months tried / Underline science
```

#### Generate a password with 20 *any* characters

```text
$ xpg aaaaaaaaaaaaaaaaaaaa
.l0L{vAKoe,9DPW;kLhs
```

#### Same as the previous example, but shorter

*This works because a character-only pattern is padded to the minimum length
with the last character.*

```text
$ xpg a -m 20
Ndo6)m7gzg{SZ/!B%%N=
```

#### Generate a password with 5 lowercase, 1 uppercase, 1 digit, and 1 symbol

```text
$ xpg cccccCds
yfqxbW5{
```

#### Generate a password with 5 lowercase, 1 uppercase, 1 digit, and 1 symbol and shuffle the characters

```text
$ xpg -s cccccCds
xrz1d=Qd
```

#### Same as the previous example, but shorter

*This works because a character-only pattern is padded to the minimum length
with the last character.*

```text
$ xpg -sm 8 Cdsc
v8e{saLz
```

#### Generate 3 passwords each from a different pattern

```text
$ xpg wwww WWWW TTTT
landsquaremayorentire
PARTSTARHEARSUCH
MoscowTriangleLateMouth
```

#### Apply upper casing

```text
$ xpg -A upper 'T T T T'
TOGETHER RHYTHM DREAM SWEET
```

#### Apply lower casing

```text
$ xpg -A lower 'T T T T'
eight team president might
```

#### Apply title casing

```text
$ xpg -A title 'T T T T'
Summer Shot Favor Bring
```

#### Apply toggle casing

```text
$ xpg -A toggle 'T T T T'
lEAVE tHINK sALT iCELAND
```

#### Apply camel casing

```text
$ xpg -A camel 'T T T T'
fruitIndianMilkFrom
```

#### Apply pascal casing

```text
$ xpg -A pascal 'T T T T'
DestroyFarmersEnoughSoldier
```

#### Apply upper-camel casing

```text
$ xpg -A upper-camel 'T T T T'
ColumnRaiseDenmarkTied
```

#### Apply snake casing

```text
$ xpg -A snake 'T T T T'
toward_opposite_machine_sister
```

#### Apply upper-snake casing

```text
$ xpg -A upper-snake 'T T T T'
PASSED_STRANGER_DANCE_SAND
```

#### Apply screaming-snake casing

```text
$ xpg -A screaming-snake 'T T T T'
SEVEN_BALL_APRIL_NOTE
```

#### Apply kebab casing

```text
$ xpg -A kebab 'T T T T'
quiet-hunger-suddenly-says
```

#### Apply cobol casing

```text
$ xpg -A cobol 'T T T T'
WHOM-INCREASE-INSIDE-SEVEN
```

#### Apply upper-kebab casing

```text
$ xpg -A upper-kebab 'T T T T'
ENGLISH-FORWARD-HUGE-STORE
```

#### Apply train casing

```text
$ xpg -A train 'T T T T'
Wings-Person-Listen-Girl
```

#### Apply flat casing

```text
$ xpg -A flat 'T T T T'
possiblestrengthpickedalways
```

#### Apply upper-flat casing

```text
$ xpg -A upper-flat 'T T T T'
COWSCURRENTEVENINGDOUBLE
```

#### Apply alternating casing

```text
$ xpg -A alternating 'T T T T'
wEsT pUbLiC lEaSt BrOuGhT
```

#### Apply random casing

```text
$ xpg -A random 'T T T T'
pRePARed sEll racE Mark
```

#### Apply pseudo-random casing

```text
$ xpg -A pseudo-random 'T T T T'
sIgHT ceNt ShOW maNNeR
```

#### Generate a password with words with 1, 2, 3, 4, and 5 syllables

```text
$ xpg '{a:1}-{W:a:2}-{T:a:3}-{olympian:4}-{T:ext:5}'
kind-WRITTEN-Industry-aphrodite-Philophrosyne
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

