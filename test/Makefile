
test:
	cd ../ && cargo build --release
	gcc test.c -o test
	LD_PRELOAD=../target/release/libfeelbad.so ./test