.PHONY: build
build:
	@go build -o day1 src/*.go

.PHONY: run
run: build
	@./day1

.PHONY: test
test:
	go test src/*.go

.PHONY: bench
bench:
	go test src/*.go -run=XXX -bench=.

.PHONY: clean
clean:
	-rm day1