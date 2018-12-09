# Concurrency project
cargo run to execute src/main.rs

## TODO:
see src/main.rs

Add procedural generation of bodies

Add metric tracking (total run time, per cpu time spent on delays, cpu idle time, etc)

Create two work distribution methods:
1. Send work to each cpu and wait for it to come back before sending more
2. Estimate performance of each cpu and send work trying to minimize delays
