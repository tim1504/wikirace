#!/bin/bash
mkdir data
cd data

echo "Downloading dumps..."
wget \
https://dumps.wikimedia.org/dewiki/latest/dewiki-latest-page.sql.gz \
https://dumps.wikimedia.org/dewiki/latest/dewiki-latest-pagelinks.sql.gz

echo "Unzipping dumps..."
gzip -d *.gz

echo "Creating csv..."
# Very hacky, but works
grep -E -o "\(.*?\)[,|;]" dewiki-latest-page.sql \
| sed -E -n "s/^\(([0-9]+),0,'([^']*)',([0-9]+).*$/\1 \3 \2/p" > page.txt
rm dewiki-latest-page.sql

grep -E -o "\(.*?\)[,|;]" dewiki-latest-pagelinks.sql \
| sed -E -n "s/^\(([0-9]+),0,'([^']*)',0.*$/\1 \2/p" > pagelinks.txt
rm dewiki-latest-pagelinks.sql

echo "Done."
