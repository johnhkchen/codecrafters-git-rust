test:
    cargo test

watch:
    cargo watch -x fmt -x check -x test

coverage:
    cargo tarpaulin

cctest:
    codecrafters test