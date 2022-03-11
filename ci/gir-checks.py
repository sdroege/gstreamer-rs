from itertools import chain
import os
import sys
from pathlib import Path as P
from subprocess import check_call as exec

NATIVE_CRATES = ["gstreamer-utils"]

def git(*args):
    exec(["git"] + list(args))

def check_no_git_diff():
    git("diff", "--exit-code")

check_no_git_diff()
git("clone", "--depth", "1", "https://github.com/gtk-rs/checker")
check_no_git_diff()

rootdir = P(".")
checker_dir = P("checker")
with (checker_dir / "Cargo.toml").open("a") as f:
    f.write("[workspace]\n")

check_no_git_diff()
exec(['cargo', 'build', '--locked', '--color=always', '--release'], cwd=checker_dir)
check_no_git_diff()

exec('cargo run --color=always --release -- ../gstreamer* ../gstreamer-gl/{egl,wayland,x11}', cwd=checker_dir, shell=True)

gl_dir = rootdir / 'gstreamer-gl'
for crate in chain(rootdir.glob('gstreamer*'), [gl_dir / 'egl', gl_dir / 'wayland', gl_dir / 'x11']):
    # Ignore "native" crates
    if crate.name in NATIVE_CRATES:
        continue

    print(f'--> Checking doc aliases in {crate.absolute()}')
    exec(['python3', 'doc_aliases.py', crate.absolute()], cwd=checker_dir)

    print(f'--> {crate.absolute()}')
    exec(['./checker/check_init_asserts', crate.absolute()])

check_no_git_diff()
