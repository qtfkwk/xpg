# About

xkcd-style password generator

# Features

* Balanced code verbosity, comments, documentation, tests, benchmarks, external
  dependencies
* CLI utility
* Rust library, function, macro

# Usage

## CLI

~~~
$ xpg -h
xpg 0.1.2
xkcd-style password generator

USAGE:
    xpg [FLAGS] [OPTIONS]

FLAGS:
        --analyze    Analyze
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --count <count>    Number of passwords; default: 1
    -w, --words <words>    Number of words in password; default: 4
~~~

~~~
$ xpg -V
xpg 0.1.2
~~~

### Examples

~~~
$ xpg
PeaceFirmWerePower
~~~

~~~
$ xpg -c 10
ColorSpringRingHall
AnotherCentLendDelaware
SureEveryTriedVowel
ConsiderLearnSendWave
JerusalemJoinRichHole
FarmExactlyStationGrew
DecimalForcePresidentsLater
EnjoyAppearGrownWood
OftenShallDirectionCourse
MayorGreeceCongoDecimal
~~~

~~~
$ xpg -w 8
SpeakAlongLaborGrassNailBattlePositionReach
~~~

~~~
$ xpg -w 8 -c 10
LostButterFellowWouldJoinedOfferEggsShine
StreamFlowYardCountGeneralEffectSecondLearn
SpeedGermanyCouldThinkMondayKissStrikeOctober
DearStationWentLikeAfricaPerhapsBeenWomen
CongoAgainstPlantEarlyGonePracticeAlabamaSail
HeavenUncleSuggestedNoiseBurningPassedEtchingTrust
FightColombiaJapanWillJoinedAgreedSentLabor
RiddenReachSubjectEnemyAfraidGroupCompanySuppose
NoneHouseBonesPastDressEffortBoatFellow
NailIrelandTestCowsScoreLikelyWheatWorth
~~~

~~~
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

~~~

~~~
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

~~~

# Changelog

* 0.1.2 (2019-09-08)
    * minor fixes
* 0.1.1 (2019-09-08)
    * minor fixes
* 0.1.0 (2019-09-08)
    * store wordlist as constant and operate on it without copying
    * select N words without repetition via `choose_multiple`
    * tests
    * benchmark various implementations
    * `xpg` function
    * `xpg!` macro to enable optional word count argument
    * command line interface via clap

# References

* [xkcd #936: Password Strength](https://xkcd.com/936/)

  ![](fig/password_strength.png)

* Word list comes from Bart Busschots'
  [HSXKPasswd](https://www.bartbusschots.ie/s/publications/software/xkpasswd/)
  Perl module ([GitHub](https://github.com/bbusschots/hsxkpasswd),
  [CPAN: Crypt::HSXKPasswd](http://search.cpan.org/perldoc?Crypt%3A%3AHSXKPasswd)),
  specifically
  [lib/Crypt/HSXKPasswd/Dictionary/EN.pm@1d88564:38](https://github.com/bbusschots/hsxkpasswd/blob/1d88564d5bf74cf48025b372bcb635fc022962dd/lib/Crypt/HSXKPasswd/Dictionary/EN.pm#L38)

# Tests

~~~
$ cargo test

running 11 tests
test tests::xpg_can_return_one_word ... ok
test tests::xpg_can_return_four_words ... ok
test tests::default_xpg_macro_returns_four_words ... ok
test tests::xpg_can_return_five_words ... ok
test tests::xpg_cannot_return_zero_words ... ok
test tests::xpg_can_return_three_words ... ok
test tests::xpg_macro_can_return_five_words ... ok
test tests::xpg_macro_cannot_return_zero_words ... ok
test tests::xpg_macro_can_return_four_words ... ok
test tests::xpg_macro_can_return_one_word ... ok
test tests::xpg_macro_can_return_three_words ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

~~~

# Benchmarks

Benchmark | time
---|---
1 | 54.084 us 54.232 us 54.383 us
2 | 191.25 ns 191.64 ns 192.26 ns
3 | 190.73 ns 190.86 ns 191.03 ns

## Benchmark 1

[src/lib.rs:1374](src/lib.rs#L1374)

~~~rust
let mut wordlist: Vec<String> = Vec::new();
for word in &WORDLIST[..] {
    wordlist.push(word.to_string());
}
let p: Vec<String> = wordlist.choose_multiple(&mut thread_rng(), words)
    .cloned()
    .collect();
return p.join("");
~~~

[benches/xpg.rs.1.log](benches/xpg.rs.1.log)

~~~

running 6 tests
test tests::default_xpg_returns_four_words ... ignored
test tests::xpg_can_return_five_words ... ignored
test tests::xpg_can_return_four_words ... ignored
test tests::xpg_can_return_one_word ... ignored
test tests::xpg_can_return_three_words ... ignored
test tests::xpg_cannot_return_zero_words ... ignored

test result: ok. 0 passed; 0 failed; 6 ignored; 0 measured; 0 filtered out


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Benchmarking xpg
Benchmarking xpg: Warming up for 3.0000 s
Benchmarking xpg: Collecting 100 samples in estimated 5.1942 s (96k iterations)
Benchmarking xpg: Analyzing
xpg                     time:   [54.084 us 54.232 us 54.383 us]
Found 18 outliers among 100 measurements (18.00%)
  1 (1.00%) low severe
  16 (16.00%) high mild
  1 (1.00%) high severe

~~~

## Benchmark 2

[src/lib.rs:1384](src/lib.rs#L1384)

~~~rust
let p: Vec<&str> = WORDLIST.choose_multiple(&mut thread_rng(), words)
    .cloned().collect::<Vec<&str>>();
return p.join("");
~~~

[benches/xpg.rs.2.log](benches/xpg.rs.2.log)

~~~

running 6 tests
test tests::default_xpg_returns_four_words ... ignored
test tests::xpg_can_return_five_words ... ignored
test tests::xpg_can_return_four_words ... ignored
test tests::xpg_can_return_one_word ... ignored
test tests::xpg_can_return_three_words ... ignored
test tests::xpg_cannot_return_zero_words ... ignored

test result: ok. 0 passed; 0 failed; 6 ignored; 0 measured; 0 filtered out


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Benchmarking xpg
Benchmarking xpg: Warming up for 3.0000 s
Benchmarking xpg: Collecting 100 samples in estimated 5.0002 s (24M iterations)
Benchmarking xpg: Analyzing
xpg                     time:   [191.25 ns 191.64 ns 192.26 ns]
                        change: [-99.649% -99.648% -99.646%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

~~~

## Benchmark 3

[src/lib.rs:1390](src/lib.rs#L1390)

~~~rust
WORDLIST.choose_multiple(&mut thread_rng(), words)
    .cloned().collect::<Vec<&str>>().join("")
~~~

[benches/xpg.rs.3.log](benches/xpg.rs.3.log)

~~~

running 6 tests
test tests::default_xpg_returns_four_words ... ignored
test tests::xpg_can_return_five_words ... ignored
test tests::xpg_can_return_four_words ... ignored
test tests::xpg_can_return_one_word ... ignored
test tests::xpg_can_return_three_words ... ignored
test tests::xpg_cannot_return_zero_words ... ignored

test result: ok. 0 passed; 0 failed; 6 ignored; 0 measured; 0 filtered out


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Benchmarking xpg
Benchmarking xpg: Warming up for 3.0000 s
Benchmarking xpg: Collecting 100 samples in estimated 5.0003 s (25M iterations)
Benchmarking xpg: Analyzing
xpg                     time:   [190.73 ns 190.86 ns 191.03 ns]
                        change: [-0.5139% -0.1940% +0.1608%] (p = 0.27 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe

~~~

