import json
import uc
import sys

# print("OK?")
a = ''
c = ''
for i in range(len(sys.argv)):
    if i > 0:
        a += " " + sys.argv[i]

b = a.split(" ")
for i in range(len(b)):
    if b[i] != '':
        c += chr(int(b[i]))

emulator = uc.emu('./acl_fuzz.exe', c.encode("UTF-8"), [], emulator_base=0xbeef000)
# emulator = uc.emu('projects/CYW20735B1/gen/lmp_fuzz.exe', b'', [], emulator_base=0xbeef000)
emulator.run()

results = emulator.results

emulator.coverage_activity_json = json.dumps(emulator.coverage_activity)

my_need = {}
for r in results:
    my_need['ExitType'] = r['reason']
my_need['Coverage'] = len(emulator.coverage_activity_json)

js = json.dumps(my_need)
print(js)