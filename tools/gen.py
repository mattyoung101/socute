tokens = "rx pl ra0 wa0 lop top ct0 ct1 ct2 ct3".split()

for token in tokens:
    # #[regex("(?i)mov")]
    # Mov,
    print(f"#[regex(\"(?i){token}\")]\n{token.title()},\n")
