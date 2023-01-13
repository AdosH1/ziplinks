# Description
Ziplinks is a web server that takes in web urls and combines them into a single shortened link.  
It can be used to share a batch of links easily, save workflows or whatever you can come up with.  
  

## Build / Run
You may build the server via Rust or using the docker file.  

Rust
```rust
cargo run
```

Docker
```docker
docker build -t ziplinks .
docker run --rm -d -p 7878:7878 ziplinks
```

## Todo
Ziplinks is a purpose built web server and has only the parts it needs written (as opposed to implemented the entire HTTP protocol).
There's a few things we need to do to make the code more sustainable / scalable in we were to extend this application.  

1. Static webpage generation (as opposed to just serving static webpage)
    - There's a mix of both currently and its best to choose a good way to generate and roll with it.
2. Database integration
    - For simplicity, links are currently stored in an in-memory cache, as the size of links is incredibly small.
    - In the future a database should be used to be able to run more instances of ziplinks and increase throughput
3. Better link primitives
    - Links should have meta data including timestamps etc for future clean up