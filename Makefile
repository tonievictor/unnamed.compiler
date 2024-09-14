build: 
	cargo build
test: build
	tests/test_compiler target/debug/unnamed --chapter 1 --stage lex
