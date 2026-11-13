/* SPDX-License-Identifier: MIT */
/* z_xvt 0044c6c0 */
extern ushort DAT_006d9d60;
extern uint DAT_006d9d78;
extern char DAT_00703ca0;
extern char DAT_006da3e0;
extern char DAT_00524798;
extern char DAT_0070ad92;
extern char DAT_00525ce0;
extern uint DAT_006d9d7a;
extern char s_disableldr1pln_00525028[];
extern char s_capfreeldr1pln_00525060[];
extern char s_capescortersldr1pln_0052504c[];

short __cdecl FUN_0044bec0(uint param_1);
short __cdecl FUN_0044c1c0(uint param_1);
short __cdecl FUN_0044b810(uint param_1);

bool __cdecl FUN_0044c6c0(ushort param_1)
{
  short sVar1;
  int iVar2;
  uint uVar3;
  char *pcVar4;
  char *pcVar5;
  char *pcVar6;
  bool bVar7;

  uVar3 = (((uint)(ushort)DAT_006d9d60 * 0x159) >> 0x10) << 16 | (uint)param_1;
  DAT_006d9d78 = (uint)param_1 * 0x52 == 0;
  pcVar5 = (char *)((int)&DAT_00703ca0 +
                   (uint)*(unsigned char *)((int)&DAT_006da3e0 +
                                  (uint)(unsigned char)(&DAT_00524798)
                                              [(unsigned char)(&DAT_0070ad92)
                                                     [(uint)DAT_006d9d60 * 0x566 +
                                                      (uint)param_1 * 0x52]]) * 0x55);
  iVar2 = 0xf;
  pcVar4 = pcVar5;
  pcVar6 = s_disableldr1pln_00525028;
  do {
    if (iVar2 == 0) break;
    iVar2 = iVar2 + -1;
    DAT_006d9d78 = *pcVar4 == *pcVar6;
    pcVar4 = pcVar4 + 1;
    pcVar6 = pcVar6 + 1;
  } while ((bool)DAT_006d9d78);
  iVar2 = 0xf;
  DAT_006d9d7a = 7;
  pcVar4 = pcVar5;
  pcVar6 = s_capfreeldr1pln_00525060;
  bVar7 = (bool)DAT_006d9d78;
  do {
    if (iVar2 == 0) break;
    iVar2 = iVar2 + -1;
    bVar7 = *pcVar4 == *pcVar6;
    pcVar4 = pcVar4 + 1;
    pcVar6 = pcVar6 + 1;
  } while (bVar7);
  if (!bVar7) {
    iVar2 = 0xf;
    pcVar4 = pcVar5;
    pcVar6 = s_disableldr1pln_00525028;
    do {
      if (iVar2 == 0) break;
      iVar2 = iVar2 + -1;
      bVar7 = *pcVar4 == *pcVar6;
      pcVar4 = pcVar4 + 1;
      pcVar6 = pcVar6 + 1;
    } while (bVar7);
    if (!bVar7) {
      iVar2 = 0xd;
      pcVar4 = pcVar5;
      pcVar6 = &DAT_00525ce0;
      do {
        if (iVar2 == 0) break;
        iVar2 = iVar2 + -1;
        bVar7 = *pcVar4 == *pcVar6;
        pcVar4 = pcVar4 + 1;
        pcVar6 = pcVar6 + 1;
      } while (bVar7);
      if (!bVar7) {
        iVar2 = 0x14;
        pcVar4 = s_capescortersldr1pln_0052504c;
        do {
          if (iVar2 == 0) break;
          iVar2 = iVar2 + -1;
          bVar7 = *pcVar5 == *pcVar4;
          pcVar5 = pcVar5 + 1;
          pcVar4 = pcVar4 + 1;
        } while (bVar7);
        if (bVar7) {
          sVar1 = FUN_0044bec0(uVar3);
        }
        else {
          sVar1 = FUN_0044c1c0(uVar3);
        }
        goto LAB_0044c79d;
      }
    }
  }
  sVar1 = FUN_0044b810(uVar3);
LAB_0044c79d:
  return sVar1 != -1;
}
