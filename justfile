test:
    cargo test

watch:
    cargo watch -x fmt -x check -x test

coverage:
    cargo tarpaulin

cctest:
    codecrafters test

install_cctest:
    curl https://codecrafters.io/install.sh | sh

ready:
    just install_cctest&
    cargo build
    cargo test