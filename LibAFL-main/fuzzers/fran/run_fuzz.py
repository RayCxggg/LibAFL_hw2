import json
import uc

# print("OK?")

emulator = uc.emu('./acl_fuzz.exe', b'', [], emulator_base=0xbeef000)
# emulator = uc.emu('projects/CYW20735B1/gen/lmp_fuzz.exe', b'', [], emulator_base=0xbeef000)
emulator.run()

results = emulator.results

emulator.coverage_activity_json = json.dumps(emulator.coverage_activity)

my_need = {}
for r in results:
    my_need['ExitKind'] = r['reason']
my_need['Coverage'] = len(emulator.coverage_activity_json)

js = json.dumps(my_need)
print(js)