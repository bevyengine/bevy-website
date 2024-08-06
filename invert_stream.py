import sys
import re

def expand_hex(hexcode): return (''.join([c * 2 for c in hexcode]) if len(hexcode) in (3, 4) else hexcode)
def invert_hex(hexcode): return '{:06X}'.format(0xFFFFFF - int(hexcode, 16))

if __name__ == "__main__":
  pattern = re.compile(r'#([0-9a-fA-F]{3}([0-9a-fA-F]{1,5})?)\b')
  sys.stdout.write(
    pattern.sub(
      lambda mm: '#' + invert_hex(expand_hex(mm.group(1))),
      sys.stdin.read()
    )
  )
