APP=day4

.PHONY: build
build:
	@go build -o $(APP) src/*.go

.PHONY: debug-build
	@go build -gcflags '-N -l' -o $(APP) src/*.go

.PHONY: run
run: build
	@./$(APP)

.PHONY: test
test:
	go test src/*.go

.PHONY: bench
bench:
	go test src/*.go -run=XXX -bench=.

.PHONY: clean
clean:
	-rm $(APP)