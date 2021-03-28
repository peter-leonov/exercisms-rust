use robot_name as robot;
use std::collections::HashSet;

fn assert_name_matches_pattern(n: &str) {
    assert!(n.len() == 5, "name is exactly 5 characters long");
    assert!(
        n[0..2].chars().all(|c| ('A'..='Z').contains(&c)),
        "name starts with 2 uppercase letters"
    );
    assert!(
        n[2..].chars().all(|c| ('0'..='9').contains(&c)),
        "name ends with 3 numbers"
    );
}

fn assert_name_is_persistent(r: &robot::Robot) {
    // The type system already proves this, but why not.
    let n1 = r.name();
    let n2 = r.name();
    let n3 = r.name();
    assert_eq!(n1, n2);
    assert_eq!(n2, n3);
}

#[test]
fn test_name_should_match_expected_pattern() {
    let r = robot::Robot::new();
    assert_name_matches_pattern(r.name());
}

#[test]
fn test_name_is_persistent() {
    assert_name_is_persistent(&robot::Robot::new());
}

#[test]
fn test_prng() {
    // `m` and `c` are relatively prime,
    // `a − 1` is divisible by all prime factors of m,
    // a − 1 is divisible by 4 if m is divisible by 4.
    let m = 26 * 26 * 1000;
    let pm = 2 * 13 * 5;
    let c = 7 * 11;
    let a1 = pm * 2;
    let a = a1 + 1;
    let x = m / 2;

    let mut g = robot::LCG::new(x, a, c, m);
    let mut seen = HashSet::with_capacity(m as usize);

    for _ in 0..m {
        seen.insert(g.next());
    }

    assert_eq!(seen.len(), m as usize);
}

#[test]
fn test_different_robots_have_different_names() {
    let r1 = robot::Robot::new();
    let r2 = robot::Robot::new();
    assert_ne!(r1.name(), r2.name(), "Robot names should be different");
}

#[test]
fn test_new_name_should_match_expected_pattern() {
    let mut r = robot::Robot::new();
    assert_name_matches_pattern(r.name());
    r.reset_name();
    assert_name_matches_pattern(r.name());
}

#[test]
fn test_new_name_is_persistent() {
    let mut r = robot::Robot::new();
    r.reset_name();
    assert_name_is_persistent(&r);
}

#[test]
fn test_new_name_is_different_from_old_name() {
    let mut r = robot::Robot::new();
    let n1 = r.name().to_string();
    r.reset_name();
    let n2 = r.name().to_string();
    assert_ne!(n1, n2, "Robot name should change when reset");
}
