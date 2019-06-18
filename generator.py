#!/usr/bin/env python

from os import listdir
from os.path import isfile, join
from subprocess import call
import sys

need_rebuild = False

def update_workspace():
    with open('Cargo.toml', 'r') as f:
        old_lines = f.readlines()
    lines = old_lines[:]
    with open('Cargo.toml', 'w') as f:
        lines.insert(len(lines) - 2, '"gir",')
        f.write(''.join(lines))
    success = True
    try:
        call(['bash', '-c', 'cd gir && cargo build --release'])
    except:
        success = False
    with open('Cargo.toml', 'w') as f:
        f.write(''.join(old_lines))
    return success


if not isfile('./gir/src'):
    need_rebuild = True
    print('=> Initializing gir submodule...')
    call(['bash', '-c', 'git submodule update --init'])
    print('<= Done!')

question = 'Do you want to update gir submodule? [y/N]'
if sys.version_info[0] < 3:
    line = raw_input(question)
else:
    line = input(question)
line = line.strip()
if line.lower() == 'y':
    need_rebuild = True
    print('=> Updating gir submodule...')
    call(['bash', '-c', 'cd gir && git reset --hard HEAD && git pull -f origin master'])
    print('<= Done!')

if need_rebuild is True or not os.path.isfile('./gir/target/release/gir'):
    print('=> Building gir...')
    if update_workspace() is True:
        print('<= Done!')
    else:
        print('<= Failed...')
        sys.exit(1)

print('=> Regenerating crates...')
for entry in [f for f in listdir('.') if isfile(join('.', f))]:
    if entry.startswith('Gir_Gst') and entry.endswith('.toml'):
        print('==> Regenerating "{}"...'.format(entry))
        call(['./target/release/gir', '-c', entry])
        print('<== Done!')
print('<= Done!')
print("Don't forget to check if everything has been correctly generated!")
