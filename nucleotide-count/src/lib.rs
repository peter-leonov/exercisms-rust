use std::collections::HashMap;

// Here I'm just trying to reuse count() at all costs
// to practice the "optional chaining" operator `?`.
// So, yes, the performance is struggling and the better
// solution would be to have it all inside out.

fn is_valid(c: char) -> bool {
    match c {
        'A' | 'C' | 'G' | 'T' => true,
        _ => false,
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_valid(nucleotide) {
        return Err(nucleotide);
    }
    dna.chars().try_fold(0, |acc, x| {
        if x == nucleotide {
            Ok(acc + 1)
        } else if is_valid(x) {
            Ok(acc)
        } else {
            Err(x)
        }
    })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = HashMap::with_capacity(4);

    map.insert('A', count('A', dna)?);
    map.insert('C', count('C', dna)?);
    map.insert('G', count('G', dna)?);
    map.insert('T', count('T', dna)?);

    Ok(map)
}
