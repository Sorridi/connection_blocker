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

Setup example:
  1) iptables -t mangle -N PERMITTED -p tcp --dport 22
  2) iptables -t mangle -A PERMITTED -s <YOUR_IP> -j ACCEPT
  3) iptables -t mangle -A PREROUTING -j PERMITTED
  4) iptables -t mangle -N BLOCKED -p tcp --dport 22
  5) iptables -t mangle -A PREROUTING -j BLOCKED
  
BEFORE RUNNING THE TOOL, PLEASE BE SURE TO PUT YOUR IP THE "PERMITTED" CHAIN, OR WHITELIST IT IN THE ./connection_blocker_data/whitelist_ips.txt THAT WILL BE CREATED THE SECOND TIME YOU EXECUTE THE TOOL.
  
  6) ./connection_blocker /var/log/auth.log BLOCKED mangle 84600
