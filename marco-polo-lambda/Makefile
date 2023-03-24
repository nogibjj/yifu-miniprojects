format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

release-arm:
	cargo lambda build --release --arm64

deploy:
	cargo lambda deploy --iam-role arn:aws:iam::969594881030:role/cargo-lambda-role-ee2b9a43-2712-4509-b856-ba3427a27b6b


invoke:
	cargo lambda invoke --remote \
  		--data-ascii '{"name": "Marco"}' \
  		--output-format json \
  		marco-polo-lambda

all: format lint test run