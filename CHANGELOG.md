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
* 0.8.0 (2023-11-25): remove benchmark; simplify cli options; add `-r` and `-C`
  options; restore `-c 0` functionality; add `codename-series` special pattern;
  add word kind subpatterns; enable external runtime configuration via new
  `Config` struct
* 0.8.1 (2023-11-25): fix `-C` option
* 0.9.0 (2023-11-26): add extended words; enable multiple patterns; add `-e` and
  `-L` options; make `-d` dump the loaded not default configuration
* 0.10.0 (2023-11-27): add syllables and `haiku*` special patterns; fix the
  codename series algorithm; make olympian/chthonic kinds extended; update
  dependencies
* 0.11.0 (2023-11-28): add bat pager on \*nix
* 0.12.0 (2023-11-29): fix haiku algorithm; updated dependencies
* 0.13.0 (2023-12-01): add/fix months; fix dump; update dependencies

