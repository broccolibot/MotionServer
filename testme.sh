host=team955@10.42.0.114
cross build --target=aarch64-unknown-linux-gnu &&
    scp target/aarch64-unknown-linux-gnu/debug/aimcjog $host:~/
    #ssh -t $host ./aimcjog 19
