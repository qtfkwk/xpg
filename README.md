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
  -p, --pattern <PATTERN>  Password pattern (`W`: WORD, `w`: word, `T`: Word,
                           `k`: `shuffle(cccc(c|d)(C|c))` `C`: A-Z, `c`: a-z,
                           `d`: 0-9, `s`: `~!@#$%^&*-_=+;:,./?()[]{}<>`, `a`:
                           `C|c|d|s`) [default: wwww]
      --keychain           Generate keychain-style password(s); equivalent to
                           `-p 'k-k-k'`
      --code-name          Generate code name(s); equivalent to `-p 'W W'`
  -c, --count <NUMBER>     Number of passwords [default: 1]
      --length <NUMBER>    Length
      --minimum <NUMBER>   Minimum length
      --maximum <NUMBER>   Maximum length
      --attempts <NUMBER>  Attempts [default: 10]
  -h, --help               Print help
  -V, --version            Print version
```

```text
$ xpg -V
xpg 0.7.0
```

### Examples

```text
$ xpg
monthsuccessmisswelcome
```

```text
$ xpg -c 10
grassamericaunderlinestranger
periodsinglethrowpeace
shoredeepfoolturn
intolastusuallyhurt
gentlehusbandcoatdetails
increaseherefoodtogether
heavythrowngreatcapital
passeddividemodernquite
readthoughtclockforget
realizeladychildrenstand
```

```text
$ xpg -p wwwwwwww
saturdaypushgibraltarhourreportmouthdistantcarry
```

```text
$ xpg -p wwwwwwww -c 10
romeropewriteyardshoesexampleringmaterial
sceneperugiftoverrollsingpasslast
niececouldaprilfinlandbrothercharacterwillthick
dollarsfinishedalthoughchancedinnerbankerhersmorning
usuallylevelfridayafraidlistplainplanewore
repeatednieceeffortobservevaluetherediscoverring
certainworkerswithinslowlynotelanguageescapewrite
staymachinequestionscakepresidenthopedesertdelight
escapemonthpushstarsegyptsmokewhilesettled
lightsubstancesalthoughbelowamongrockclockpark
```

```text
$ xpg -p wwddd
manylast811
```

```text
$ xpg -p wwss
ableright]{
```

```text
$ xpg -p wwds
musicenergy2(
```

```text
$ xpg -p wwwwcc
whilelakegeneralfrenchsf
```

```text
$ xpg -p wwwwCC
theycamestudystreamUL
```

```text
$ xpg -p TTTT
StrongFlowersLargeAsia
```

```text
$ xpg -p WWWW
SIZESENSENECKHEAR
```

```text
$ xpg -p wwwwaaaaa
everygivesmarchcoatuuopw
```

```text
$ xpg -p aaaaaaaaaaaaaaaaaaaa
sY@UHF1FoV~$FIoNp,pC
```

```text
$ xpg --keychain
mx7myy-ghbanl-mhLalq
```

```text
$ xpg --code-name
FRESH HISTORY
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
* 0.6.1 (2023-11-22): fix readme
* 0.7.0 (2023-11-24): complete redesign

# References

* Word list originates from Bart Busschots'
  [HSXKPasswd](https://www.bartbusschots.ie/s/publications/software/xkpasswd/)
  Perl module ([GitHub](https://github.com/bbusschots/hsxkpasswd),
  [CPAN: Crypt::HSXKPasswd](http://search.cpan.org/perldoc?Crypt%3A%3AHSXKPasswd)),
  specifically
  [lib/Crypt/HSXKPasswd/Dictionary/EN.pm@1d88564:38](https://github.com/bbusschots/hsxkpasswd/blob/1d88564d5bf74cf48025b372bcb635fc022962dd/lib/Crypt/HSXKPasswd/Dictionary/EN.pm#L38)

