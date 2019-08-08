host=nvidia@192.168.1.2
#cross build --target=aarch64-unknown-linux-gnu &&
    scp target/aarch64-unknown-linux-gnu/debug/{server,test_client,aimcjog} $host:~/
