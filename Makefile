all: 0.png
.PHONY: all

target/debug/helmod-planner: src/**
	cargo build

0.dot: target/debug/helmod-planner recipe.json yellow_science_complete.txt.b64
	target/debug/helmod-planner --recipes recipe.json --helmod yellow_science_complete.txt.b64

0.png: 0.dot
	dot -Tpng 0.dot -o 0.png
