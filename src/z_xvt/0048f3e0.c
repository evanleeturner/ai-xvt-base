/* SPDX-License-Identifier: MIT */
/* z_xvt 0048f3e0 */
extern uint _DAT_0052a560;
extern char DAT_0052a55c;
extern uint DAT_00625738[0x8000];
extern uint DAT_00605738[0x8000];

void *_malloc(uint);

int __cdecl FUN_0048f3e0(uint param_1, uint param_2, int param_3)
{
  uint *puVar1;
  uint uVar2;
  ushort uVar3;
  ushort uVar4;
  uint uVar5;
  
  uVar4 = 0;
  _DAT_0052a560 = _DAT_0052a560 + 1;
  if (DAT_0052a55c == '\0') {
    do {
      uVar3 = uVar4 + 1;
      DAT_00625738[uVar4] = 0;
      DAT_00605738[uVar4] = 0;
      uVar4 = uVar3;
    } while (uVar3 < 0x8000);
    DAT_0052a55c = '\x01';
  }
  uVar5 = 0;
  do {
    if (DAT_00625738[uVar5] == 0) break;
    uVar4 = (ushort)uVar5 + 1;
    uVar5 = (uint)uVar4;
  } while (uVar4 < 0x8000);
  if ((short)uVar5 == -0x8000) {
    return 0;
  }
  puVar1 = (uint *)_malloc(param_1);
  DAT_00625738[uVar5] = (uint)puVar1;
  if (puVar1 != (uint *)0x0) {
    if (param_3 != 0) {
      for (uVar2 = param_1 >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
        *puVar1 = 0;
        puVar1 = puVar1 + 1;
      }
      for (uVar2 = param_1 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
        *(char *)puVar1 = 0;
        puVar1 = (uint *)((int)puVar1 + 1);
      }
    }
    DAT_00605738[uVar5] = param_1;
    return uVar5 + 1;
  }
  return 0;
}
