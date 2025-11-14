.PHONY:install
install:
	cargo install --path app --force

.PHONY: check
check:
	cargo clippy --all -- -W clippy::all -W clippy::pedantic -W clippy::nursery -A clippy::used_underscore_binding -A clippy::cast_sign_loss -A clippy::cast_possible_truncation
