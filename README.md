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
xkcd-style password generator

Usage: xpg [OPTIONS]

Options:
      --analyze            Analyze
  -w, --words <WORDS>      Number of words in password [default: 4]
  -c, --count <COUNT>      Number of passwords [default: 1]
  -d, --digits <DIGITS>    Append digits [default: 0]
  -s, --symbols <SYMBOLS>  Append symbols [default: 0]
  -h, --help               Print help information
  -V, --version            Print version information
~~~

~~~
$ xpg -V
xpg 0.3.1
~~~

### Examples

~~~
$ xpg
DutyNeptuneBillExpress
~~~

~~~
$ xpg -c 10
ControlPartialBesideCannot
LaughedPickNorthernVery
TemperatureStepFillFence
LaughterDressSleptVowel
SickMaterialSpokeRegion
EarthStandCoolLook
WonderCoveredLiveBusiness
RomeWhereDealFamily
ElementsStudyUnderlineTeach
LastAlongOrderlyFeel
~~~

~~~
$ xpg -w 8
GuessFrenchGreenDollarsEveningCommonCompanyMontana
~~~

~~~
$ xpg -w 8 -c 10
CaughtWereJapaneseSurpriseAmongKillContinueObject
SuddenlyIndiaSingleFireHeavenMarkAmsterdamEither
ThisProcessSandGermanyKillBroadWishBuild
JumpedDearDevelopedDirectionMachineInsideHoldCaptain
AppearDriveHappySuccessChileBankSpentMind
LearnTrackDelawareWishJudgeStreamSongGift
SorryStoodBelieveTestSmellValueBuiltLead
GuessSilverDiscoveredAfraidExperimentColdPrettyOther
WonderFreeSomeoneTailNovemberHangProbableExplain
ImportantHollandMisterSaltVenusVoiceWomanToday
~~~

~~~
$ xpg -w 2 -d 3
ListDollar001
~~~

~~~
$ xpg -w 2 -s 2
TailDuring-*
~~~

~~~
$ xpg -w 2 -d 1 -s 1
SinceKentucky1*
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

* 0.3.1 (2022-10-01)
    * fix readme
* 0.3.0 (2022-10-01)
    * update dependencies
    * add digits and symbols options
* 0.2.0 (2019-09-09)
    * expose `xpg!` macro
    * improve documentation
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

# Legal

```
Copyright 2019-2022 qtfkwk

Permission is hereby granted, free of charge, to any person obtaining a copy of 
this software and associated documentation files (the "Software"), to deal in 
the Software without restriction, including without limitation the rights to 
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies 
of the Software, and to permit persons to whom the Software is furnished to do 
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all 
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR 
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, 
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE 
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER 
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, 
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE 
SOFTWARE.
```

[MIT License](https://opensource.org/licenses/MIT)

# Tests

~~~
$ cargo test

running 11 tests
test tests::xpg_macro_cannot_return_zero_words - should panic ... ok
test tests::xpg_cannot_return_zero_words - should panic ... ok
test tests::xpg_macro_can_return_one_word ... ok
test tests::xpg_can_return_one_word ... ok
test tests::xpg_macro_can_return_four_words ... ok
test tests::xpg_can_return_three_words ... ok
test tests::xpg_can_return_four_words ... ok
test tests::default_xpg_macro_returns_four_words ... ok
test tests::xpg_macro_can_return_three_words ... ok
test tests::xpg_can_return_five_words ... ok
test tests::xpg_macro_can_return_five_words ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

~~~

