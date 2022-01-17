curl -o ./mdbook.tar.gz https://github.com/rust-lang/mdBook/releases/download/v0.4.15/mdbook-v0.4.15-x86_64-unknown-linux-gnu.tar.gz && \
	tar -xf ./mdbook.tar.gz && \
	./mdbook build ./lavender-book
