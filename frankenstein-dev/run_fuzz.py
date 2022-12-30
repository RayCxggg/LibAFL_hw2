import json
from core import uc

emulator = uc.emu('projects/CYW20735B1/gen/acl_fuzz.exe', b'', [], emulator_base=0xbeef000)
# emulator = uc.emu('projects/CYW20735B1/gen/lmp_fuzz.exe', b'', [], emulator_base=0xbeef000)
emulator.run()

results = emulator.results

emulator.coverage_activity_json = json.dumps(emulator.coverage_activity)

my_need = {}
my_need['ExitKind'] = []
for r in results:
    my_need['ExitKind'].append(r['reason'])
my_need['Coverage'] = len(emulator.coverage_activity_json)

print(my_need)