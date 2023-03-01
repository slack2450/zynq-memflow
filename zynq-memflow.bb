# Auto-Generated by cargo-bitbake 0.3.16
#
inherit cargo

# If this is git based prefer versioned ones if they exist
# DEFAULT_PREFERENCE = "-1"

# how to get zynq-memflow could be as easy as but default to a git checkout:
# SRC_URI += "crate://crates.io/zynq-memflow/0.1.0"
SRC_URI += "git://git@github.com/slack2450/zynq-memflow.git;protocol=ssh;nobranch=1"
SRCREV = "ab2caa58d23d3394b913a82ef697c0cc461b4f85"
S = "${WORKDIR}/git"
CARGO_SRC_DIR = ""
PV:append = ".AUTOINC+ab2caa58d2"

# please note if you have entries that do not begin with crate://
# you must change them to how that package can be fetched
SRC_URI += " \
    crate://crates.io/abi_stable/0.10.5 \
    crate://crates.io/abi_stable_derive/0.10.3 \
    crate://crates.io/abi_stable_shared/0.10.3 \
    crate://crates.io/adler/1.0.2 \
    crate://crates.io/ahash/0.7.6 \
    crate://crates.io/aho-corasick/0.7.20 \
    crate://crates.io/as_derive_utils/0.10.3 \
    crate://crates.io/autocfg/1.1.0 \
    crate://crates.io/base64/0.13.1 \
    crate://crates.io/bit_field/0.10.2 \
    crate://crates.io/bitflags/1.3.2 \
    crate://crates.io/bumpalo/3.12.0 \
    crate://crates.io/cc/1.0.79 \
    crate://crates.io/cfg-if/0.1.10 \
    crate://crates.io/cfg-if/1.0.0 \
    crate://crates.io/cglue-gen/0.2.7 \
    crate://crates.io/cglue-macro/0.2.3 \
    crate://crates.io/cglue/0.2.12 \
    crate://crates.io/coarsetime/0.1.23 \
    crate://crates.io/console/0.15.5 \
    crate://crates.io/core_extensions/1.5.3 \
    crate://crates.io/core_extensions_proc_macros/1.5.3 \
    crate://crates.io/crc32fast/1.3.2 \
    crate://crates.io/crossbeam-channel/0.5.7 \
    crate://crates.io/crossbeam-utils/0.8.15 \
    crate://crates.io/darling/0.13.4 \
    crate://crates.io/darling_core/0.13.4 \
    crate://crates.io/darling_macro/0.13.4 \
    crate://crates.io/dataview/0.1.2 \
    crate://crates.io/dataview/1.0.1 \
    crate://crates.io/dirs-sys/0.3.7 \
    crate://crates.io/dirs/4.0.0 \
    crate://crates.io/either/1.8.1 \
    crate://crates.io/encode_unicode/0.3.6 \
    crate://crates.io/fallible-iterator/0.2.0 \
    crate://crates.io/fixed-slice-vec/0.10.0 \
    crate://crates.io/flate2/1.0.25 \
    crate://crates.io/fnv/1.0.7 \
    crate://crates.io/form_urlencoded/1.1.0 \
    crate://crates.io/generational-arena/0.2.8 \
    crate://crates.io/getrandom/0.2.8 \
    crate://crates.io/goblin/0.5.4 \
    crate://crates.io/hashbrown/0.12.3 \
    crate://crates.io/ident_case/1.0.1 \
    crate://crates.io/idna/0.3.0 \
    crate://crates.io/indicatif/0.16.2 \
    crate://crates.io/instant/0.1.12 \
    crate://crates.io/itertools/0.10.5 \
    crate://crates.io/itoa/1.0.5 \
    crate://crates.io/js-sys/0.3.58 \
    crate://crates.io/lazy_static/1.4.0 \
    crate://crates.io/libc/0.2.139 \
    crate://crates.io/libloading/0.7.4 \
    crate://crates.io/lock_api/0.4.9 \
    crate://crates.io/log/0.4.17 \
    crate://crates.io/memchr/2.5.0 \
    crate://crates.io/memflow-derive/0.2.0-beta3 \
    crate://crates.io/memflow/0.2.0-beta9 \
    crate://crates.io/memmap/0.7.0 \
    crate://crates.io/miniz_oxide/0.6.2 \
    crate://crates.io/no-std-compat/0.4.1 \
    crate://crates.io/num_threads/0.1.6 \
    crate://crates.io/number_prefix/0.4.0 \
    crate://crates.io/once_cell/1.9.0 \
    crate://crates.io/parking_lot/0.11.2 \
    crate://crates.io/parking_lot_core/0.8.6 \
    crate://crates.io/paste/1.0.11 \
    crate://crates.io/pdb/0.7.0 \
    crate://crates.io/pelite-macros/0.1.1 \
    crate://crates.io/pelite/0.9.0 \
    crate://crates.io/percent-encoding/2.2.0 \
    crate://crates.io/plain/0.2.3 \
    crate://crates.io/proc-macro-crate/1.1.3 \
    crate://crates.io/proc-macro2/1.0.51 \
    crate://crates.io/progress-streams/1.1.0 \
    crate://crates.io/quote/1.0.23 \
    crate://crates.io/rangemap/1.3.0 \
    crate://crates.io/redox_syscall/0.2.16 \
    crate://crates.io/redox_users/0.4.3 \
    crate://crates.io/regex-syntax/0.6.28 \
    crate://crates.io/regex/1.7.1 \
    crate://crates.io/repr_offset/0.2.2 \
    crate://crates.io/ring/0.16.20 \
    crate://crates.io/rustc_version/0.2.3 \
    crate://crates.io/rustc_version/0.4.0 \
    crate://crates.io/rustls/0.20.8 \
    crate://crates.io/rustversion/1.0.11 \
    crate://crates.io/ryu/1.0.12 \
    crate://crates.io/scopeguard/1.1.0 \
    crate://crates.io/scroll/0.10.2 \
    crate://crates.io/scroll/0.11.0 \
    crate://crates.io/scroll_derive/0.11.0 \
    crate://crates.io/sct/0.7.0 \
    crate://crates.io/semver-parser/0.7.0 \
    crate://crates.io/semver/0.9.0 \
    crate://crates.io/semver/1.0.16 \
    crate://crates.io/serde/1.0.152 \
    crate://crates.io/serde_derive/1.0.152 \
    crate://crates.io/serde_json/1.0.93 \
    crate://crates.io/simplelog/0.12.0 \
    crate://crates.io/smallvec/1.10.0 \
    crate://crates.io/spin/0.5.2 \
    crate://crates.io/strsim/0.10.0 \
    crate://crates.io/syn/1.0.109 \
    crate://crates.io/termcolor/1.1.3 \
    crate://crates.io/thiserror-impl/1.0.38 \
    crate://crates.io/thiserror/1.0.38 \
    crate://crates.io/time-core/0.1.0 \
    crate://crates.io/time-macros/0.2.8 \
    crate://crates.io/time/0.3.20 \
    crate://crates.io/tinyvec/1.6.0 \
    crate://crates.io/tinyvec_macros/0.1.1 \
    crate://crates.io/toml/0.5.11 \
    crate://crates.io/tstr/0.2.3 \
    crate://crates.io/tstr_proc_macros/0.2.2 \
    crate://crates.io/typed-arena/2.0.2 \
    crate://crates.io/unicode-bidi/0.3.10 \
    crate://crates.io/unicode-ident/1.0.6 \
    crate://crates.io/unicode-normalization/0.1.22 \
    crate://crates.io/untrusted/0.7.1 \
    crate://crates.io/ureq/2.6.2 \
    crate://crates.io/url/2.3.1 \
    crate://crates.io/uuid/0.8.2 \
    crate://crates.io/version_check/0.9.4 \
    crate://crates.io/volatile/0.4.6 \
    crate://crates.io/wasi/0.11.0+wasi-snapshot-preview1 \
    crate://crates.io/wasm-bindgen-backend/0.2.81 \
    crate://crates.io/wasm-bindgen-macro-support/0.2.81 \
    crate://crates.io/wasm-bindgen-macro/0.2.81 \
    crate://crates.io/wasm-bindgen-shared/0.2.81 \
    crate://crates.io/wasm-bindgen/0.2.81 \
    crate://crates.io/web-sys/0.3.58 \
    crate://crates.io/webpki-roots/0.22.6 \
    crate://crates.io/webpki/0.22.0 \
    crate://crates.io/widestring/0.5.1 \
    crate://crates.io/winapi-i686-pc-windows-gnu/0.4.0 \
    crate://crates.io/winapi-util/0.1.5 \
    crate://crates.io/winapi-x86_64-pc-windows-gnu/0.4.0 \
    crate://crates.io/winapi/0.3.9 \
    crate://crates.io/windows-sys/0.42.0 \
    crate://crates.io/windows_aarch64_gnullvm/0.42.1 \
    crate://crates.io/windows_aarch64_msvc/0.42.1 \
    crate://crates.io/windows_i686_gnu/0.42.1 \
    crate://crates.io/windows_i686_msvc/0.42.1 \
    crate://crates.io/windows_x86_64_gnu/0.42.1 \
    crate://crates.io/windows_x86_64_gnullvm/0.42.1 \
    crate://crates.io/windows_x86_64_msvc/0.42.1 \
    crate://crates.io/x86_64/0.14.10 \
    git://github.com/slack2450/memflow-win32;protocol=https;nobranch=1;name=memflow-win32-defs;destsuffix=memflow-win32-defs \
    git://github.com/slack2450/memflow-win32;protocol=https;nobranch=1;name=memflow-win32;destsuffix=memflow-win32 \
"

SRCREV_FORMAT .= "_memflow-win32"
SRCREV_memflow-win32 = "0.2.0-beta9"
EXTRA_OECARGO_PATHS += "${WORKDIR}/memflow-win32"
SRCREV_FORMAT .= "_memflow-win32-defs"
SRCREV_memflow-win32-defs = "0.2.0-beta9"
EXTRA_OECARGO_PATHS += "${WORKDIR}/memflow-win32-defs"

# FIXME: update generateme with the real MD5 of the license file
LIC_FILES_CHKSUM = " \
    "

SUMMARY = "zynq-memflow"
HOMEPAGE = "https://github.com/slack2450/zynq-memflow.git"
LICENSE = "CLOSED"

# includes this file if it exists but does not fail
# this is useful for anything you may want to override from
# what cargo-bitbake generates.
include zynq-memflow-${PV}.inc
include zynq-memflow.inc
