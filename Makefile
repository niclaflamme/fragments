.PHONY: new serve compile rename dev permissions build

new:
	@./scripts/new_post.sh

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
