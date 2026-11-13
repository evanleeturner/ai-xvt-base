/* SPDX-License-Identifier: MIT */
/* z_bop 0046fee0 */
DWORD __stdcall timeGetTime(void);

extern uint DAT_0099f6e0;
extern int DAT_00999be8;
extern int DAT_00999bec;
extern int DAT_00999bf0;
extern int DAT_00999bf4;
extern int DAT_00999bf8;
extern int DAT_00999bfc;
extern int DAT_00999c00;
extern int DAT_00999c04;
extern int DAT_00999c08;
extern int DAT_00999e08;
extern int DAT_00999e0c;
extern int DAT_00999e10;
extern int DAT_00999e14;

uint __cdecl FUN_0046fee0(int param_1)
{
  int iVar1;
  int *piVar2;
  DWORD DVar3;
  uint uVar4;
  
  uVar4 = 0;
  if (DAT_0099f6e0 != 0) {
    piVar2 = &DAT_00999be8;
    do {
      if (*piVar2 == param_1) {
        return uVar4;
      }
      piVar2 = piVar2 + 0x8e;
      uVar4 = uVar4 + 1;
    } while (uVar4 < DAT_0099f6e0);
  }
  if ((DAT_0099f6e0 == uVar4) && (uVar4 < 0x28)) {
    (&DAT_00999be8)[uVar4 * 0x8e] = param_1;
    iVar1 = uVar4 * 0x238;
    *(int *)((char *)&DAT_00999bec + iVar1) = 0x7f;
    *(int *)((char *)&DAT_00999bf4 + iVar1) = 0x7f;
    *(int *)((char *)&DAT_00999bfc + iVar1) = 0x7f;
    (&DAT_00999bf0)[uVar4 * 0x8e] = 0x7f;
    (&DAT_00999bf8)[uVar4 * 0x8e] = 0x7f;
    (&DAT_00999c00)[uVar4 * 0x8e] = 0x7f;
    (&DAT_00999c04)[uVar4 * 0x8e] = 0;
    *(int *)((char *)&DAT_00999c08 + iVar1) = 0x39;
    (&DAT_00999e08)[uVar4 * 0x8e] = 1;
    DVar3 = timeGetTime();
    (&DAT_00999e0c)[uVar4 * 0x8e] = DVar3;
    (&DAT_00999e10)[uVar4 * 0x8e] = 0;
    (&DAT_00999e14)[uVar4 * 0x8e] = 0;
    DAT_0099f6e0 = DAT_0099f6e0 + 1;
  }
  return uVar4;
}
