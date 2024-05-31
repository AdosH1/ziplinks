build:
	cargo build --release

image:
	docker build -t ziplinks .

run:
	docker run --restart always -d -p 7878:8080 ziplinks