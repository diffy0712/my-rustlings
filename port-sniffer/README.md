# Open Port Sniffer

Multithreaded port sniffer with nice cli using clap

## TODOs
[x] read arguments from cli using [clap](https://github.com/clap-rs/clap)
   - [x] ip address
   - [x] start_port
   - [x] end_port
   - [ ] validate if start and end ports are not out of range

[ ] try to connect to the port using TcpStream::connect
   - [ ] add multithreading using [tokio](https://tokio.rs/)


[ ] unit tests