FROM quay.io/fedora/fedora-minimal:40 as builder
WORKDIR /code
COPY . .
RUN microdnf update -y && \
    microdnf install -y npm rust cargo openssl-devel git-core && \
    microdnf clean all
RUN cd public && NODE_ENV=production npm run css && cd ..
RUN cargo build -r

FROM quay.io/fedora/fedora-minimal:40
WORKDIR /srv/masto-thread-renderer
RUN microdnf update -y && microdnf clean all
COPY --from=builder /code/Rocket.toml .
COPY --from=builder /code/public ./public
COPY --from=builder /code/target/release/masto-thread-renderer /usr/local/bin/
EXPOSE 8080
ENV ROCKET_CONFIG=/srv/masto-thread-renderer/Rocket.toml
ENV ROCKET_PROFILE=release
CMD ["/usr/local/bin/masto-thread-renderer"]
