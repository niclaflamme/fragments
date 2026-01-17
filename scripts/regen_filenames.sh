#!/usr/bin/env sh
set -eu

posts_dir="posts"

if [ ! -d "$posts_dir" ]; then
	echo "No $posts_dir directory found."
	exit 0
fi

for file in "$posts_dir"/*.md; do
	[ -e "$file" ] || continue
	if [ "$(basename "$file")" = "example.md" ]; then
		continue
	fi

	title="$(awk -F ': ' '/^Title: /{print $2; exit}' "$file")"
	date="$(awk -F ': ' '/^Date: /{print $2; exit}' "$file")"
	raw_slug="$(awk -F ': ' '/^Slug: /{print $2; exit}' "$file")"

	if [ -z "$title" ] || [ -z "$date" ]; then
		echo "Skipping (missing Title or Date): $file"
		continue
	fi

	case "$raw_slug" in
		*"<"*|*">"*)
			raw_slug=""
			;;
	esac

	if [ -n "$raw_slug" ]; then
		slug_source="$raw_slug"
	else
		slug_source="$title"
	fi
	slug="$(printf "%s" "$slug_source" | tr 'A-Z' 'a-z' | sed -E 's/[^a-z0-9]+/_/g; s/^_+//; s/_+$//')"
	new_name="${posts_dir}/${date}-${slug}.md"

	if [ "$file" = "$new_name" ]; then
		continue
	fi

	if [ -e "$new_name" ]; then
		echo "Target exists, skipping: $new_name"
		continue
	fi

	mv "$file" "$new_name"
	echo "Renamed: $file -> $new_name"
done
