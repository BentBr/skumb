## Local setup and installation
Copy `.env.example` to `.env` and adjust the values to your needs and see Makefile for available commands.

```bash
cp .env.example .env
make help
```

### Rust
### Diesel
- `diesel migration generate create_to_do_items` Using diesel to create a new migration
- Running the migration: `diesel migration run`
- Undoing and running again the migration: `diesel migration redo`
### Docker (compose)


https://9hzofcbnk1.execute-api.eu-central-1.amazonaws.com/
https://9hzofcbnk1.execute-api.eu-central-1.amazonaws.com/test


Todos:
- clippy strict
- add tests
- add chat entity
- add BE chat uri + web transport connection
- add auth layer to endpoints (current an auth user can query all endpoints)


## Building Dockerfiles
### BE
`docker build -t skumb .` \
`docker run --env-file .env -p 9123:9123 -p 9124:9124 skumb`

### FE
`cd fe` \
`docker build -t skumb-fe .` \
`docker run -p 81:80 skumb-fe`



## AWS manual deployment of new (backend) Docker image

Authenticate Docker to your ECR \
`aws ecr get-login-password --profile rust --region eu-central-1 | docker login --username AWS --password-stdin 767398008502.dkr.ecr.eu-central-1.amazonaws.com` \
(ask for rust profile in aws creds helper)

Build and push your Docker image \
Building: `docker build -t skumb .` \
Tagging: `docker tag skumb:latest 767398008502.dkr.ecr.eu-central-1.amazonaws.com/skumb:latest` \
Pushing: `docker push 767398008502.dkr.ecr.eu-central-1.amazonaws.com/skumb:latest`

## AWS manual deployment of new (frontend) assets (S3 static hosting)

Build new assets \
`docker compose run --rm -e VITE_API_URL=api.skumb.eu -e VITE_URL=skumb.eu -e VITE_HTTP_PROTOCOL=https -e VITE_WS_PROTOCOL=wss node yarn build`

Pushing to S3 \
`aws s3 sync fe/dist s3://skumb.eu --profile rust`

Invalidate Cloudfront cache \
`aws cloudfront create-invalidation --distribution-id EJ5ATN9KG4YOI --paths "/*" --profile rust --no-cli-pager`

Todos:
- pretty fe
  - ~~new chat (from existing)
    (routing strategy?)~~
  - ~~container size mobile~~
  - content
  - colours + styling
  - ~~chat scrolls to bottom~~
- new FE design + proper styles
- CI for BE + FE (remember the base uri)
  - ~~adding cloudfront invalidation~~
  - fixing pipelines in general
- Functions
  - auth with stored chats
  - different chat levels
    - 1:1
    - 1:1 stored
    - chat with email for account holder (entity)
- ~~Multi lang with i18n~~
- avoid clashing names (when both chats share same name)
