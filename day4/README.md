This one was an interesting challenge

1 I decided now would be a good time explore parallelism and concurrency

-- Go is SOOO INCREDIBLY EASY to do concurrency
-- Rust isn't much harder, but Rust "breaks" once the amount of
threads requested gets to a ridiculous level
--- Note that Go starts crapping itself at ridiculous levels too
but at least the program doesn't crash like it does on rust
Performance is destroyed though, so there isn't any real reason to say
make 100000 "go routines"

--- Go is MUCH easier to use strings
---- The naive approach to Rust strings is not only harder to use
But has a SIGNIFICANT performance penalty.
I had to "cheat" and figure out why Rust's performance was so bad
turns out turning an integer into a string using .to_string() is 
super slow.
So I learned of a package called itoa, and this brings performance up to snuff

Note: Naive Rust was twice as slow as Go
Rust+itoa is twice as fast as Go

This 'day' really brings into focus what Go is good at.
It's easier to use concurrency (and that concurrency turns into parallelism by default)
and it's easier to use strings in Go.
In the end though, Rust was Faster

If memory needed to be shared between threads, I think that Rust would be superior
simply because Go doesn't have Enforced Mutexing