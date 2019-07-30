host=team955@192.168.1.109
cross build --target=aarch64-unknown-linux-gnu &&
    scp target/aarch64-unknown-linux-gnu/debug/aimcjog $host:~/ &&
    ssh -t $host ./aimcjog 19
