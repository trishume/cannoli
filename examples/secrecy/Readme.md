# Trace bug repro

The `secrecy` binary is from DEF CON QUALS 2022. It uses self-modifying code to decrypt itself, which might have something to do with this issue or might not.

The GDB and Cannoli script simply trace two different program locations that based on reversing should be paired.
The GDB shows them paired as expected, but the Cannoli script misses one of the executions.

The code that produced the binary is open-source post-CTF [here](https://github.com/Nautilus-Institute/quals-2022/blob/main/secrecy/secrecy.c).

## Output with gdb

```
$ make gdb
0x2415
0x2427
0x2415
0x2427
0x2415
0x2427
0x2415
0x2427
```

## Output with cannoli

```
$ make cannoli_client
$ make cannoli
New client 0xb746d03b87659b64
0x2415
0x2427
0x2427
0x2415
0x2427
0x2415
0x2427
Lost client 0xb746d03b87659b64
```
