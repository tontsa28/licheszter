#!/bin/sh

mkdir berserk
cd berserk
git clone https://github.com/lichess-org/berserk .
git clone https://github.com/lichess-org/lila-docker
mv lila-docker/scripts/ .
rm -rf lila-docker
sed -i 's/nginx/lila:9663/g' scripts/berserk-connect-bots.py
pip install -e .
python scripts/berserk-connect-bots.py
