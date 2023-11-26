/*!

xkcd-style password generator

*/

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use rand::{seq::SliceRandom, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use ucfirst::ucfirst;

//--------------------------------------------------------------------------------------------------

pub const CONFIG: &str = include_str!("../config.json");

lazy_static! {
    pub static ref WORD_KIND_SUBS: HashMap<String, WordKind> = WORD_KINDS_ALL
        .iter()
        .flat_map(|x| {
            let sub = x.sub();
            [
                (sub.clone(), *x),
                (format!("{{W:{}", &sub[1..]), *x),
                (format!("{{T:{}", &sub[1..]), *x),
            ]
        })
        .collect();
}

//--------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum WordKind {
    Astronomy,
    Planet,
    Moon,
    MarsMoon,
    JupiterMoon,
    SaturnMoon,
    UranusMoon,
    NeptuneMoon,
    Continent,
    Country,
    UsState,
    City,
    Place,
    Nationality,
    MaleName,
    FemaleName,
    Name,
    Element,
    Mythology,
    RomanMyth,
    GreekMyth,
    Olympian,
    Chthonic,
    Month,
    Day,
    Color,
    Noun,
    Pronoun,
    ProperNoun,
    PluralNoun,
    SingularNoun,
    Adjective,
    Adverb,
    Verb,
    IntransitiveVerb,
    TransitiveVerb,
    AuxiliaryVerb,
    VerbPast,
    Preposition,
    Conjunction,
    Interjection,
    All,
    Extended,
    AllExtended,
}

use WordKind::*;

const WORD_KINDS_ALL: [WordKind; 44] = [
    Astronomy,
    Planet,
    Moon,
    MarsMoon,
    JupiterMoon,
    SaturnMoon,
    UranusMoon,
    NeptuneMoon,
    Continent,
    Country,
    UsState,
    City,
    Place,
    Nationality,
    MaleName,
    FemaleName,
    Name,
    Element,
    Mythology,
    RomanMyth,
    GreekMyth,
    Olympian,
    Chthonic,
    Month,
    Day,
    Color,
    Noun,
    Pronoun,
    ProperNoun,
    PluralNoun,
    SingularNoun,
    Adjective,
    Adverb,
    Verb,
    IntransitiveVerb,
    TransitiveVerb,
    AuxiliaryVerb,
    VerbPast,
    Preposition,
    Conjunction,
    Interjection,
    All,
    Extended,
    AllExtended,
];

const WORD_KINDS_EXTENDED: [WordKind; 7] = [
    MarsMoon,
    JupiterMoon,
    SaturnMoon,
    UranusMoon,
    NeptuneMoon,
    Extended,
    AllExtended,
];

impl WordKind {
    fn enumerate(&self, extended: bool) -> Vec<WordKind> {
        let mut r = vec![*self];
        match self {
            Astronomy => r.append(&mut vec![SingularNoun, Noun]),
            Planet => r.append(&mut vec![Astronomy, SingularNoun, Noun]),
            Moon => r.append(&mut vec![Astronomy, SingularNoun, Noun]),
            MarsMoon => r.append(&mut vec![Moon, Astronomy, SingularNoun, Noun, Extended]),
            JupiterMoon => r.append(&mut vec![Moon, Astronomy, SingularNoun, Noun, Extended]),
            SaturnMoon => r.append(&mut vec![Moon, Astronomy, SingularNoun, Noun, Extended]),
            UranusMoon => r.append(&mut vec![Moon, Astronomy, SingularNoun, Noun, Extended]),
            NeptuneMoon => r.append(&mut vec![Moon, Astronomy, SingularNoun, Noun, Extended]),
            Continent => r.append(&mut vec![Place, ProperNoun, SingularNoun, Noun]),
            Country => r.append(&mut vec![Place, ProperNoun]),
            UsState => r.append(&mut vec![Place, ProperNoun]),
            City => r.append(&mut vec![Place, ProperNoun]),
            Nationality => r.push(Adjective),
            MaleName => r.append(&mut vec![Name, ProperNoun]),
            FemaleName => r.append(&mut vec![Name, ProperNoun]),
            Name => r.push(ProperNoun),
            Element => r.append(&mut vec![ProperNoun, Noun]),
            Mythology => r.append(&mut vec![ProperNoun, Noun]),
            RomanMyth => r.append(&mut vec![Mythology, ProperNoun, Noun]),
            GreekMyth => r.append(&mut vec![Mythology, ProperNoun, Noun]),
            Olympian => r.append(&mut vec![GreekMyth, ProperNoun, Noun]),
            Chthonic => r.append(&mut vec![GreekMyth, ProperNoun, Noun]),
            Month => r.append(&mut vec![SingularNoun, Noun]),
            Day => r.append(&mut vec![SingularNoun, Noun]),
            Color => r.append(&mut vec![SingularNoun, Noun, Verb]),
            IntransitiveVerb => r.push(Verb),
            TransitiveVerb => r.push(Verb),
            AuxiliaryVerb => r.push(Verb),
            VerbPast => r.push(Verb),
            PluralNoun => r.push(Noun),
            SingularNoun => r.push(Noun),
            _ => {}
        }
        if extended || !r.contains(&Extended) {
            r.push(All);
        }
        r.push(AllExtended);
        r
    }

    fn sub(&self) -> String {
        match self {
            Adjective => "{adj}",
            Adverb => "{adv}",
            All => "{a}",
            AllExtended => "{a.ext}",
            Astronomy => "{ast}",
            AuxiliaryVerb => "{v.aux}",
            Chthonic => "{chthonic}",
            City => "{city}",
            Color => "{color}",
            Conjunction => "{conj}",
            Continent => "{cont}",
            Country => "{country}",
            Day => "{day}",
            Element => "{el}",
            Extended => "{ext}",
            FemaleName => "{fname}",
            GreekMyth => "{greekmyth}",
            Interjection => "{i}",
            IntransitiveVerb => "{v.int}",
            JupiterMoon => "{jupitermoon}",
            MaleName => "{mname}",
            MarsMoon => "{marsmoon}",
            Month => "{mon}",
            Moon => "{moon}",
            Mythology => "{myth}",
            Name => "{name}",
            Nationality => "{nat}",
            NeptuneMoon => "{neptunemoon}",
            Noun => "{n}",
            Olympian => "{olympian}",
            Place => "{place}",
            Planet => "{planet}",
            PluralNoun => "{n.pl}",
            Preposition => "{prep}",
            Pronoun => "{n.pro}",
            ProperNoun => "{n.prop}",
            RomanMyth => "{romanmyth}",
            SaturnMoon => "{saturnmoon}",
            SingularNoun => "{n.s}",
            TransitiveVerb => "{v.tr}",
            UranusMoon => "{uranusmoon}",
            UsState => "{us-state}",
            Verb => "{v}",
            VerbPast => "{v.past}",
        }
        .to_string()
    }
}

//--------------------------------------------------------------------------------------------------

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    characters: BTreeMap<char, String>,
    words: BTreeMap<String, Vec<WordKind>>,

    #[serde(skip)]
    kinds: HashMap<WordKind, Vec<String>>,

    #[serde(skip)]
    alphabets: BTreeMap<char, Vec<char>>,

    #[serde(skip)]
    pub extended: bool,
}

impl Config {
    pub fn from_path(path: &Path, extended: bool) -> Result<Config> {
        let r: Config = serde_json::from_reader(BufReader::new(File::open(path)?))?;
        Ok(r.build(extended))
    }

    pub fn from_str(s: &str, extended: bool) -> Result<Config> {
        Ok(serde_json::from_str::<Config>(s)?.build(extended))
    }

    pub fn dump(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    fn build(mut self, extended: bool) -> Config {
        // Build word lists for each kind
        let mut kinds: HashMap<WordKind, BTreeSet<String>> = WORD_KINDS_ALL
            .iter()
            .map(|x| (*x, BTreeSet::new()))
            .collect();
        for (word, word_kinds) in &self.words {
            let word_kinds = word_kinds
                .iter()
                .flat_map(|x| x.enumerate(extended))
                .collect::<Vec<_>>();
            let word_is_extended = word_kinds.contains(&Extended);
            for kind in word_kinds {
                let kind_is_extended = WORD_KINDS_EXTENDED.contains(&kind);
                let s = kinds.get_mut(&kind).unwrap();
                if !s.contains(word) && extended || (!word_is_extended || kind_is_extended) {
                    s.insert(word.to_string());
                }
            }
        }
        self.kinds = kinds
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();

        // Build alphabets
        for (c, s) in &self.characters {
            self.alphabets.insert(*c, s.chars().collect());
        }

        self.extended = extended;

        self
    }

    pub fn list(&self, sub: &String) -> Result<Vec<String>> {
        if let Some(kind) = WORD_KIND_SUBS.get(sub) {
            if let Some(list) = self.kinds.get(kind) {
                Ok(if sub.starts_with("{W:") {
                    list.iter().map(|x| x.to_uppercase()).collect()
                } else if sub.starts_with("{T:") {
                    list.iter().map(|x| ucfirst(x)).collect()
                } else {
                    list.clone()
                })
            } else {
                Err(anyhow!("Unknown"))
            }
        } else {
            Err(anyhow!("Invalid sub: `{sub}`!"))
        }
    }

    fn get_n(&self, n: usize, kind: WordKind) -> Vec<String> {
        self.kinds
            .get(&kind)
            .unwrap()
            .choose_multiple(&mut thread_rng(), n)
            .cloned()
            .collect()
    }

    fn get(&self, kind: WordKind) -> String {
        self.kinds
            .get(&kind)
            .unwrap()
            .choose(&mut thread_rng())
            .unwrap()
            .clone()
    }

    /**
    Generate a keychain word type 1 (5 lowercase letters, 1 uppercase letter)
    */
    fn keychain_word_1(&self) -> String {
        shuffle(&format!(
            "{}{}",
            random_str(5, self.alphabets.get(&'c').unwrap()),
            random_str(1, self.alphabets.get(&'C').unwrap()),
        ))
    }

    /**
    Generate a keychain word type 2 (5 lowercase letters, 1 digit)
    */
    fn keychain_word_2(&self) -> String {
        shuffle(&format!(
            "{}{}",
            random_str(5, self.alphabets.get(&'c').unwrap()),
            random_str(1, self.alphabets.get(&'d').unwrap()),
        ))
    }

    /**
    Generate a keychain word type 3 (6 lowercase letters)
    */
    fn keychain_word_3(&self) -> String {
        random_str(6, self.alphabets.get(&'c').unwrap())
    }

    /**
    Generate a keychain-style password with `n` "words"

    First 3 "words" are types 1, 2, and 3.
    Additional words are of random type.
    Words are shuffled.
    */
    fn keychain_words(&self, n: usize) -> Vec<String> {
        let mut words = vec![];
        if n == 0 {
            return words;
        }
        words.push(self.keychain_word_1());
        if n >= 2 {
            words.push(self.keychain_word_2());
        }
        if n >= 3 {
            words.push(self.keychain_word_3());
        }
        let mut rng = thread_rng();
        for _ in 4..=n {
            words.push(match KEYCHAIN_WORDS.choose(&mut rng).unwrap() {
                KeychainWord1 => self.keychain_word_1(),
                KeychainWord2 => self.keychain_word_2(),
                KeychainWord3 => self.keychain_word_3(),
            });
        }
        words.shuffle(&mut rng);
        words
    }

    /**
    Generate a keychain-style password like `plvifc-z9kedn-imcbDp`
    */
    pub fn keychain(&self) -> String {
        self.keychain_words(3).join("-")
    }

    /**
    Generate a "code name" like `BLUE STEEL`
    */
    pub fn codename(&self) -> String {
        format!(
            "{} {}",
            self.get(Adjective).to_uppercase(),
            self.get(Noun).to_uppercase(),
        )
    }

    /**
    Generate a "code name series"

    ```text
    TAKENSTAR PRINTEDBOARD
    TAKENSTAR GONEWAGON
    TAKENSTAR PROUDCOOK
    TAKENSTAR YOURLEAVE
    TAKENSTAR DONEPARTY
    TAKENSTAR USUALENTER
    TAKENSTAR FOURBOAT
    TAKENSTAR NICEABOVE
    TAKENSTAR FRENCHPASS
    TAKENSTAR FINEWAVE
    ```
    */
    pub fn codename_series(&self, n: usize) -> String {
        let f = || {
            format!(
                "{}{}",
                self.get(Adjective).to_uppercase(),
                self.get(Noun).to_uppercase(),
            )
        };
        let umbrella = f();
        let mut r = vec![];
        while r.len() < n {
            r.push(format!("{umbrella} {}", f()));
        }
        r.join("\n")
    }

    /**
    Generate a password from the given pattern
    */
    pub fn generate(&self, pattern: &str) -> String {
        let j = pattern.len();
        let mut subs = vec![];
        let mut words = 0;
        let mut kc_words = 0;
        let mut pre = BTreeMap::new();
        'outer: for (sub, kind) in WORD_KIND_SUBS.iter() {
            let mut i = 0;
            while i < j {
                if let Some(p) = pattern[i..].find(sub) {
                    let p = i + p;
                    pre.insert(p, (sub, *kind));
                    i = p + sub.len();
                } else {
                    continue 'outer;
                }
            }
        }
        let mut i = 0;
        for (p, (sub, kind)) in pre.iter() {
            let s = &pattern[i..*p];
            if *p > i {
                words += s
                    .chars()
                    .map(|c| if ['w', 'W', 'T'].contains(&c) { 1 } else { 0 })
                    .sum::<usize>();
                kc_words += s
                    .chars()
                    .map(|c| if c == 'k' { 1 } else { 0 })
                    .sum::<usize>();
                subs.push(Sub::Pattern(s));
            }
            subs.push(Sub::Special((sub.to_string(), *kind)));
            i = *p + sub.len();
        }
        if subs.is_empty() {
            words += pattern
                .chars()
                .map(|c| if ['w', 'W', 'T'].contains(&c) { 1 } else { 0 })
                .sum::<usize>();
            kc_words += pattern
                .chars()
                .map(|c| if c == 'k' { 1 } else { 0 })
                .sum::<usize>();
            subs.push(Sub::Pattern(pattern));
        } else if i < j {
            subs.push(Sub::Pattern(&pattern[i..j]));
        }

        let mut words = self.get_n(words, if self.extended { AllExtended } else { All });
        let mut kc_words = self.keychain_words(kc_words);

        let mut rng = thread_rng();
        let mut r = String::new();

        for sub in subs {
            match sub {
                Sub::Pattern(s) => {
                    for c in s.chars() {
                        match c {
                            'W' => r.push_str(&words.remove(0).to_uppercase()),
                            'w' => r.push_str(&words.remove(0)),
                            'T' => r.push_str(&ucfirst(&words.remove(0))),
                            'k' => r.push_str(&kc_words.remove(0)),
                            _ => {
                                if let Some(alphabet) = self.alphabets.get(&c) {
                                    r.push(*alphabet.choose(&mut rng).unwrap());
                                } else {
                                    r.push(c);
                                }
                            }
                        }
                    }
                }
                Sub::Special((sub, kind)) => {
                    let word = self.get(kind);
                    let word = if sub.starts_with("{W:") {
                        word.to_uppercase()
                    } else if sub.starts_with("{T:") {
                        ucfirst(&word)
                    } else {
                        word
                    };
                    r.push_str(&word);
                }
            }
        }
        r
    }
}

//--------------------------------------------------------------------------------------------------

#[derive(Debug)]
enum Sub<'a> {
    Pattern(&'a str),
    Special((String, WordKind)),
}

//--------------------------------------------------------------------------------------------------

enum KeychainWord {
    KeychainWord1,
    KeychainWord2,
    KeychainWord3,
}

use KeychainWord::*;

const KEYCHAIN_WORDS: [KeychainWord; 3] = [KeychainWord1, KeychainWord2, KeychainWord3];

//--------------------------------------------------------------------------------------------------

/**

Generate a random string of length `n` from `alphabet`

*/
fn random_str(n: usize, alphabet: &[char]) -> String {
    let len = alphabet.len();
    let mut r = String::new();
    for _ in 0..n {
        r.push(alphabet[thread_rng().gen_range(0..len)]);
    }
    r
}

/**

Shuffle a string slice

*/
fn shuffle(s: &str) -> String {
    let mut rng = thread_rng();
    let mut r = s.chars().collect::<Vec<_>>();
    r.shuffle(&mut rng);
    r.into_iter().collect()
}
