rootdir := ''
prefix := '/usr'
clean := '0'
debug := '0'
vendor := '0'
target := if debug == '1' { 'debug' } else { 'release' }
vendor_args := if vendor == '1' { '--frozen --offline' } else { '' }
debug_args := if debug == '1' { '' } else { '--release' }
cargo_args := vendor_args + ' ' + debug_args


sharedir := rootdir + prefix + '/share'
iconsdir := sharedir + '/icons/hicolor/scalable/apps'
bindir := rootdir + prefix + '/bin'

id := 'com.system76.CosmicAppList'


all: _extract_vendor
    cargo build {{cargo_args}}

# Installs files into the system
install:
    # app list
    install -Dm0644 resources/adw-user-colors.service ~/.config/systemd/user/
    install -Dm0755 target/release/adw-user-colors {{bindir}}/adw-user-colors

# Extracts vendored dependencies if vendor=1
_extract_vendor:
    #!/usr/bin/env sh
    if test {{vendor}} = 1; then
        rm -rf vendor; tar pxf vendor.tar
    fi
