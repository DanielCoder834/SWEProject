server: 
	cd ./src/backend; \
	openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj /CN=localhost; \
	cargo build --release; printf '\n' | cargo run --release;

server-test:
	cd ./src/backend; \
    	openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj /CN=localhost; \
    	cargo test;

server-test-coverage:
	cd ./src/backend; \
        	openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj /CN=localhost; \
        	cargo llvm-cov;

server-docker:
	docker compose -f "docker-compose.yaml" up -d --build