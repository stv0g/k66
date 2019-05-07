.PHONY: default
default: all;

.PHONY: clean
clean:
	rm -rf src

.PHONY: setup
setup:
	cargo install --force svd2rust form
	rustup target add thumbv7em-none-eabihf
	rustup component add rustfmt

.PHONY: generate
generate: clean
	echo "\`\`\`bash" > WARNINGS.md
	svd2rust -i MK66F18.svd 2>> WARNINGS.md
	echo "\`\`\`" >> WARNINGS.md
	form -i lib.rs -o src/ && rm lib.rs
	cargo fmt

.PHONY: package
package:
	cargo package

.PHONY: examples
examples:
	(cd examples/flash_led_lptmr; cargo build)

check_master:
	@echo "check if on master"
	git branch | grep \* | cut -d ' ' -f2 | grep -q master

no_diff:
	@echo "check if master is even with origin"
	git diff -s --exit-code origin/master

check_tag:
	@echo "check if HEAD is tagged"
	git describe --tags --exact-match HEAD

.PHONY: publish
publish: check_master no_diff check_tag
	cargo publish

.PHONY: all
all: setup generate package
