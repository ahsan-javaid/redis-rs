### Description
Redis server written in rust by following rust standard library. 

![Screenshot](https://avatars.githubusercontent.com/u/1529926?s=200&v=4)

### How to test

* ping
```
nc localhost 6379

ping
+PONG
```
* parallel ping
```
echo -e "ping\nping" | redis-cli
```

### How to run 
```
cargo run 
```

### Tests
To be added

### Contributions
Feel free to contribute and send pull request

### Architecture
- Based on rust standard library