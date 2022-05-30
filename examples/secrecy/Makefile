cannoli:
	~/misc/QEMU/build/qemu-x86_64 -cannoli /home/tristan/Box/Dev/Contests/contests/dcquals22/cannoli/target/release/libcannoli_server.so ./secrecy

cannoli_client:
	cargo run --release

gdb:
	gdb ./secrecy -ex 'source gdbtrace.py' -batch
