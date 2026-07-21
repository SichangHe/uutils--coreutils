//! Verus model for `normalize_decimal_leading_zeroes`.
//!
//! The model proves the byte-level normalization decision. Rust tests bridge
//! its cases to the production `str` implementation and `head` entry point.

use vstd::prelude::*;

fn main() {}

verus! {

spec fn nonzero_decimal(byte: u8) -> bool {
    49 <= byte && byte <= 57
}

fn is_nonzero_decimal(byte: u8) -> (result: bool)
    ensures
        result == nonzero_decimal(byte),
{
    49 <= byte && byte <= 57
}

spec fn zero_run_ends_at(input: Seq<u8>, start: int) -> bool {
    &&& 0 <= start
    &&& start <= input.len()
    &&& forall|index: int| 0 <= index && index < start ==> input[index] == 48
    &&& (start < input.len() ==> input[start] != 48)
}

spec fn collapse_allowed(input: Seq<u8>, start: int) -> bool {
    &&& 1 < start
    &&& start < input.len()
    &&& input[start] != 120
    &&& (input[start] != 98 || input.len() == start + 1)
}

spec fn normalization_matches(input: Seq<u8>, output: Seq<u8>, start: int) -> bool {
    &&& zero_run_ends_at(input, start)
    &&& if start == input.len() {
        output == input
    } else if nonzero_decimal(input[start]) {
        output == input.subrange(start, input.len() as int)
    } else if collapse_allowed(input, start) {
        output == Seq::<u8>::empty().push(48) + input.subrange(start, input.len() as int)
    } else {
        output == input
    }
}

fn leading_zeroes(input: &Vec<u8>) -> (start: usize)
    ensures
        zero_run_ends_at(input@, start as int),
{
    let mut start = 0;
    while start < input.len() && input[start] == 48
        invariant
            start <= input.len(),
            forall|index: int| 0 <= index && index < start ==> input@[index] == 48,
        decreases input.len() - start,
    {
        start = start + 1;
    }
    start
}

fn copy_suffix(input: &Vec<u8>, begin: usize) -> (output: Vec<u8>)
    requires
        begin <= input.len(),
    ensures
        output@ == input@.subrange(begin as int, input@.len() as int),
{
    let mut output = Vec::new();
    let mut index = begin;
    while index < input.len()
        invariant
            begin <= index <= input.len(),
            output@ == input@.subrange(begin as int, index as int),
        decreases input.len() - index,
    {
        output.push(input[index]);
        index = index + 1;
    }
    output
}

fn zero_then_suffix(input: &Vec<u8>, begin: usize) -> (output: Vec<u8>)
    requires
        begin <= input.len(),
    ensures
        output@
            == Seq::<u8>::empty().push(48)
                + input@.subrange(begin as int, input@.len() as int),
{
    let mut output = Vec::new();
    output.push(48);
    let mut index = begin;
    while index < input.len()
        invariant
            begin <= index <= input.len(),
            output@
                == Seq::<u8>::empty().push(48)
                    + input@.subrange(begin as int, index as int),
        decreases input.len() - index,
    {
        output.push(input[index]);
        index = index + 1;
    }
    output
}

/// Return the production normalizer's byte-level outcome.
fn normalize_leading_zeroes(input: &Vec<u8>) -> (output: Vec<u8>)
    ensures
        exists|start: int| normalization_matches(input@, output@, start),
{
    let start = leading_zeroes(input);
    assert(zero_run_ends_at(input@, start as int));
    if start == input.len() {
        let output = copy_suffix(input, 0);
        assert(input@.subrange(0, input@.len() as int) == input@);
        assert(normalization_matches(input@, output@, start as int));
        return output;
    }

    let first = input[start];
    if is_nonzero_decimal(first) {
        let output = copy_suffix(input, start);
        assert(normalization_matches(input@, output@, start as int));
        return output;
    }

    if start > 1 && first != 120 && (first != 98 || input.len() == start + 1) {
        let output = zero_then_suffix(input, start);
        assert(normalization_matches(input@, output@, start as int));
        return output;
    }

    let output = copy_suffix(input, 0);
    assert(!nonzero_decimal(first));
    assert(!collapse_allowed(input@, start as int));
    assert(input@.subrange(0, input@.len() as int) == input@);
    assert(normalization_matches(input@, output@, start as int));
    output
}

} // verus!
