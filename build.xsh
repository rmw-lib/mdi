#!/usr/bin/env xonsh

# import lzma
import tomli
from os.path import dirname,abspath,exists
from fire import Fire
import platform
from humanize import naturalsize
import os

PWD = dirname(abspath(__file__))

cd @(PWD)

p".xonshrc".exists() && source .xonshrc

system = platform.system().lower()
if system == 'darwin':
  system = f'apple-{system}'
elif system == 'linux':
  system = 'unknown-linux-gnu'

$RUSTFLAGS="-C target-feature=+crt-static -C link-self-contained=yes"
# -l static=stdc++"

# x86_64-unknown-linux-gnu
# system = 'unknown-linux-gnu'

TARGET=f'{platform.machine()}-{system}'

@Fire
def main():
  with open(join(PWD,"Cargo.toml"),"rb") as f:
    toml = tomli.load(f)

  app = toml['package']['name']

  cargo build \
  --release \
  --target @(TARGET) \
  -Z build-std=std,panic_abort

# -Z build-std-features=panic_immediate_abort

  out=f"target/{TARGET}/release/{app}"
  strip @(out)

  ./sh/upx.sh
  upx --best --lzma @(out)

#  with open(out,'rb') as f:
#    with lzma.open(out+'.xz','wb') as o:
#      o.write(f.read())

  stat = os.stat(out)
  print(naturalsize(stat.st_size))
  print(out)
