# Changelog

* 0.1.0 (2019-09-08): Store wordlist as constant and operate on it without copying; select N words without repetition via `choose_multiple`; tests; benchmark various implementations; `xpg` function; `xpg!` macro to enable optional word count argument; command line interface via clap
    * 0.1.1 (2019-09-08): Minor fixes
    * 0.1.2 (2019-09-08): Minor fixes
* 0.2.0 (2019-09-09): Expose `xpg!` macro; improve documentation
* 0.3.0 (2022-10-01): Update dependencies; add digits and symbols options
    * 0.3.1 (2022-10-01): Fix readme
* 0.4.0 (2022-10-02): Add --lowercase, --uppercase, and --any options; enable --words option to be zero if --any option is greater than zero; enable infinite passwords via `-c 0`
* 0.5.0 (2022-10-13): Add --keychain and --code-name options
* 0.6.0 (2023-11-22): Remove num and factorial crate dependencies; improve permutations function; update dependencies; apply clippy fixes and cargo fmt; reorganize
    * 0.6.1 (2023-11-22): Fix readme
* 0.7.0 (2023-11-24): Complete redesign
* 0.8.0 (2023-11-25): Remove benchmark; simplify cli options; add `-r` and `-C` options; restore `-c 0` functionality; add `codename-series` special pattern; add word kind subpatterns; enable external runtime configuration via new `Config` struct
    * 0.8.1 (2023-11-25): Fix `-C` option
* 0.9.0 (2023-11-26): Add extended words; enable multiple patterns; add `-e` and `-L` options; make `-d` dump the loaded not default configuration
* 0.10.0 (2023-11-27): Add syllables and `haiku*` special patterns; fix the codename series algorithm; make olympian/chthonic kinds extended; update dependencies
* 0.11.0 (2023-11-28): Add bat pager on \*nix
* 0.12.0 (2023-11-29): Fix haiku algorithm; updated dependencies
* 0.13.0 (2023-12-01): Add/fix months; fix dump; update dependencies
* 0.14.0 (2023-12-11): Implement Default for Config; updated dependencies
    * 0.14.1 (2024-04-04): Update dependencies; fix changelog
* 0.15.0 (2024-07-30): Fix makefile; update dependencies
    * 0.15.1 (2024-08-22): Fix changelog; fix readme; add `commit` target to makefile; update dependencies
* 0.16.0 (2024-08-25): Add `-A` / `--apply-case` option
* 0.17.0 (2024-08-25): Clean up code; add descriptive haiku variants; nuke github actions configuration; update dependencies
    * 0.17.1 (2024-08-25): Fix readme; fix clippy
    * 0.17.2 (2024-10-25): Add clap color; update dependencies
    * 0.17.3 (2024-12-04): Update dependencies
* 0.18.0 (2025-01-07): Add `-s` / `--shuffle` option; expose `shuffle` function; update dependencies

