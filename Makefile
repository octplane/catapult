target/release/catapult:
	docker build -t octplane/catapult:latest .
clean:
	rm -rf target
doc: target/doc/catapult/index.html
	cargo doc
upload_doc: doc
	rsync -az target/doc/ oct@zoy.org:~/public_html/public/doc/
