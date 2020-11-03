# connection_blocker
Connection blocker / blacklister against ssh bruteforce, made in Rust.

Demo: https://youtu.be/mpgj-rdT-A8

Compile the project with: cargo build --release

The file needs 3 arguments separed with a whitespace: 
  1) Auth.log file (on debian /var/log/auth.log)
  2) Chain name
  3) Table name

Example: ./connection_blocker /var/log/auth.log PREROUTING mangle
