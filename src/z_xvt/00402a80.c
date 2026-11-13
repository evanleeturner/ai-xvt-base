/* SPDX-License-Identifier: MIT */
/* z_xvt 00402a80 */
extern unsigned int FUN_00402b60(HFILE param_1, int *param_2, unsigned int *param_3);
extern unsigned short FUN_0048f3c0(unsigned int param_1, unsigned int param_2);
extern int FUN_0048f4f0(unsigned int param_1);
extern void FUN_0048f510(void);
extern long _ftell(HFILE);
extern int _fseek(HFILE, long, int);

unsigned short FUN_00402a80(HFILE param_1)
{
  unsigned short uVar1;
  long _Offset;
  unsigned int uVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  int *piVar6;
  int iVar7;

  iVar7 = 0;
  iVar5 = 0;
  _Offset = _ftell(param_1);
  while (1) {
    uVar2 = FUN_00402b60(param_1, (int *)0, (unsigned int *)0);
    if (uVar2 == 0xFFFFFFFF) break;
    if (uVar2 != 0) {
      iVar5 = iVar5 + uVar2;
      iVar7 = iVar7 + 1;
    }
  }
  iVar4 = 0;
  _fseek(param_1, _Offset, 0);
  uVar1 = FUN_0048f3c0(iVar5 + iVar7 * 4 + 0xE, 0);
  iVar3 = FUN_0048f4f0((unsigned int)uVar1);
  *(int *)(iVar3 + 6) = iVar7;
  iVar5 = 0;
  *(int *)iVar3 = iVar3;
  *(unsigned short *)(iVar3 + 4) = uVar1;
  *(int *)(iVar3 + 10) = iVar3 + 0xE;
  piVar6 = (int *)(iVar3 + 0xE + iVar7 * 4);
  if (0 < iVar7) {
    do {
      uVar2 = FUN_00402b60(param_1, piVar6, (unsigned int *)(*(int *)(iVar3 + 10) + iVar4));
      if (uVar2 != 0) {
        iVar4 = iVar4 + 4;
        iVar5 = iVar5 + 1;
        piVar6 = (int *)((int)piVar6 + uVar2);
      }
    } while (iVar5 < iVar7);
  }
  FUN_0048f510();
  return uVar1;
}
