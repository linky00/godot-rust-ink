set dotenv-load := true

default:
    just build-rust ${TARGET} debug
    just copy-to-addon ${TARGET} debug ${BIN_NAME} 

build-rust target type:
    cd rust && cargo build --target={{ target }} {{ if type == "release" { "--release" } else { "" } }}

copy-to-addon target type bin_name:
    #!/usr/bin/env bash 
    addon_dir="godot/addons/godot-rust-ink/bin/{{ type }}"
    mkdir -p "$addon_dir"
    cp rust/target/{{ target }}/{{ type }}/{{ bin_name }} "$addon_dir"

build-ink:
    inklecate -o godot/story.json ink/main.ink 