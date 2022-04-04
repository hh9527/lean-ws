FROM scratch
COPY ./target/x86_64-unknown-linux-musl/release/exe1 /exe1
ENTRYPOINT ["/exe1"]
