/*!
xkcd-style password generator
*/

use anyhow::{anyhow, Result};
use rand::{seq::SliceRandom, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use ucfirst::ucfirst;

//--------------------------------------------------------------------------------------------------

pub const CONFIG: &str = include_str!("../config.json");

//--------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum WordKind {
    Adjective,
    AdjectiveSyllables(usize),
    Adverb,
    AdverbSyllables(usize),
    All,
    AllSyllables(usize),
    AllExtended,
    AllExtendedSyllables(usize),
    Astronomy,
    AstronomySyllables(usize),
    AuxiliaryVerb,
    AuxiliaryVerbSyllables(usize),
    Chthonic,
    ChthonicSyllables(usize),
    City,
    CitySyllables(usize),
    Color,
    ColorSyllables(usize),
    Conjunction,
    ConjunctionSyllables(usize),
    Continent,
    ContinentSyllables(usize),
    Country,
    CountrySyllables(usize),
    Day,
    DaySyllables(usize),
    Element,
    ElementSyllables(usize),
    Extended,
    ExtendedSyllables(usize),
    FemaleName,
    FemaleNameSyllables(usize),
    GreekMyth,
    GreekMythSyllables(usize),
    Interjection,
    InterjectionSyllables(usize),
    IntransitiveVerb,
    IntransitiveVerbSyllables(usize),
    JupiterMoon,
    JupiterMoonSyllables(usize),
    MaleName,
    MaleNameSyllables(usize),
    MarsMoon,
    MarsMoonSyllables(usize),
    Month,
    MonthSyllables(usize),
    Moon,
    MoonSyllables(usize),
    Mythology,
    MythologySyllables(usize),
    Name,
    NameSyllables(usize),
    Nationality,
    NationalitySyllables(usize),
    NeptuneMoon,
    NeptuneMoonSyllables(usize),
    Noun,
    NounSyllables(usize),
    Olympian,
    OlympianSyllables(usize),
    Place,
    PlaceSyllables(usize),
    Planet,
    PlanetSyllables(usize),
    PluralNoun,
    PluralNounSyllables(usize),
    Preposition,
    PrepositionSyllables(usize),
    Pronoun,
    PronounSyllables(usize),
    ProperNoun,
    ProperNounSyllables(usize),
    RomanMyth,
    RomanMythSyllables(usize),
    SaturnMoon,
    SaturnMoonSyllables(usize),
    SingularNoun,
    SingularNounSyllables(usize),
    TransitiveVerb,
    TransitiveVerbSyllables(usize),
    UranusMoon,
    UranusMoonSyllables(usize),
    UsState,
    UsStateSyllables(usize),
    Verb,
    VerbSyllables(usize),
    VerbPast,
    VerbPastSyllables(usize),
}

use WordKind::*;

const WORD_KINDS_EXTENDED: [WordKind; 9] = [
    AllExtended,
    Chthonic,
    Extended,
    JupiterMoon,
    MarsMoon,
    NeptuneMoon,
    Olympian,
    SaturnMoon,
    UranusMoon,
];

impl WordKind {
    fn enumerate(&self, extended: bool) -> Vec<WordKind> {
        let mut r = vec![*self];
        match self {
            Astronomy => r.append(&mut vec![SingularNoun, Noun]),
            AuxiliaryVerb => r.push(Verb),
            Chthonic => r.append(&mut vec![GreekMyth, Mythology, ProperNoun, Noun, Extended]),
            City => r.append(&mut vec![Place, ProperNoun]),
            Color => r.append(&mut vec![SingularNoun, Noun, Verb]),
            Continent => r.append(&mut vec![Place, ProperNoun, SingularNoun, Noun]),
            Country => r.append(&mut vec![Place, ProperNoun]),
            Day => r.append(&mut vec![SingularNoun, Noun]),
            Element => r.append(&mut vec![ProperNoun, Noun]),
            FemaleName => r.append(&mut vec![Name, ProperNoun]),
            GreekMyth => r.append(&mut vec![Mythology, ProperNoun, Noun]),
            IntransitiveVerb => r.push(Verb),
            JupiterMoon => r.append(&mut vec![Moon, Astronomy, SingularNoun, Noun, Extended]),
            MaleName => r.append(&mut vec![Name, ProperNoun]),
            MarsMoon => r.append(&mut vec![Moon, Astronomy, SingularNoun, Noun, Extended]),
            Month => r.append(&mut vec![SingularNoun, Noun, ProperNoun]),
            Moon => r.append(&mut vec![Astronomy, SingularNoun, Noun]),
            Mythology => r.append(&mut vec![ProperNoun, Noun]),
            Name => r.push(ProperNoun),
            Nationality => r.push(Adjective),
            NeptuneMoon => r.append(&mut vec![Moon, Astronomy, SingularNoun, Noun, Extended]),
            Olympian => r.append(&mut vec![GreekMyth, Mythology, ProperNoun, Noun, Extended]),
            Planet => r.append(&mut vec![Astronomy, SingularNoun, Noun]),
            PluralNoun => r.push(Noun),
            RomanMyth => r.append(&mut vec![Mythology, ProperNoun, Noun]),
            SaturnMoon => r.append(&mut vec![Moon, Astronomy, SingularNoun, Noun, Extended]),
            SingularNoun => r.push(Noun),
            TransitiveVerb => r.push(Verb),
            UranusMoon => r.append(&mut vec![Moon, Astronomy, SingularNoun, Noun, Extended]),
            UsState => r.append(&mut vec![Place, ProperNoun]),
            VerbPast => r.push(Verb),
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
            Adjective => String::from("{adj}"),
            Adverb => String::from("{adv}"),
            All => String::from("{a}"),
            AllExtended => String::from("{a.ext}"),
            Astronomy => String::from("{ast}"),
            AuxiliaryVerb => String::from("{v.aux}"),
            Chthonic => String::from("{chthonic}"),
            City => String::from("{city}"),
            Color => String::from("{color}"),
            Conjunction => String::from("{conj}"),
            Continent => String::from("{cont}"),
            Country => String::from("{country}"),
            Day => String::from("{day}"),
            Element => String::from("{el}"),
            Extended => String::from("{ext}"),
            FemaleName => String::from("{fname}"),
            GreekMyth => String::from("{greekmyth}"),
            Interjection => String::from("{i}"),
            IntransitiveVerb => String::from("{v.int}"),
            JupiterMoon => String::from("{jupitermoon}"),
            MaleName => String::from("{mname}"),
            MarsMoon => String::from("{marsmoon}"),
            Month => String::from("{mon}"),
            Moon => String::from("{moon}"),
            Mythology => String::from("{myth}"),
            Name => String::from("{name}"),
            Nationality => String::from("{nat}"),
            NeptuneMoon => String::from("{neptunemoon}"),
            Noun => String::from("{n}"),
            Olympian => String::from("{olympian}"),
            Place => String::from("{place}"),
            Planet => String::from("{planet}"),
            PluralNoun => String::from("{n.pl}"),
            Preposition => String::from("{prep}"),
            Pronoun => String::from("{n.pro}"),
            ProperNoun => String::from("{n.prop}"),
            RomanMyth => String::from("{romanmyth}"),
            SaturnMoon => String::from("{saturnmoon}"),
            SingularNoun => String::from("{n.s}"),
            TransitiveVerb => String::from("{v.tr}"),
            UranusMoon => String::from("{uranusmoon}"),
            UsState => String::from("{us-state}"),
            Verb => String::from("{v}"),
            VerbPast => String::from("{v.past}"),
            AdjectiveSyllables(n) => format!("{{adj:{n}}}"),
            AdverbSyllables(n) => format!("{{adv:{n}}}"),
            AllSyllables(n) => format!("{{a:{n}}}"),
            AllExtendedSyllables(n) => format!("{{a.ext:{n}}}"),
            AstronomySyllables(n) => format!("{{ast:{n}}}"),
            AuxiliaryVerbSyllables(n) => format!("{{v.aux:{n}}}"),
            ChthonicSyllables(n) => format!("{{chthonic:{n}}}"),
            CitySyllables(n) => format!("{{city:{n}}}"),
            ColorSyllables(n) => format!("{{color:{n}}}"),
            ConjunctionSyllables(n) => format!("{{conj:{n}}}"),
            ContinentSyllables(n) => format!("{{cont:{n}}}"),
            CountrySyllables(n) => format!("{{country:{n}}}"),
            DaySyllables(n) => format!("{{day:{n}}}"),
            ElementSyllables(n) => format!("{{el:{n}}}"),
            ExtendedSyllables(n) => format!("{{ext:{n}}}"),
            FemaleNameSyllables(n) => format!("{{fname:{n}}}"),
            GreekMythSyllables(n) => format!("{{greekmyth:{n}}}"),
            InterjectionSyllables(n) => format!("{{i:{n}}}"),
            IntransitiveVerbSyllables(n) => format!("{{v.int:{n}}}"),
            JupiterMoonSyllables(n) => format!("{{jupitermoon:{n}}}"),
            MaleNameSyllables(n) => format!("{{mname:{n}}}"),
            MarsMoonSyllables(n) => format!("{{marsmoon:{n}}}"),
            MonthSyllables(n) => format!("{{mon:{n}}}"),
            MoonSyllables(n) => format!("{{moon:{n}}}"),
            MythologySyllables(n) => format!("{{myth:{n}}}"),
            NameSyllables(n) => format!("{{name:{n}}}"),
            NationalitySyllables(n) => format!("{{nat:{n}}}"),
            NeptuneMoonSyllables(n) => format!("{{neptunemoon:{n}}}"),
            NounSyllables(n) => format!("{{n:{n}}}"),
            OlympianSyllables(n) => format!("{{olympian:{n}}}"),
            PlaceSyllables(n) => format!("{{place:{n}}}"),
            PlanetSyllables(n) => format!("{{planet:{n}}}"),
            PluralNounSyllables(n) => format!("{{n.pl:{n}}}"),
            PrepositionSyllables(n) => format!("{{prep:{n}}}"),
            PronounSyllables(n) => format!("{{n.pro:{n}}}"),
            ProperNounSyllables(n) => format!("{{n.prop:{n}}}"),
            RomanMythSyllables(n) => format!("{{romanmyth:{n}}}"),
            SaturnMoonSyllables(n) => format!("{{saturnmoon:{n}}}"),
            SingularNounSyllables(n) => format!("{{n.s:{n}}}"),
            TransitiveVerbSyllables(n) => format!("{{v.tr:{n}}}"),
            UranusMoonSyllables(n) => format!("{{uranusmoon:{n}}}"),
            UsStateSyllables(n) => format!("{{us-state:{n}}}"),
            VerbSyllables(n) => format!("{{v:{n}}}"),
            VerbPastSyllables(n) => format!("{{v.past:{n}}}"),
        }
    }

    fn with_syllables(&self, syllables: usize) -> WordKind {
        match self {
            Adjective => AdjectiveSyllables(syllables),
            Adverb => AdverbSyllables(syllables),
            All => AllSyllables(syllables),
            AllExtended => AllExtendedSyllables(syllables),
            Astronomy => AstronomySyllables(syllables),
            AuxiliaryVerb => AuxiliaryVerbSyllables(syllables),
            Chthonic => ChthonicSyllables(syllables),
            City => CitySyllables(syllables),
            Color => ColorSyllables(syllables),
            Conjunction => ConjunctionSyllables(syllables),
            Continent => ContinentSyllables(syllables),
            Country => CountrySyllables(syllables),
            Day => DaySyllables(syllables),
            Element => ElementSyllables(syllables),
            Extended => ExtendedSyllables(syllables),
            FemaleName => FemaleNameSyllables(syllables),
            GreekMyth => GreekMythSyllables(syllables),
            Interjection => InterjectionSyllables(syllables),
            IntransitiveVerb => IntransitiveVerbSyllables(syllables),
            JupiterMoon => JupiterMoonSyllables(syllables),
            MaleName => MaleNameSyllables(syllables),
            MarsMoon => MarsMoonSyllables(syllables),
            Month => MonthSyllables(syllables),
            Moon => MoonSyllables(syllables),
            Mythology => MythologySyllables(syllables),
            Name => NameSyllables(syllables),
            Nationality => NationalitySyllables(syllables),
            NeptuneMoon => NeptuneMoonSyllables(syllables),
            Noun => NounSyllables(syllables),
            Olympian => OlympianSyllables(syllables),
            Place => PlaceSyllables(syllables),
            Planet => PlanetSyllables(syllables),
            PluralNoun => PluralNounSyllables(syllables),
            Preposition => PrepositionSyllables(syllables),
            Pronoun => PronounSyllables(syllables),
            ProperNoun => ProperNounSyllables(syllables),
            RomanMyth => RomanMythSyllables(syllables),
            SaturnMoon => SaturnMoonSyllables(syllables),
            SingularNoun => SingularNounSyllables(syllables),
            TransitiveVerb => TransitiveVerbSyllables(syllables),
            UranusMoon => UranusMoonSyllables(syllables),
            UsState => UsStateSyllables(syllables),
            Verb => VerbSyllables(syllables),
            VerbPast => VerbPastSyllables(syllables),
            _ => *self,
        }
    }
}

//--------------------------------------------------------------------------------------------------

#[derive(Clone, Deserialize, Serialize)]
pub struct WordDetails {
    kinds: Vec<WordKind>,
    syllables: usize,
}

//--------------------------------------------------------------------------------------------------

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    characters: BTreeMap<char, String>,
    words: BTreeMap<String, WordDetails>,

    #[serde(skip)]
    words_built: BTreeMap<String, WordDetails>,

    #[serde(skip)]
    kinds: BTreeMap<WordKind, Vec<String>>,

    #[serde(skip)]
    alphabets: BTreeMap<char, Vec<char>>,

    #[serde(skip)]
    pub subs: BTreeMap<String, WordKind>,

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
        // Build word lists for each kind and syllable count
        let mut kinds: HashMap<WordKind, BTreeSet<String>> = HashMap::new();
        self.words_built = BTreeMap::new();
        for (word, word_details) in &self.words {
            let word_kinds = word_details
                .kinds
                .iter()
                .flat_map(|x| x.enumerate(extended))
                .collect::<Vec<_>>();
            let word_is_extended = word_kinds.contains(&Extended);
            for kind in &word_kinds {
                let kind_is_extended = WORD_KINDS_EXTENDED.contains(kind);
                if extended || !word_is_extended || kind_is_extended {
                    let s = kinds.entry(*kind).or_default();
                    s.insert(word.to_string());
                    let s = kinds
                        .entry(kind.with_syllables(word_details.syllables))
                        .or_default();
                    s.insert(word.to_string());
                }
            }
            self.words_built.insert(
                word.clone(),
                WordDetails {
                    kinds: word_kinds,
                    syllables: word_details.syllables,
                },
            );
        }
        self.kinds = kinds
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();
        self.subs = self
            .kinds
            .keys()
            .flat_map(|x| {
                let sub = x.sub();
                [
                    (sub.clone(), *x),
                    (format!("{{W:{}", &sub[1..]), *x),
                    (format!("{{T:{}", &sub[1..]), *x),
                ]
            })
            .collect();

        // Build alphabets
        for (c, s) in &self.characters {
            self.alphabets.insert(*c, s.chars().collect());
        }

        self.extended = extended;

        self
    }

    pub fn list(&self, sub: &String) -> Result<Vec<String>> {
        if let Some(kind) = self.subs.get(sub) {
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

    /// Generate a keychain word type 1 (5 lowercase letters, 1 uppercase letter)
    fn keychain_word_1(&self) -> String {
        shuffle(&format!(
            "{}{}",
            random_str(5, self.alphabets.get(&'c').unwrap()),
            random_str(1, self.alphabets.get(&'C').unwrap()),
        ))
    }

    /// Generate a keychain word type 2 (5 lowercase letters, 1 digit)
    fn keychain_word_2(&self) -> String {
        shuffle(&format!(
            "{}{}",
            random_str(5, self.alphabets.get(&'c').unwrap()),
            random_str(1, self.alphabets.get(&'d').unwrap()),
        ))
    }

    /// Generate a keychain word type 3 (6 lowercase letters)
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

    /// Generate a keychain-style password like `plvifc-z9kedn-imcbDp`
    pub fn keychain(&self) -> String {
        self.keychain_words(3).join("-")
    }

    /// Generate a "code name" like `BLUE STEEL`
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
        let mut adjectives = self
            .get_n(n + 1, Adjective)
            .iter()
            .map(|x| x.to_uppercase())
            .collect::<Vec<_>>();
        let mut nouns = self
            .get_n(n + 1, Noun)
            .iter()
            .map(|x| x.to_uppercase())
            .collect::<Vec<_>>();
        let umbrella = format!("{}{}", adjectives.remove(0), nouns.remove(0));
        let mut r = vec![];
        while !adjectives.is_empty() {
            r.push(format!(
                "{umbrella} {}{}",
                adjectives.remove(0),
                nouns.remove(0),
            ));
        }
        r.join("\n")
    }

    /// Generate a haiku
    pub fn haiku(&self, variant: HaikuVariant) -> String {
        let (add_syllables, kebab, newline) = variant.options();
        let (word_sep, line_sep, syl_sep) = if newline {
            (" ", "\n", " ")
        } else if kebab {
            ("-", "/", "")
        } else {
            (" ", " / ", " ")
        };
        let max = 5;

        let mut rng = thread_rng();

        let mut lines = vec![];
        let mut w = BTreeMap::new();

        // Pick random syllable pattern
        for mut c in [5, 7, 5] {
            let mut words = vec![];
            while c > 0 {
                let poss = (1..=std::cmp::min(c, max)).collect::<Vec<_>>();
                let i = poss.choose(&mut rng).unwrap();
                words.push(*i);
                c -= *i;
                w.entry(*i).and_modify(|e| *e += 1).or_insert(1);
            }
            lines.push(words);
        }

        // Get words without repetion
        let mut w = w
            .iter()
            .map(|(syllables, words)| {
                (
                    *syllables,
                    self.get_n(
                        *words,
                        if self.extended {
                            AllExtendedSyllables(*syllables)
                        } else {
                            AllSyllables(*syllables)
                        },
                    ),
                )
            })
            .collect::<BTreeMap<usize, Vec<String>>>();

        let mut r = vec![];

        for line in &lines {
            let mut words = vec![];
            for (i, syllables) in line.iter().enumerate() {
                let s = w.get_mut(syllables).unwrap();
                let word = s.remove(0);
                let details = self.words_built.get(&word).unwrap();
                let word = if i == 0 || details.kinds.contains(&ProperNoun) {
                    ucfirst(&word)
                } else {
                    word
                };
                let word = word
                    .replace("Newhampshire", "New Hampshire")
                    .replace("Newjersey", "New Jersey")
                    .replace("Newmexico", "New Mexico")
                    .replace("Newyork", "New York")
                    .replace("Northamerica", "North America")
                    .replace("Northcarolina", "North Carolina")
                    .replace("Northdakota", "North Dakota")
                    .replace("Rhodeisland", "Rhode Island")
                    .replace("Southamerica", "South America")
                    .replace("Southcarolina", "South Carolina")
                    .replace("Southdakota", "South Dakota")
                    .replace("Westvirginia", "West Virginia");
                words.push(if i == 0 { ucfirst(&word) } else { word });
            }
            let mut words = words.join(word_sep);
            if add_syllables {
                words.push_str(&format!(
                    "{syl_sep}({})",
                    line.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                ));
            }
            r.push(words);
        }

        r.join(line_sep)
    }

    /// Generate a password from the given pattern
    pub fn generate(&self, pattern: &str) -> String {
        let j = pattern.len();
        let mut subs = vec![];
        let mut words = 0;
        let mut kc_words = 0;
        let mut pre = BTreeMap::new();
        'outer: for (sub, kind) in self.subs.iter() {
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

impl Default for Config {
    fn default() -> Config {
        Config::from_str(CONFIG, false).unwrap()
    }
}

//--------------------------------------------------------------------------------------------------

pub enum HaikuVariant {
    Normal,
    WithSyllables,
    WithSyllablesCondensed,
    Condensed,
    Full,
    FullWithSyllables,
}

impl HaikuVariant {
    fn options(&self) -> (bool, bool, bool) {
        match self {
            Self::Condensed => (false, true, false),
            Self::WithSyllablesCondensed => (true, true, false),
            Self::Normal => (false, false, false),
            Self::WithSyllables => (true, false, false),
            Self::Full => (false, false, true),
            Self::FullWithSyllables => (true, false, true),
        }
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

/// Generate a random string of length `n` from `alphabet`
fn random_str(n: usize, alphabet: &[char]) -> String {
    let len = alphabet.len();
    let mut r = String::new();
    for _ in 0..n {
        r.push(alphabet[thread_rng().gen_range(0..len)]);
    }
    r
}

/// Shuffle a string slice
fn shuffle(s: &str) -> String {
    let mut rng = thread_rng();
    let mut r = s.chars().collect::<Vec<_>>();
    r.shuffle(&mut rng);
    r.into_iter().collect()
}
