# ============== BUILD IMAGE ==============
FROM rust:1.78.0-buster as builder
ENV APP_BIN_NAME=your_app_name_here
WORKDIR /usr/src/${APP_BIN_NAME}
COPY . .

# Build the project
RUN cargo build --release

# =========== PRODUCTION IMAGE ===========
FROM debian:buster-slim as production
WORKDIR /usr/local/bin

# Install system dependencies for runtime libraries
RUN apt-get update && \
	apt-get install -y default-mysql-client default-libmysqlclient-dev ca-certificates dos2unix file && \
	rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/${APP_BIN_NAME}/target/release/${APP_BIN_NAME} .
COPY docker-entrypoint.sh .

# Make sure the entrypoint script is executable
RUN dos2unix ./docker-entrypoint.sh
RUN chmod +x ./docker-entrypoint.sh

EXPOSE 8080

ENTRYPOINT ["docker-entrypoint.sh"]
CMD ["${APP_BIN_NAME}"]
