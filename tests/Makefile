
tests: gu-client.tests

build:
	docker-compose build

gu-client.tests: build
	docker-compose up -d
	HUB_ADDR=$$(docker-compose exec hub gu-hub --json lan list -I hub | jq -r '.[0].Addresses'); \
		 echo $$HUB_ADDR; \
		 docker-compose exec prov gu-provider hubs connect -S $${HUB_ADDR}
	docker-compose exec hub gu-hub --json peer list | jq '.[0]'
	docker-compose exec hub /bin/bash -c "cd ../gu-client && cargo test --features integration_tests -- --nocapture"



.PHONY: tests
