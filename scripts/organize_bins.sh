#!/bin/bash
# Shell script to organize binaries into target/${profile}/bin

PROFILE="${1:-release}"
TARGET_DIR="target/$PROFILE"
BIN_DIR="$TARGET_DIR/bin"

# Create bin directory
mkdir -p "$BIN_DIR"

# List of all command binaries
commands=(
    arch base64 basename cat cd cksum clear clr cls
    comm cp cut date del df diff dir dirname du
    echo env expand factor false find fold grep head
    hostname join ln ls md5 mkdir mv nl nproc od
    paste printenv pwd readlink realpath regex rev rm
    rmdir rx seq sha256 sleep sort split stat strings
    sum tac tail tee touch tr tree true truncate
    uname unexpand uniq uptime wc which whoami xxd yes
)

echo "Organizing binaries into $BIN_DIR..."

# Copy nullbox main binary
if [ -f "$TARGET_DIR/nullbox" ]; then
    cp "$TARGET_DIR/nullbox" "$BIN_DIR/nullbox"
    echo "  [OK] nullbox"
fi

# Copy all command binaries
copied=0
missing=0

for cmd in "${commands[@]}"; do
    if [ -f "$TARGET_DIR/$cmd" ]; then
        cp "$TARGET_DIR/$cmd" "$BIN_DIR/$cmd"
        ((copied++))
    else
        echo "  [SKIP] $cmd (not found)"
        ((missing++))
    fi
done

echo ""
echo "Summary:"
echo "  Copied: $copied binaries"
echo "  Missing: $missing binaries"
echo "  Location: $BIN_DIR"
echo ""
echo "Done! All binaries are in: $BIN_DIR"
