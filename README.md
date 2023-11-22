# About

xkcd-style password generator

![](t/fig/password_strength.png)

[xkcd #936: Password Strength](https://xkcd.com/936/)

# Usage

```text
$ xpg -h
xkcd-style password generator

Usage: xpg [OPTIONS]

Options:
      --analyze             Calculate total number of possible passwords for the
                            specified number of words
  -w, --words <NUMBER>      Number of words in password [default: 4]
  -c, --count <NUMBER>      Number of passwords [default: 1]
  -d, --digits <NUMBER>     Append digits [default: 0]
  -s, --symbols <NUMBER>    Append symbols [default: 0]
  -l, --lowercase <NUMBER>  Append lowercase letters [default: 0]
  -u, --uppercase <NUMBER>  Append uppercase letters [default: 0]
  -a, --any <NUMBER>        Append *any* characters (digits, symbols, lowercase,
                            uppercase) [default: 0]
      --keychain            Generate keychain-style password(s)
      --code-name           Generate code name(s)
  -h, --help                Print help
  -V, --version             Print version
```

```text
$ xpg -V
xpg 0.6.0
```

### Examples

```text
$ xpg
SpeedStrengthMajorMake
```

```text
$ xpg -c 10
ValleyWarmBeyondHappen
ThrewBecomeWereBecame
WelcomeGroundCattleYear
MostFingersWoreLocated
PatternDifferentContinuedDream
MarkWingsControlDried
ReceiveShownTradeChoose
DeepGlassIndianFelt
HoursWireContinuePretty
CriedRestBelowReply
```

```text
$ xpg -w 8
PlayTheyPromiseDriedNorwayInsectsDrawJust
```

```text
$ xpg -w 8 -c 10
DistantTriedEggsDetailsCentOpenShirtDecember
LisbonSharpSomeonePaperYearThoseWinterWhat
PortugalWhileRollNothingHappyLaterJordanVerb
DestroyGuardStoneIndustryWalkThreeVenusElements
AmericaWarmBlackParisIndeedAnswerBreadAction
DailyNameThusToreFightAgreeHeartKitchen
DoorBarbadosFruitStateWiseCheckComeKilled
WordAustriaCigarettePerfectOsloInchesSteelAfter
TuesdayPolandHighPeaceForwardDishEightDifficult
WingDoesKnewSaltCopyDistanceCropsExercise
```

```text
$ xpg -w 2 -d 3
PriceGlass826
```

```text
$ xpg -w 2 -s 2
TruckDrop%{
```

```text
$ xpg -w 2 -d 1 -s 1
SpotClean5<
```

```text
$ xpg -l 2
SeedServiceLearnEachiq
```

```text
$ xpg -u 2
GatherFigureWhileListXL
```

```text
$ xpg -a 5
ToneBrokenHangGiftcddZ1
```

```text
$ xpg -w 0 -a 20
xWTln$)(0^*T^=]zz*ZG
```

```text
$ xpg --keychain
1afjvh-vyaJug-whmmwt
```

```text
$ xpg --code-name
PLACE PLUTO
```

```text
$ xpg --analyze
* Word list length: 1,259
* Words in password: 4
* Total permutations (without repetition): 2,500,525,503,024

    ```
    1,259! / (1,259 - 4)!
    1,259! / 1,255!
    2,500,525,503,024
    ```

Words | Permutations
---|---:
1 | 1,259
2 | 1,583,822
3 | 1,990,864,254
4 | 2,500,525,503,024
5 | 3,138,159,506,295,120
6 | 3,935,252,020,894,080,480
7 | 4,930,870,782,180,282,841,440
8 | 6,173,450,219,289,714,117,482,880
... | ...

```

```text
$ xpg --analyze -w 8
* Word list length: 1,259
* Words in password: 8
* Total permutations (without repetition): 6,173,450,219,289,714,117,482,880

    ```
    1,259! / (1,259 - 8)!
    1,259! / 1,251!
    6,173,450,219,289,714,117,482,880
    ```

Words | Permutations
---|---:
1 | 1,259
2 | 1,583,822
3 | 1,990,864,254
4 | 2,500,525,503,024
5 | 3,138,159,506,295,120
6 | 3,935,252,020,894,080,480
7 | 4,930,870,782,180,282,841,440
8 | 6,173,450,219,289,714,117,482,880
... | ...

```

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

# References

* Word list comes from Bart Busschots'
  [HSXKPasswd](https://www.bartbusschots.ie/s/publications/software/xkpasswd/)
  Perl module ([GitHub](https://github.com/bbusschots/hsxkpasswd),
  [CPAN: Crypt::HSXKPasswd](http://search.cpan.org/perldoc?Crypt%3A%3AHSXKPasswd)),
  specifically
  [lib/Crypt/HSXKPasswd/Dictionary/EN.pm@1d88564:38](https://github.com/bbusschots/hsxkpasswd/blob/1d88564d5bf74cf48025b372bcb635fc022962dd/lib/Crypt/HSXKPasswd/Dictionary/EN.pm#L38)

