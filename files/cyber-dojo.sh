set -e

# --------------------------------------------------------------
# Text files under /sandbox are automatically returned...
source ~/cyber_dojo_fs_cleaners.sh
export REPORT_DIR=${CYBER_DOJO_SANDBOX}/report
function cyber_dojo_enter()
{
  # 1. Only return _newly_ generated reports.
  cyber_dojo_reset_dirs ${REPORT_DIR}
}
function cyber_dojo_exit()
{
  # 2. Remove text files we don't want returned.
  cyber_dojo_delete_dirs target # ...
  cyber_dojo_delete_files Cargo.lock  # ...
}
cyber_dojo_enter
trap cyber_dojo_exit EXIT SIGTERM

# ------------------------------------------------------------------------
PATH=/usr/local/cargo/bin:${PATH}
#RUST_BACKTRACE=1

rm -rf report || true
mkdir report 2> /dev/null || true
cargo test --features "strict" -- --nocapture
cargo clippy 2>> ./report/style.txt
cargo fmt
