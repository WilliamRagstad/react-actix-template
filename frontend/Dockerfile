# ============== BUILD IMAGE ==============
FROM node:20.13.1-slim as builder
WORKDIR /app
COPY . .
RUN npm install
RUN npm run build

# =========== PRODUCTION IMAGE ===========
FROM nginx:stable-alpine as production
COPY --from=builder /app/build /usr/share/nginx/html
COPY nginx/nginx.conf /etc/nginx/nginx.conf
EXPOSE 80

# Start Nginx when the container runs
CMD ["nginx", "-g", "daemon off;"]
