# Dockerized log multiplexer (staging)

Staging version of the log multiplexer.

## Build

The Makefile builds an image that can be run either locally (for debugging)
or in a Kubernetes cluster.

The build pulls from GitHub, to which SSH credentials must be made accessible,
as follows:
```
eval $(ssh-agent)
ssh-add /path/to/ssh/credentials
```
If upstream repos have to be re-downloaded, add `--no-cache`
to the Makefile's build command.

### Environment variables

Building expects the following env vars:
* `REGISTRY`: the desired Docker registry
* `IMAGE`: the desired Docker image name
* `TAG`: the image's desired Docker tag