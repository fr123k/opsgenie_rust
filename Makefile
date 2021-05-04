FILE:=main

build:
	cargo build

exec: build
	cargo run

clean:
	rm -rf ./target/*
	
vagrant:
	vagrant up
	vagrant ssh

openapi:
	rm -rf ./out
	docker run --rm \
		-v ${PWD}:/local openapitools/openapi-generator-cli generate \
		-i /local/swagger.json \
		-g rust \
		-v --skip-validate-spec \
		-c /local/generator.json \
		-o /local/generated/rust 

openapi-shell:
	docker run -it --rm \
		-v ${PWD}:/local openapitools/openapi-generator-cli bash

# node ./multi-file-swagger/index.js swagger.yaml schedule scheduleRotation scheduleOverride > swagger.json

openapi-rust:
	cp -f MakeFile_openapi ./generated/rust/Makefile
	$(MAKE) -C ./generated/rust/ build