FROM node:18-slim AS deps
ENV APP_ROOT /app
WORKDIR $APP_ROOT
COPY package.json package-lock.json ./
RUN npm ci
COPY . ./

FROM node:18-slim AS staging
WORKDIR $APP_ROOT
COPY --from=deps $APP_ROOT ./
ENV NODE_ENV production
RUN npm run build
EXPOSE 3000

FROM node:18-slim AS production
WORKDIR $APP_ROOT
COPY --from=deps $APP_ROOT ./
ENV NODE_ENV production
RUN npm run build
EXPOSE 3000
CMD ["npm", "start"]
