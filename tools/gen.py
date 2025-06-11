tokens = "mvi z nz s ns c nc t0 nt0 zs nzs dma d0".split()

for token in tokens:
    # #[regex("(?i)mov")]
    # Mov,
    print(f"#[regex(\"(?i){token}\")]\n{token.title()},\n")
