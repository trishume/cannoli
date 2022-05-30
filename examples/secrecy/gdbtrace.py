class Tracepoint(gdb.Breakpoint):
    def __init__(self, base, location):
        super().__init__(f"*{hex(base+location)}", gdb.BP_BREAKPOINT, internal=True)
        self.loc_s = location
        return

    def stop(self):
        print(hex(self.loc_s))
        return False

def base_offset():
    import re
    maps = gdb.execute("info proc map", to_string=True)
    base = re.search(r"0x([0-9a-f]+)", maps)
    return int(base.group(1), 16)

# Tracepoint()
gdb.execute('starti')
b = base_offset()
Tracepoint(b,0x2415)
Tracepoint(b,0x2427)
gdb.execute('c')
