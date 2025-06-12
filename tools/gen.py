tokens = "equ org ends if ifdef alh all m0 m1 m2 m3 mc0 mc1 mc2 mc3".split()

for token in tokens:
    # #[regex("(?i)mov")]
    # Mov,
    print(f"#[regex(\"(?i){token}\")]\n{token.title()},\n")
