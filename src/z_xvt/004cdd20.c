/* SPDX-License-Identifier: MIT */
/* z_xvt 004cdd20 */
void __stdcall FUN_004d4d90(void);
int __cdecl FUN_004d4d00(void);

extern char DAT_00759645;
extern int DAT_007806fa;
extern int DAT_00780732;
extern int DAT_0078077a;
extern int DAT_0078077e;
extern int DAT_00758830;

int __cdecl FUN_004cdd20(int param_1)
{
  char cVar1;
  int iVar2;
  int *piVar3;
  int bVar4;
  int local_val;
  int one;

  cVar1 = DAT_00759645;
  if (DAT_007806fa == 0) {
    return 0;
  }
  FUN_004d4d90();
  iVar2 = 0;
  bVar4 = DAT_00780732 == 0;
  if (0 < DAT_00780732) {
    piVar3 = (int *)&DAT_0078077a;
    do {
      __builtin_memcpy(&local_val, (void *)piVar3, 4);
      if (local_val == param_1) break;
      piVar3 = piVar3 + 10;
      iVar2 = iVar2 + 1;
    } while (iVar2 < DAT_00780732);
    bVar4 = iVar2 == DAT_00780732;
  }
  if (bVar4) {
    if (cVar1 != '\0') {
      DAT_00758830 = FUN_004d4d00();
    }
    return 0;
  }
  one = 1;
  __builtin_memcpy((void *)&DAT_0078077e + iVar2 * 0x28, &one, 4);
  if (cVar1 != '\0') {
    DAT_00758830 = FUN_004d4d00();
  }
  return 1;
}
