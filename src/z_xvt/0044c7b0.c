/* SPDX-License-Identifier: MIT */
/* z_xvt 0044c7b0 */
extern unsigned int DAT_006d9d60;
extern char DAT_006d9d78;
extern char DAT_006d9d7a;
extern char DAT_00703ca0;
extern char DAT_006da3e0;
extern char DAT_00524798;
extern char DAT_0070ad92;
extern char s_disableldr1pln_00525028[];
extern char s_capescortersldr1pln_0052504c[];

short __fastcall FUN_0044bec0(unsigned int);
short __fastcall FUN_0044c870(unsigned int);

int __fastcall FUN_0044c7b0(unsigned int param_1, unsigned int param_2, unsigned short param_3)
{
  short sVar1;
  int iVar2;
  unsigned int uVar3;
  char *pcVar4;
  char *pcVar5;
  char *pcVar6;
  char bVar7;

  uVar3 = ((param_2 >> 0x10) << 16) | param_3;
  DAT_006d9d78 = ((unsigned int)DAT_006d9d60 * 0x11) == 0;
  pcVar5 = (char *)((int)&DAT_00703ca0 +
                   (unsigned int)*(unsigned char *)((int)&DAT_006da3e0 +
                                  (unsigned int)((unsigned char *)&DAT_00524798)
                                              [(unsigned char)((unsigned char *)&DAT_0070ad92)
                                                     [(unsigned int)DAT_006d9d60 * 0x566 +
                                                      (unsigned int)param_3 * 0x52]]) * 0x55);
  iVar2 = 0xf;
  pcVar4 = pcVar5;
  pcVar6 = s_disableldr1pln_00525028;
  do {
    if (iVar2 == 0) break;
    iVar2 = iVar2 - 1;
    DAT_006d9d78 = *pcVar4 == *pcVar6;
    pcVar4 = pcVar4 + 1;
    pcVar6 = pcVar6 + 1;
  } while (DAT_006d9d78);
  iVar2 = 0x14;
  DAT_006d9d7a = 2;
  pcVar4 = s_capescortersldr1pln_0052504c;
  bVar7 = DAT_006d9d78;
  do {
    if (iVar2 == 0) break;
    iVar2 = iVar2 - 1;
    bVar7 = *pcVar5 == *pcVar4;
    pcVar5 = pcVar5 + 1;
    pcVar4 = pcVar4 + 1;
  } while (bVar7);
  if (bVar7) {
    sVar1 = FUN_0044bec0(uVar3);
  }
  else {
    sVar1 = FUN_0044c870(uVar3);
  }
  return sVar1 != -1;
}
