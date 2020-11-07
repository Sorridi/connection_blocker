# connection_blocker
Connection blocker / blacklister against ssh bruteforce, made in Rust.

As it uses iptables, it will work only on linux.

Demo: https://youtu.be/mpgj-rdT-A8

Compile the project with: cargo build --release

The file needs 3 arguments separed with a whitespace: 
  1) Auth.log file (on debian /var/log/auth.log)
  2) Chain name
  3) Table name
  4) Time (in seconds) of pause to unblock connections that have 0 bytes sent

Example:
  1) iptables -t mangle -N BLOCKED -p tcp --dport 22
  2) iptables -t mangle -A PREROUTING -j BLOCKED
  3) ./connection_blocker /var/log/auth.log BLOCKED mangle 84600
