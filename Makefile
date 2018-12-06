# https://serverless.com/framework/docs/providers/aws/guide/credentials/
STAGE ?= "prod"

help:
	@echo Public targets:
	@grep -E '^[^_][^_][a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'
	@echo "Private targets: (use at own risk)"
	@grep -E '^__[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[35m%-20s\033[0m %s\n", $$1, $$2}'

dependencies: ## Install build dependencies
	@echo "installing dependencies..."
	@npm install --silent

package: dependencies ## Compile and package application
	@echo "packaging function..."
	@npx serverless \
		package \
		--stage $(STAGE)

deploy: ## Deploy application
	@echo "deploying function..."
	@npx serverless \
		deploy \
		--stage $(STAGE) \
		--conceal

destroy: ## Destroy application
	@echo "destroying function..."
	@npx serverless \
		remove \
		--stage $(STAGE) \

logs: ## Fetches a stream of logs
	@echo "fetching function logs..."
	@npx serverless \
		logs -f hello \
		--stage $(STAGE)

invoke: ## Invoke function remotely (requires deployment)
	@echo "invoking function..."
	@npx serverless \
		invoke -f hello \
		--stage $(STAGE)