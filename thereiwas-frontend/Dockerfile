# ------------------------------------------------------------------------------
# NodeJS Build Stage
# ------------------------------------------------------------------------------
FROM node:23-alpine AS node-build

WORKDIR /usr/src/app

COPY thereiwas-frontend /usr/src/app

RUN npm ci

RUN npm run build

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------
FROM nginx:alpine

COPY --from=node-build --chown=1001 /usr/src/app/build /usr/share/nginx/html
