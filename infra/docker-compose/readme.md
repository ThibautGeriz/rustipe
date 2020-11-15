# Deploy using docker

SSL conf is not included, do not use as is in production.

## Requirements

- [Docker](https://www.docker.com/)
- [Docker compose](https://docs.docker.com/compose/)

## command

This is for a local config. Make sure to update the config for other env. Especially for production.

```sh
# export AWS secrets
export AWS_ACCESS_KEY_ID=Xblabla
export AWS_SECRET_ACCESS_KEY=blabla

# Build it
docker-compose --env-file ./config/.env.local build

# Run it
docker-compose --env-file ./config/.env.local up
```
