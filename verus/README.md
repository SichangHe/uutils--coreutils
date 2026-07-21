# signed-number leading-zero proof

`parse_signed_num_normalization.rs` proves the three byte-level outcomes used by `normalize_decimal_leading_zeroes`: drop zeroes before a nonzero decimal digit, collapse repeated zeroes before a safe suffix boundary, or retain the operand. It deliberately excludes `parse_size` semantics; the production bridge invokes both real parser APIs on 21 deterministic forms and invokes `head --bytes=0K`.

The exact staged verifier command and provenance are recorded with the task
evidence; run it from the repository root against
`verus/parse_signed_num_normalization.rs`.
