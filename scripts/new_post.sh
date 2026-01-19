#!/usr/bin/env sh
set -eu

draft=false
title="New"

if [ "${1:-}" = "--draft" ]; then
	draft=true
	shift
fi

if [ "${1:-}" != "" ]; then
	title="$1"
fi

posts_dir="posts"
mkdir -p "$posts_dir"

slug="$(printf "%s" "$title" | tr 'A-Z' 'a-z' | sed -E 's/[^a-z0-9]+/_/g; s/^_+//; s/_+$//')"
date="$(date +%Y-%m-%d)"
file="${posts_dir}/${date}-${slug}.md"

if [ -e "$file" ]; then
	echo "File already exists: $file"
	exit 1
fi

cat > "$file" <<EOF
Title: ${title}
Date: ${date}
Subtitle:
Slug:
Draft: ${draft}

---

Start typing on this line
EOF

echo "Created $file"
