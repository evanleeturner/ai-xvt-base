/* SPDX-License-Identifier: MIT */
/* z_bop 0046ffc0 */
extern DWORD DAT_00527028;
extern DWORD DAT_00527024;
extern DWORD DAT_0052702c;
extern DWORD DAT_0099f6e0;
extern DWORD DAT_00999bec;

DWORD __stdcall timeGetTime(void);

void __stdcall FUN_0046ffc0(void)
{
  DWORD DVar1;
  unsigned int uVar2;
  unsigned int *puVar3;
  
  uVar2 = 0;
  DAT_00527028 = DAT_00527024;
  DAT_0052702c = 0;
  if (DAT_0099f6e0 != 0) {
    puVar3 = (unsigned int *)&DAT_00999bec;
    do {
      uVar2 = uVar2 + 1;
      *puVar3 = puVar3[1];
      puVar3[2] = puVar3[3];
      puVar3[4] = puVar3[5];
      DVar1 = timeGetTime();
      puVar3[0x88] = DVar1;
      puVar3 = puVar3 + 0x8e;
    } while (uVar2 < DAT_0099f6e0);
  }
  return;
}
