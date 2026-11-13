/* SPDX-License-Identifier: MIT */
/* z_bop 004f07d0 */
extern int DAT_00aa607c;
extern int DAT_00aa6084;
extern int DAT_00aa6090;
extern int DAT_00aa60d0;
extern int DAT_00aa615e;
extern int DAT_00aa62ae;

int __stdcall FUN_004d1400(void);
void FUN_004e0490(void);
void FUN_004f02f0(void);
void FUN_004f0510(void);

int __cdecl FUN_004f07d0(int param_1)
{
  unsigned int uVar1;
  int iVar2;
  int *puVar3;
  int *piVar4;
  int *puVar5;
  unsigned int uVar6;
  
  uVar6 = 0;
  if (param_1 == 0) {
    uVar1 = FUN_004d1400();
    puVar5 = &DAT_00aa6090;
    puVar3 = &DAT_00aa615e;
    do {
      if (uVar6 < uVar1) {
        *puVar5 = *puVar3;
        puVar5[1] = 0;
      }
      else {
        *puVar5 = 0;
      }
      puVar5 = puVar5 + 2;
      puVar3 = (int *)((int)puVar3 + 0x2a);
      uVar6 = uVar6 + 1;
    } while (puVar3 < &DAT_00aa62ae);
    DAT_00aa607c = 0;
  }
  FUN_004e0490();
  iVar2 = 0;
  piVar4 = &DAT_00aa6090;
  do {
    if ((*piVar4 != 0) && (piVar4[1] == 0)) {
      iVar2 = iVar2 + 1;
    }
    piVar4 = piVar4 + 2;
  } while (piVar4 < &DAT_00aa60d0);
  if (iVar2 == 0) {
    return 1;
  }
  FUN_004f02f0();
  if (DAT_00aa6084 == 1) {
    FUN_004f0510();
  }
  return 0;
}
