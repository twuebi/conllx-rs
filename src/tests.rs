use std::io;
use std::str;
use std::collections::BTreeMap;

use super::ReadSentence;
use super::{Features, Sentence, Token, TokenBuilder, WriteSentence};

const TEST_FRAGMENT: &'static str =
    r"1	Die	die	ART	ART	nsf	2	DET
2	Großaufnahme	Großaufnahme	N	NN	nsf	0	ROOT

1	Gilles	Gilles	N	NE	nsm	0	ROOT
2	Deleuze	Deleuze	N	NE	case:nominative|number:singular|gender:masculine	1	APP";

// Not according to CoNLL-X, but we want to handle it anyway.
const TEST_FRAGMENT_ROBUST: &'static str =
    r"1	Die	die	ART	ART	nsf	2	DET
2	Großaufnahme	Großaufnahme	N	NN	nsf	0	ROOT


1	Gilles	Gilles	N	NE	nsm	0	ROOT
2	Deleuze	Deleuze	N	NE	case:nominative|number:singular|gender:masculine	1	APP";

const TEST_FRAGMENT_MARKED_EMPTY: &'static str =
    r"1	Die	die	ART	ART	nsf	2	DET	_	_
2	Großaufnahme	Großaufnahme	N	NN	nsf	0	ROOT	_	_

1	Gilles	Gilles	N	NE	nsm	0	ROOT	_	_
2	Deleuze	Deleuze	N	NE	case:nominative|number:singular|gender:masculine	1	APP	_	_";

fn test_sentences() -> Vec<Sentence> {
    vec![
        Sentence::new(vec![
            TokenBuilder::new()
                .form("Die")
                .lemma("die")
                .cpos("ART")
                .pos("ART")
                .features(Features::from_string("nsf"))
                .head(2)
                .head_rel("DET")
                .token(),
            TokenBuilder::new()
                .form("Großaufnahme")
                .lemma("Großaufnahme")
                .cpos("N")
                .pos("NN")
                .features(Features::from_string("nsf"))
                .head(0)
                .head_rel("ROOT")
                .token(),
        ]),
        Sentence::new(vec![
            TokenBuilder::new()
                .form("Gilles")
                .lemma("Gilles")
                .cpos("N")
                .pos("NE")
                .features(Features::from_string("nsm"))
                .head(0)
                .head_rel("ROOT")
                .token(),
            TokenBuilder::new()
                .form("Deleuze")
                .lemma("Deleuze")
                .cpos("N")
                .pos("NE")
                .features(Features::from_string(
                    "case:nominative|number:singular|gender:masculine",
                ))
                .head(1)
                .head_rel("APP")
                .token(),
        ]),
    ]
}

fn string_reader(s: &str) -> Box<io::BufRead> {
    Box::new(io::Cursor::new(s.as_bytes().to_owned()))
}

fn test_parsing(correct: Vec<Sentence>, fragment: &str) {
    let reader = super::Reader::new(string_reader(fragment));
    let sentences: Vec<Sentence> = reader.sentences().map(|s| s.unwrap()).collect();
    assert_eq!(correct, sentences);
}

#[test]
fn reader() {
    test_parsing(test_sentences(), TEST_FRAGMENT);
}

#[test]
fn reader_robust() {
    test_parsing(test_sentences(), TEST_FRAGMENT_ROBUST);
}

#[test]
fn reader_marked_empty() {
    test_parsing(test_sentences(), TEST_FRAGMENT_MARKED_EMPTY);
}

#[test]
#[should_panic(expected = "ParseIntError")]
fn reader_rejects_non_numeric_id() {
    let mut reader = super::Reader::new(string_reader("test"));
    reader.read_sentence().unwrap();
}

#[test]
fn writer() {
    let output = Vec::new();
    let mut writer = super::Writer::new(Box::new(output));

    for sentence in test_sentences() {
        writer.write_sentence(&sentence).unwrap();
    }

    assert_eq!(
        TEST_FRAGMENT_MARKED_EMPTY,
        str::from_utf8(writer.get_ref()).unwrap()
    );
}

fn token_with_features() -> Vec<Token> {
    vec![
        TokenBuilder::new()
            .form("Gilles")
            .lemma("Gilles")
            .cpos("N")
            .pos("NE")
            .features(Features::from_string(
                "case:nominative|number:singular|gender:masculine",
            ))
            .head(0)
            .head_rel("ROOT")
            .token(),
        TokenBuilder::new()
            .form("Deleuze")
            .lemma("Deleuze")
            .cpos("N")
            .pos("NE")
            .features(Features::from_string("nominative|singular|masculine"))
            .head(1)
            .head_rel("APP")
            .token(),
    ]
}

fn features_correct() -> Vec<BTreeMap<String, Option<String>>> {
    let mut correct1 = BTreeMap::new();
    correct1.insert("case".to_owned(), Some("nominative".to_owned()));
    correct1.insert("number".to_owned(), Some("singular".to_owned()));
    correct1.insert("gender".to_owned(), Some("masculine".to_owned()));

    let mut correct2 = BTreeMap::new();
    correct2.insert("nominative".to_owned(), None);
    correct2.insert("singular".to_owned(), None);
    correct2.insert("masculine".to_owned(), None);

    vec![correct1, correct2]
}

#[test]
fn features() {
    let tokens = token_with_features();
    let features = features_correct();

    for (token, correct) in tokens.iter().zip(features) {
        let kv = token.features().as_ref().unwrap().as_map();
        assert_eq!(correct, kv);
    }
}
