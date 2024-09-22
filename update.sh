#!/usr/bin/env bash

if ! command -v chiptool &> /dev/null; then
    echo "chiptool could not be found. Install it with the following command:"
    echo ""
    echo "    cargo install --git https://github.com/embassy-rs/chiptool --locked"
    echo ""
    exit 1
fi

if ! command -v form &> /dev/null; then
    echo "form could not be found. Install it with the following command:"
    echo ""
    echo "    cargo install form"
    echo ""
    exit 1
fi

set -euxo pipefail

rm -rf src/nrf*
rm -rf src/lib.rs

declare -A identifiers=(
    # Chip name to SVD/YAML/.X identifier
    # This allows to have multiple chip definition with the same SVD/YAML/.X file-set (e.g. for the nrf5340-app chip)
    ["nrf52840"]="nrf52840"
)
declare -A targets=(
    # Chip name to CPU arch target
    ["nrf52840"]="thumbv7em-none-eabihf"
)
chips=($(printf "%s\n" "${!identifiers[@]}" | sort))

# Generated types
for chip in ${chips[@]}; do 
    chip_identifier=${identifiers[$chip]}
    chiptool generate --svd svd/$chip_identifier.svd --transform svd/$chip_identifier.yaml
    rustfmt lib.rs
    sed -i '/#!\[no_std]/d' lib.rs
    form -i lib.rs -o src/$chip
    mv src/$chip/lib.rs src/$chip/mod.rs
    rm lib.rs
done

# Create build.rs from template
chip_list=""
for chip in ${chips[@]}; do
    chip_identifier=${identifiers[$chip]}
    chip_list+=$(printf '("%s", "%s"), ' "$chip" "$chip_identifier")
done
chip_list=${chip_list%, }   # Remove the trailing comma and space
sed "s/CHIP_LIST/$chip_list/" build.rs.in > build.rs

# Create Cargo.toml from template
features_list=""
flavors_list=""
for chip in ${chips[@]}; do
    chip_target=${targets[$chip]}
    features_list+=$(printf '%s = []' "$chip")
    features_list+="\n"
    flavors_list+=$(printf '    { name = "%s", features = ["%s"], target = "%s" },' "$chip" "$chip" "$chip_target")
    flavors_list+="\n"
done
flavors_list=${flavors_list%,\\n}   # Remove the trailing comma and newline
features_list=${features_list%\\n}   # Remove the trailing newline

awk -v features="$features_list" \
    -v flavors="$flavors_list" \
     '{gsub(/FEATURES/, features); gsub(/FLAVORS/, flavors)}1' Cargo.toml.in > Cargo.toml

# Create src/lib.rs
cfg_features=$(printf 'feature = "%s", ' "${chips[@]}")
cfg_features=${cfg_features%, }   # Remove the trailing comma and space
chip_cfg_attrs=""
for chip in "${chips[@]}"; do
    chip_cfg_attrs+=$(printf '#[cfg_attr(feature = "%s", path = "./%s/mod.rs")]' "$chip" "$chip")
done

# Perform the replacements in the template and save to `src/lib.rs`
chip_list=$(printf '`%s`, ' "${chips[@]}")
chip_list=${chip_list%, }   # Remove the trailing comma and space
awk -v cfg_all="$cfg_features" \
    -v cfg_any="$cfg_features" \
    -v chip_list="$chip_list" \
    -v chip_cfg_attrs="$chip_cfg_attrs" \
    '{gsub(/CHIPS_CFG_ALL/, cfg_all); gsub(/CHIPS_CFG_ANY/, cfg_any); gsub(/CHIP_LIST/, chip_list); gsub(/CHIP_CFG_ATTRS/, chip_cfg_attrs)}1' src/lib.rs.in > src/lib.rs

# Code formatting and sanity check
cargo fmt

for chip in ${chips[@]}; do 
    cargo check --features rt,$chip
done
