#!make

include .env
export $(shell sed 's/=.*//' .env)


IMAGE_FULL=$(REGISTRY)/$(IMAGE):$(TAG)

.PHONY: build run tag db db_path container runtime_env

# SSH_AUTH_SOCK defined via 'eval $(ssh-agent)'. But this also wipes
# out already ssh-add'd identities, so the latter will then have to
# be re-ssh-added.
#
# https://vsupalov.com/docker-buildkit-features/ discusses BuildKit.
build: build_env
	DOCKER_BUILDKIT=1 \
	docker build --ssh default=${SSH_AUTH_SOCK} \
	--tag $(IMAGE_FULL) \
	-f Dockerfile \
	../..

push:
	docker push $(IMAGE_FULL)

rmi: build_env
	docker rmi $(IMAGE_FULL)

###########################################################

build_env: registry image tag

###########################################################

registry:
ifndef REGISTRY
	$(error REGISTRY not defined)
endif

image:
ifndef IMAGE
	$(error IMAGE not defined)
endif

tag:
ifndef TAG
	$(error TAG not defined)
endif