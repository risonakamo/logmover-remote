#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use sublime_fuzzy::{FuzzySearch,Scoring};
use regex::{Regex};
use lazy_static::lazy_static;
use std::borrow::Cow;

fn main()
{
    // let result=best_match(
    //     "[SubsPlease] Uma Musume - Pretty Derby S2 - 08 (1080p) [5752663E].mkv",
    //     // "[SubsPlease] Uma Musume - Pretty Derby S2 - 08 (1080p) [5752663E].mkv",
    //     "[Erai-raws] Uma Musume - Pretty Derby Season 2 - 09 [1080p][Multiple Subtitle].mkv"
    // );

    let mut scoring=Scoring::emphasize_distance();

    scoring.bonus_consecutive=100000;

    let result=FuzzySearch::new(
        "[SubsPlease] Uma Musume - Pretty Derby S2 - 08 (1080p) [5752663E].mkv",
        "[Erai-raws] Uma Musume - Pretty Derby Season 2 - 09 [1080p][Multiple Subtitle].mkv"
    )
    .score_with(&scoring)
    .case_insensitive()
    .best_match();

    println!("{:#?}",result);

    println!("{}",simplifyName("actualy just works"));
    println!("{}",simplifyName("[Erai-raws] Uma Musume - Pretty Derby Season 2 - 09 [1080p][Multiple Subtitle].mkv"));
}

/// perform custom name simplification
fn simplifyName(filename:&str)->String
{
    lazy_static!
    {
        static ref replacer1:Regex=Regex::new(r"[\[\(].*?[\]\)]|\.mkv|\.mp4|END|end").unwrap();
        static ref replacer2:Regex=Regex::new(r"[^\w]|\d").unwrap();
    }

    let afterReplacer1:Cow<str>=replacer1.replace_all(filename,"");
    let res:Cow<str>=replacer2.replace_all(&afterReplacer1,"");

    return res.to_lowercase();
}