# Global
BASE_DIR=$(shell pwd)
UNAME_S=$(shell uname -s)
STAGE=${USER}

AWS_CLI=aws
TERRAFORM=terraform -chdir=./infrastructure

CROSS_TARGET=x86_64-unknown-linux-musl
CROSS_COMPILE=x86_64-linux-musl-

# Tasks

.PHONY: deploy

# Build Locally
build: 
	@cargo build 

test:
	@cargo test

#  Terraform
plan:
	@$(TERRAFORM) plan

terraform.init:
	@$(TERRAFORM) init

deploy:
	@$(TERRAFORM) apply -auto-approve

remove:
	@$(TERRAFORM) destroy -auto-approve


release:
ifeq ("$(UNAME_S)","Linux")
	@cargo build --target=$(CROSS_TARGET) --release
else
	@CROSS_COMPILE=$(CROSS_COMPILE) cargo build --target=$(CROSS_TARGET) --release
endif

package.graphql: 
	@mkdir -p deploy/graphql
	@cp target/$(CROSS_TARGET)/release/graphql deploy/graphql/bootstrap
	# @upx -9 deploy/graphql/bootstrap
	@zip -j -9 deploy/graphql.zip deploy/graphql/bootstrap

package: package.graphql

release.package.deploy: release package deploy

tail.graphql:
	@LOG_GROUP_NAME=$(shell $(TERRAFORM) output graphql_lambda_log_group); \
	$(AWS_CLI) logs tail $$LOG_GROUP_NAME --follow --format short
