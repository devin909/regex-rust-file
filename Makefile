all:
	rustc --opt-level=3 --lib lib.rs

test: 
	rust test lib.rs

run: all
	rustc test.rs -L .
	./test

com: all
	rustc test.rs -L .

clean:
	rm -r *~* test *.dSYM *.dylib 
