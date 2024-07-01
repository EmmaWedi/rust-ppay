setup:
	cargo init

install:
	cargo add actix-web actix-http chrono futures-util mongodb actix-cors jsonwebtoken config uuid redis regex rand reqwest anyhow env_logger log serde_json sha2 md5 cbc base64
	cargo add serde --features derive

run:
	docker-compose up -d
	cargo run

stop:
	docker-compose down

watch:
	docker-compose up -d
	cargo watch -c -w src -x run

build: 
	cargo build