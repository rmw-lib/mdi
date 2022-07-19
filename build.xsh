#!/usr/bin/env xonsh

# import lzma
import tomli
from os.path import dirname,abspath,exists
from fire import Fire
import platform
from humanize import naturalsize
from os import stat,makedirs,replace

PWD = dirname(abspath(__file__))

cd @(PWD)

p".xonshrc".exists() && source .xonshrc

system = platform.system().lower()
ext = ''

if system == 'darwin':
  os = f'{platform.machine()}-apple-{system}'
elif system == 'windows':
  os = 'x86_64-pc-windows-msvc'
  ext = '.exe'
elif system == 'linux':
  os = f'{platform.machine()}-unknown-linux-gnu'

$RUSTFLAGS="-C target-feature=+crt-static -C link-self-contained=yes"

# -l static=stdc++"

TARGET=f'{os}'

@Fire
def main():
  with open(join(PWD,"Cargo.toml"),"rb") as f:
    toml = tomli.load(f)

  app = toml['package']['name']+ext

  cargo build \
  --release \
  --target @(TARGET) \
  -Z build-std=std,panic_abort \
  -Z build-std-features=panic_immediate_abort

  out=f"target/{TARGET}/release/{app}"
  strip @(out)

  if system!='windows':
    ./sh/upx.sh

  upx --best --lzma @(out)

#  with open(out,'rb') as f:
#    with lzma.open(out+'.xz','wb') as o:
#      o.write(f.read())
  print(naturalsize(stat(out).st_size))

  dir = 'target/bin'
  makedirs(join(PWD,dir),exist_ok=True)

  bin = join(dir,f"{os}-{app}")
  replace(out,bin)

  print(bin)
