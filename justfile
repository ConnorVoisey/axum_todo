dev:
    cargo watch -c -i logs -i openApi.json -x r
test: 
    cargo watch -c -i logs -i openApi.json -x t
