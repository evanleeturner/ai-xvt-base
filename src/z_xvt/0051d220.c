/* SPDX-License-Identifier: MIT */
/* z_xvt 0051d220 */
extern short DAT_0074244e;
extern short DAT_0074244c;
extern unsigned int unaff_EDI;
extern int FUN_0048f4f0(unsigned int);
extern void FUN_0048f510(void);

void __cdecl FUN_0051d220(short param_1)
{
  int bVar1;
  int iVar2;
  unsigned int uVar3;

  if (DAT_0074244e == 0) {
    DAT_0074244e = param_1;
    DAT_0074244c = param_1;
    return;
  }
  uVar3 = (unaff_EDI & 0xFFFF0000) | (unsigned int)DAT_0074244e;
  bVar1 = 1;
  do {
    if ((uVar3 & 0xFFFF) == 0) break;
    iVar2 = FUN_0048f4f0(uVar3);
    if (*(short *)(iVar2 + 0x48) == 0) {
      *(short *)(iVar2 + 0x48) = param_1;
      bVar1 = 0;
    }
    FUN_0048f510();
    uVar3 = (uVar3 & 0xFFFF0000) | (unsigned int)*(short *)(iVar2 + 0x48);
  } while (bVar1);
  if (DAT_0074244c == 0) {
    DAT_0074244c = param_1;
  }
  return;
}
