create day:
    cargo generate --path ./_template --name {{day}}

lint day:
    cargo clippy -p {{day}}