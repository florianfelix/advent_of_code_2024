watch day part:
    cargo watch -q -c -w {{day}}/src -x "run -p {{day}} --bin {{part}}"

new day:
    cargo generate --path ./template --name {{day}}
