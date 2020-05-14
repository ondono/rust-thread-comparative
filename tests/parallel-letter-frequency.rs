use std::collections::HashMap;

use rust_thread_comparative::crossbeam;
use rust_thread_comparative::rayon;
use rust_thread_comparative::vanilla;

// Poem by Friedrich Schiller. The corresponding music is the European Anthem.
const ODE_AN_DIE_FREUDE: [&str; 8] = [
    "Freude schöner Götterfunken",
    "Tochter aus Elysium,",
    "Wir betreten feuertrunken,",
    "Himmlische, dein Heiligtum!",
    "Deine Zauber binden wieder",
    "Was die Mode streng geteilt;",
    "Alle Menschen werden Brüder,",
    "Wo dein sanfter Flügel weilt.",
];

// Dutch national anthem
const WILHELMUS: [&str; 8] = [
    "Wilhelmus van Nassouwe",
    "ben ik, van Duitsen bloed,",
    "den vaderland getrouwe",
    "blijf ik tot in den dood.",
    "Een Prinse van Oranje",
    "ben ik, vrij, onverveerd,",
    "den Koning van Hispanje",
    "heb ik altijd geëerd.",
];

// American national anthem
const STAR_SPANGLED_BANNER: [&str; 8] = [
    "O say can you see by the dawn's early light,",
    "What so proudly we hailed at the twilight's last gleaming,",
    "Whose broad stripes and bright stars through the perilous fight,",
    "O'er the ramparts we watched, were so gallantly streaming?",
    "And the rockets' red glare, the bombs bursting in air,",
    "Gave proof through the night that our flag was still there;",
    "O say does that star-spangled banner yet wave,",
    "O'er the land of the free and the home of the brave?",
];

#[test]
fn test_no_texts() {
    assert_eq!(vanilla::frequency(&[], 4), HashMap::new());
    assert_eq!(crossbeam::frequency(&[], 4), HashMap::new());
    assert_eq!(rayon::frequency(&[], 4), HashMap::new());
}

#[test]
#[ignore]
fn test_one_letter() {
    let mut hm = HashMap::new();
    hm.insert('a', 1);
    assert_eq!(vanilla::frequency(&["a"], 4), hm);
    assert_eq!(crossbeam::frequency(&["a"], 4), hm);
    assert_eq!(rayon::frequency(&["a"], 4), hm);
}

#[test]
#[ignore]
fn test_case_insensitivity() {
    let mut hm = HashMap::new();
    hm.insert('a', 2);
    assert_eq!(vanilla::frequency(&["aA"], 4), hm);
    assert_eq!(crossbeam::frequency(&["aA"], 4), hm);
    assert_eq!(rayon::frequency(&["aA"], 4), hm);
}

#[test]
#[ignore]
fn test_many_empty_lines() {
    let mut v = Vec::with_capacity(1000);
    for _ in 0..1000 {
        v.push("");
    }
    assert_eq!(vanilla::frequency(&v[..], 4), HashMap::new());
    assert_eq!(crossbeam::frequency(&v[..], 4), HashMap::new());
    assert_eq!(rayon::frequency(&v[..], 4), HashMap::new());
}

#[test]
#[ignore]
fn test_many_times_same_text() {
    let mut v = Vec::with_capacity(1000);
    for _ in 0..1000 {
        v.push("abc");
    }
    let mut hm = HashMap::new();
    hm.insert('a', 1000);
    hm.insert('b', 1000);
    hm.insert('c', 1000);
    assert_eq!(vanilla::frequency(&v[..], 4), hm);
    assert_eq!(crossbeam::frequency(&v[..], 4), hm);
    assert_eq!(rayon::frequency(&v[..], 4), hm);
}

#[test]
#[ignore]
fn test_punctuation_doesnt_count() {
    assert!(!vanilla::frequency(&WILHELMUS, 4).contains_key(&','));
    assert!(!crossbeam::frequency(&WILHELMUS, 4).contains_key(&','));
    assert!(!rayon::frequency(&WILHELMUS, 4).contains_key(&','));
}

#[test]
#[ignore]
fn test_numbers_dont_count() {
    assert!(!vanilla::frequency(&["Testing, 1, 2, 3"], 4).contains_key(&'1'));
    assert!(!crossbeam::frequency(&["Testing, 1, 2, 3"], 4).contains_key(&'1'));
    assert!(!rayon::frequency(&["Testing, 1, 2, 3"], 4).contains_key(&'1'));
}

#[test]
#[ignore]
fn test_all_three_anthems_1_worker() {
    // vanilla
    let mut v = Vec::new();
    for anthem in [ODE_AN_DIE_FREUDE, WILHELMUS, STAR_SPANGLED_BANNER].iter() {
        for line in anthem.iter() {
            v.push(*line);
        }
    }
    let freqs = vanilla::frequency(&v[..], 1);
    assert_eq!(freqs.get(&'a'), Some(&49));
    assert_eq!(freqs.get(&'t'), Some(&56));
    assert_eq!(freqs.get(&'ü'), Some(&2));
    // crossbeam
    let mut v = Vec::new();
    for anthem in [ODE_AN_DIE_FREUDE, WILHELMUS, STAR_SPANGLED_BANNER].iter() {
        for line in anthem.iter() {
            v.push(*line);
        }
    }
    let freqs = crossbeam::frequency(&v[..], 1);
    assert_eq!(freqs.get(&'a'), Some(&49));
    assert_eq!(freqs.get(&'t'), Some(&56));
    assert_eq!(freqs.get(&'ü'), Some(&2));
    // rayon
    let mut v = Vec::new();
    for anthem in [ODE_AN_DIE_FREUDE, WILHELMUS, STAR_SPANGLED_BANNER].iter() {
        for line in anthem.iter() {
            v.push(*line);
        }
    }
    let freqs = rayon::frequency(&v[..], 1);
    assert_eq!(freqs.get(&'a'), Some(&49));
    assert_eq!(freqs.get(&'t'), Some(&56));
    assert_eq!(freqs.get(&'ü'), Some(&2));
}

#[test]
#[ignore]
fn test_all_three_anthems_3_workers() {
    //vanilla
    let mut v = Vec::new();
    for anthem in [ODE_AN_DIE_FREUDE, WILHELMUS, STAR_SPANGLED_BANNER].iter() {
        for line in anthem.iter() {
            v.push(*line);
        }
    }
    let freqs = vanilla::frequency(&v[..], 3);
    assert_eq!(freqs.get(&'a'), Some(&49));
    assert_eq!(freqs.get(&'t'), Some(&56));
    assert_eq!(freqs.get(&'ü'), Some(&2));
    //crossbeam
    let mut v = Vec::new();
    for anthem in [ODE_AN_DIE_FREUDE, WILHELMUS, STAR_SPANGLED_BANNER].iter() {
        for line in anthem.iter() {
            v.push(*line);
        }
    }
    let freqs = crossbeam::frequency(&v[..], 3);
    assert_eq!(freqs.get(&'a'), Some(&49));
    assert_eq!(freqs.get(&'t'), Some(&56));
    assert_eq!(freqs.get(&'ü'), Some(&2));
    //rayon
    let mut v = Vec::new();
    for anthem in [ODE_AN_DIE_FREUDE, WILHELMUS, STAR_SPANGLED_BANNER].iter() {
        for line in anthem.iter() {
            v.push(*line);
        }
    }
    let freqs = rayon::frequency(&v[..], 3);
    assert_eq!(freqs.get(&'a'), Some(&49));
    assert_eq!(freqs.get(&'t'), Some(&56));
    assert_eq!(freqs.get(&'ü'), Some(&2));
}
