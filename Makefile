server: 
	cd ./src/backend; \
	openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj /CN=localhost; \
	cargo build; printf '\n' | cargo run;

server-test:
	cd ./src/backend; \
    	openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj /CN=localhost; \
    	cargo test;

server-test-coverage:
	cd ./src/backend; \
        	openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj /CN=localhost; \
        	cargo llvm-cov;