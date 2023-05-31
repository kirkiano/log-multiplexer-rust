#!make

include .env
export $(shell sed 's/=.*//' .env)


.PHONY: build release run clippy test unit_tests clean doc


build:
	cargo build

clippy:
	cargo clippy

release:
	cargo build --release

run: args
	cargo run -- --listen-locally

# If TEST is not defined, all tests will be run.
# --nocapture allows output to be displayed
test:
	cargo test -- --nocapture $(TEST)

# If TEST is not defined, all tests will be run.
# --nocapture allows output to be displayed
unit_tests:
	cargo test --lib -- --nocapture

clean:
	cargo clean

doc:
	cargo doc

############################################################

args: port mongo

############################################################

port:
ifndef PORT
	$(error PORT not defined)
endif

############################################################

mongo: mongo_uri

mongo_uri:
ifndef MONGO_URI
	$(error MONGO_URI not defined)
endif