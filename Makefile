build:
	cargo build --release

image:
	docker build --platform linux/amd64 -t ziplinks .

run:
	docker run --restart always -d -p 7878:80 ziplinks