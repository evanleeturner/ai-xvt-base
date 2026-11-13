/* SPDX-License-Identifier: MIT */
/* z_xvt 0041b920 */
extern int DAT_006d4988;
extern unsigned short DAT_0052686c[];
extern short DAT_0052b14b[];
extern short DAT_0052b14d[];
extern short DAT_0052b14f[];

extern int __cdecl FUN_00412810(int, int, int);
extern void __cdecl FUN_00483410(short, short, int);
extern void __cdecl FUN_00483540(short, short, int);
extern int __cdecl FUN_00413cc0(int, int, int, int, int, int);

void __cdecl FUN_0041b920(int param_1, int param_2)
{
  unsigned short uVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  
  iVar7 = param_2 * 0x23 + DAT_006d4988;
  uVar1 = *(unsigned short *)(DAT_0052686c + (unsigned int)*(unsigned char *)(iVar7 + 3) * 0x16);
  iVar2 = DAT_006d4988 + param_1 * 0x23;
  iVar8 = *(int *)(iVar2 + 4) - *(int *)(iVar7 + 4);
  iVar5 = *(int *)(iVar2 + 8) - *(int *)(iVar7 + 8);
  iVar6 = *(int *)(iVar2 + 0xc) - *(int *)(iVar7 + 0xc);
  iVar2 = FUN_00412810(iVar8, iVar5, iVar6);
  if ((iVar2 <= (int)((unsigned int)uVar1 * 2)) && (*(int *)(iVar7 + 0x1f) != 0)) {
    if (*(char *)(*(int *)(iVar7 + 0x1f) + 0x90) != '\0') {
      FUN_00483410(*(short *)(iVar7 + 0x12), *(short *)(iVar7 + 0x10), iVar7);
      FUN_00483540(*(short *)(iVar7 + 0x14), 0, iVar7);
    }
    iVar2 = *(int *)(iVar7 + 0x1f);
    iVar3 = FUN_00413cc0((int)*(short *)(iVar2 + 0x97), (int)*(short *)(iVar2 + 0x99),
                         (int)*(short *)(iVar2 + 0x9b), iVar8, iVar5, iVar6);
    iVar2 = *(int *)(iVar7 + 0x1f);
    iVar4 = FUN_00413cc0((int)*(short *)(iVar2 + 0x91), (int)*(short *)(iVar2 + 0x93),
                         (int)*(short *)(iVar2 + 0x95), iVar8, iVar5, iVar6);
    iVar2 = *(int *)(iVar7 + 0x1f);
    iVar5 = FUN_00413cc0((int)*(short *)(iVar2 + 0x9d), (int)*(short *)(iVar2 + 0x9f),
                         (int)*(short *)(iVar2 + 0xa1), iVar8, iVar5, iVar6);
    iVar2 = *(int *)(*(int *)(iVar7 + 0x1f) + 0xa7);
    if (iVar2 != 0) {
      iVar2 = (unsigned int)*(unsigned char *)(iVar2 + 4) * 0xef;
      iVar2 = FUN_00412810(iVar3 - *(short *)(DAT_0052b14b + iVar2),
                           iVar4 - *(short *)(DAT_0052b14f + iVar2),
                           iVar5 - *(short *)(DAT_0052b14d + iVar2));
      if (((iVar2 <= (int)(uVar1 / 6)) &&
          (iVar2 = *(int *)(DAT_006d4988 + 0x1f + param_1 * 0x23), iVar2 != 0)) &&
         (iVar2 = *(int *)(iVar2 + 0xa7), iVar2 != 0)) {
        *(unsigned int *)(iVar2 + 0x1d) = 0x28000;
        *(unsigned short *)(iVar2 + 0x23e) = 0;
      }
    }
  }
  return;
}
