# About

xkcd-style password generator

![](t/fig/password_strength.png)

[xkcd #936: Password Strength](https://xkcd.com/936/)

# Usage

```text
$ xpg -h
xpg 0.19.3 <https://crates.io/crates/xpg>

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
xpg 0.19.3
```

*See also the [Configuration] section below.*

[Configuration]: #configuration

### Examples

```text
$ xpg
riverfieldhalfentered
```

#### Generate 10 passwords

```text
$ xpg -c 10
mercurycaliforniabeautifulminutes
shapepassedcornervalley
probablyaustraliawallentered
helparizonaathensgibraltar
valleysaysleadershown
characteramericawildiceland
destroysecondmatterlength
mountainsettledsuggestedbulgaria
fulldifferentsharpblock
surpriseeuropedailyarrive
```

#### Generate passwords forever

*or until Ctrl+C / SIGINT*

```text
$ xpg -c 0
shoutedmadridwheelsright
bothsoutherntablelike
copenhagenstudentsformfriday
belfastironalongalaska
gladhumanwestsave
airplanebellmelodyview
requirepleasantheightwagon
jerusalemwinterwithoutcharge
characterexceptlanguagelone
trustdeviceparticularbottom
^C
```

#### Generate a password with 8 words

```text
$ xpg wwwwwwww
blowquietresultsmiledcropsconditionfijianything
```

#### Generate 10 passwords with 8 words

```text
$ xpg wwwwwwww -c 10
bringplanetlovetouchfinishedcaughtreplydestroy
pastangerneedledivisionguessperiodhangnoon
firmdesignnothingpreparedseenthoughdarestretched
acrossnothingaustraliapresidentsrealizewaterenjoyprinted
trainbasefinishedspeakdiscoverfingersyourselfcause
nieceenjoysufferdrivealongwillmexicolord
amongtailairplaneohiofamousbecamemeanbird
fortyratherneedmarryhuntingjoinedgifttries
cornervirginiareadysoilspaceladdershipthrow
colombiaarrivedmannerflowstoretakendivisionfinally
```

#### Generate a password with 2 words followed by 3 digits

```text
$ xpg wwddd
fromwhich929
```

#### Generate a password with 2 words followed by 2 symbols

```text
$ xpg wwss
weeksince}}
```

#### Generate a password with 2 words followed by a digit and a symbol

```text
$ xpg wwds
criedfinally3]
```

#### Generate a password with 4 words followed by 2 lowercase letters

```text
$ xpg wwwwcc
wiseladderamountaroundts
```

#### Generate a password with 4 words followed by 2 uppercase letters

```text
$ xpg wwwwCC
aloneshownsomeplantFQ
```

#### Generate a password with 4 title case words

```text
$ xpg TTTT
FinlandChildForeignTokyo
```

#### Generate a password with 4 uppercase words

```text
$ xpg WWWW
POSITIONNIGHTRUSHGIRL
```

#### Generate a password with 4 words followed by 5 *any* characters

*An "any" character can be a lowercase letter, uppercase letter, digit, or
symbol.*

```text
$ xpg wwwwaaaaa
yearworkersitselfathensO/LsK
```

#### Generate a password with 3 words and at least 15 characters

```text
$ xpg www -m 15
successbackform
```

#### Generate a password with 3 words and no more than 20 characters

```text
$ xpg www -M 20
horsewrotecuba
```

#### Generate a password with 4 words and between 20 and 25 characters

```text
$ xpg www -m 20 -M 25 -a 1000
clothesthousandfebruary
```

*See also [Usage] note 2.*

[Usage]: #usage

#### Generate a password with 2 words, 3 digits, 1 symbol, and exactly 16 characters

```text
$ xpg wwddds -l 16 -a 1000
shoulderfeet065]
```

*See also [Usage] note 2.*

#### Generate a password with a `1`, 2 title case words, and `!`

```text
$ xpg '1TT!'
1BeingPulled!
```

#### Generate a password with 1 symbol, 2 adjectives, 1 noun, and 2 digits

```text
$ xpg 's{adj}{adj}{n}dd'
,tiedgoodpast81
```

#### Generate a password with 2 words and 1 extended word

```text
$ xpg 'ww{ext}'
shoutedhawaiijanuary
```

#### Generate a password with 2 words and 1 title case extended word

```text
$ xpg 'ww{T:ext}'
frenchforgetHera
```

#### Generate a password with 2 words and 1 uppercase extended word

```text
$ xpg 'ww{W:ext}'
waveswantsERSA
```

#### Generate 10 passwords with 4 title case words and extended words merged

```text
$ xpg -ec 10 TTTT
EastMarchGiveEntire
StepMaltaItalyStretched
IndianaSingaporeSouthcarolinaDiscovered
LegsThyoneWantsIo
FractionLoneKilledBody
AitnePressLisbonListen
ResentFloorDrawingHonor
LegsWeatherFebruaryGoodbye
UmbrielNoseDetailsBread
LargePennsylvaniaStraightOregon
```

#### Generate a keychain-style password

```text
$ xpg keychain
beDshe-ueuila-bsx3if
```

#### Generate a code name

```text
$ xpg codename
LIKELY ABOVE
```

#### Generate a code name series

```text
$ xpg codename-series
LARGESHARE BURNINGWALK
LARGESHARE CAPITALLIGHT
LARGESHARE DOWNWHEEL
LARGESHARE SPECIALPLANT
LARGESHARE THREEQUESTIONS
LARGESHARE MOREFACE
LARGESHARE DESERTWOOD
LARGESHARE EASTDIRECTION
LARGESHARE CLEANOUTSIDE
LARGESHARE WHOSESATURN
```

#### Generate a code name series with 20 code names

```text
$ xpg codename-series -c 20
TIEDMIND SAFEDEGREE
TIEDMIND USUALAFRICA
TIEDMIND POSSIBLESTILL
TIEDMIND CONSONANTDRINK
TIEDMIND HIGHWANT
TIEDMIND LEVELGROUND
TIEDMIND NEVERAREA
TIEDMIND USUALLYCOTTON
TIEDMIND FAMOUSFIRE
TIEDMIND WHOSEFIFTEEN
TIEDMIND POORFALL
TIEDMIND HALFEXPERIENCE
TIEDMIND PRETTYRETURN
TIEDMIND YOUNGMUST
TIEDMIND FOREVERROSE
TIEDMIND SURECHILDREN
TIEDMIND TODAYYELLOW
TIEDMIND DEEPMETHOD
TIEDMIND PROUDPARTY
TIEDMIND LASTSUIT
```

#### Generate a haiku

```text
$ xpg haiku
Etching next shine class / Money crowd underline team / Nevada enough
```

#### Generate a haiku with syllable pattern

```text
$ xpg haiku+
Electricity (5) / President condition clear (3,3,1) / Include complete cold (2,2,1)
```

#### Generate a condensed haiku with syllable pattern

```text
$ xpg haiku-
Considerable(5)/Electricity-nature(5,2)/Information-slept(4,1)
```

#### Generate a condensed haiku

```text
$ xpg haiku--
Melody-pleasure/Considerable-world-lake/Actually-labor
```

#### Generate a haiku with newlines

```text
$ xpg haiku++
Anything smoke shore
Electricity inside
Character honor
```

#### Generate a haiku with syllable pattern and newlines

```text
$ xpg haiku+++
Malta clean rest night (2,1,1,1)
Engine Jerusalem sharp (2,4,1)
Considerable (5)
```

#### Generate a haiku with extended words

```text
$ xpg -e haiku
Philophrosyne / February whose first page / Hegemone ball
```

#### Generate a password with 20 *any* characters

```text
$ xpg aaaaaaaaaaaaaaaaaaaa
c8Rt%{!-<m4o{>,b6lNa
```

#### Same as the previous example, but shorter

*This works because a character-only pattern is padded to the minimum length
with the last character.*

```text
$ xpg a -m 20
pD-/~z24S;)~ULsrI<RE
```

#### Generate a password with 5 lowercase, 1 uppercase, 1 digit, and 1 symbol

```text
$ xpg cccccCds
sbvzxU2^
```

#### Generate a password with 5 lowercase, 1 uppercase, 1 digit, and 1 symbol and shuffle the characters

```text
$ xpg -s cccccCds
~wqwV0wm
```

#### Same as the previous example, but shorter

*This works because a character-only pattern is padded to the minimum length
with the last character.*

```text
$ xpg -sm 8 Cdsc
g9?fuxxA
```

#### Generate 3 passwords each from a different pattern

```text
$ xpg wwww WWWW TTTT
rootsendsnowmercury
CENTSKNOWBELLFENCE
AthensMovementLeaveLeast
```

#### Apply upper casing

```text
$ xpg -A upper 'T T T T'
DATE BOTSWANA HOLE CAUSE
```

#### Apply lower casing

```text
$ xpg -A lower 'T T T T'
government increase reason sent
```

#### Apply title casing

```text
$ xpg -A title 'T T T T'
Tied Grew Conditions Short
```

#### Apply toggle casing

```text
$ xpg -A toggle 'T T T T'
bESIDE sHARP iNSECTS hOME
```

#### Apply camel casing

```text
$ xpg -A camel 'T T T T'
allowThirdRoundTuesday
```

#### Apply pascal casing

```text
$ xpg -A pascal 'T T T T'
MiddleBridgeFlierToday
```

#### Apply upper-camel casing

```text
$ xpg -A upper-camel 'T T T T'
PushAfterEgyptUntil
```

#### Apply snake casing

```text
$ xpg -A snake 'T T T T'
government_trouble_made_october
```

#### Apply upper-snake casing

```text
$ xpg -A upper-snake 'T T T T'
LARGE_FORTY_SHAKE_BASE
```

#### Apply screaming-snake casing

```text
$ xpg -A screaming-snake 'T T T T'
OPPOSITE_WEATHER_CLOUD_ROCK
```

#### Apply kebab casing

```text
$ xpg -A kebab 'T T T T'
catch-lord-suggested-lisbon
```

#### Apply cobol casing

```text
$ xpg -A cobol 'T T T T'
THEREFORE-STAND-INCH-PROUD
```

#### Apply upper-kebab casing

```text
$ xpg -A upper-kebab 'T T T T'
HURRY-MARRY-DRIVE-LENGTH
```

#### Apply train casing

```text
$ xpg -A train 'T T T T'
Halt-Straight-Goodbye-Fact
```

#### Apply flat casing

```text
$ xpg -A flat 'T T T T'
destroygrownbreadreach
```

#### Apply upper-flat casing

```text
$ xpg -A upper-flat 'T T T T'
OPINIONENGLANDSEEDSCROWD
```

#### Apply alternating casing

```text
$ xpg -A alternating 'T T T T'
sUiT eQuAl TrIeD hElP
```

#### Apply random casing

```text
$ xpg -A random 'T T T T'
stick puSH placE BaskET
```

#### Apply pseudo-random casing

```text
$ xpg -A pseudo-random 'T T T T'
tOdAy BuILdInG tRiaNGlE pReParED
```

#### Generate a password with words with 1, 2, 3, 4, and 5 syllables

```text
$ xpg '{a:1}-{W:a:2}-{T:a:3}-{olympian:4}-{T:ext:5}'
cloth-UNIT-Require-aphrodite-Northcarolina
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

