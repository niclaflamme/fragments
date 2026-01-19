.PHONY: new draft serve compile rename dev permissions build build\:railway run\:railway

new:
	@./scripts/new_post.sh

draft:
	@./scripts/new_post.sh --draft

serve:
	@cargo run

dev:
	@cargo watch -x run --watch posts --watch src

build:
	@cargo run -- --export

compile:
	@./scripts/regen_filenames.sh

rename:
	@./scripts/regen_filenames.sh

permissions:
	@chmod +x scripts/*.sh

build\:railway:
	@docker build -t blog-railway .

run\:railway:
	@docker run --rm -p 3000:3000 -e PORT=3000 blog-railway
