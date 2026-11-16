/* SPDX-License-Identifier: MIT */
/* z_bop 0045ddf0 */
extern unsigned short DAT_009a73c0;
extern int DAT_009a73d8;
extern char DAT_009a73da;
extern char DAT_009d1320;
extern char DAT_009a7a40[];
extern char DAT_00524120[];
extern char DAT_009d8cd2[];
extern char s_disableldr1pln_0051a310[];
extern char s_capescortersldr1pln_0051a334[];

short FUN_0045d1c0(unsigned int param_1);
short FUN_0045deb0(unsigned int param_1);

bool __cdecl FUN_0045ddf0(unsigned short param_1)
{
  short sVar1;
  int iVar2;
  unsigned int uVar3;
  char *pcVar4;
  char *pcVar5;
  char *pcVar6;
  bool bVar7;

  uVar3 = (((unsigned int)DAT_009a73c0 * 0x159 >> 0x10) << 16) | param_1;
  DAT_009a73d8 = (unsigned int)param_1 * 0x52 == 0;
  
  {
    unsigned int idx = (unsigned int)DAT_009a73c0 * 0x566 + (unsigned int)param_1 * 0x52;
    unsigned int val1 = (unsigned char)DAT_009d8cd2[idx];
    unsigned int val2 = (unsigned char)DAT_00524120[val1];
    unsigned int val3 = (unsigned char)DAT_009a7a40[val2];
    pcVar5 = (char *)&DAT_009d1320 + val3 * 0x55;
  }

  iVar2 = 0xf;
  pcVar4 = pcVar5;
  pcVar6 = s_disableldr1pln_0051a310;
  do {
    if (iVar2 == 0) break;
    iVar2 = iVar2 - 1;
    DAT_009a73d8 = *pcVar4 == *pcVar6;
    pcVar4 = pcVar4 + 1;
    pcVar6 = pcVar6 + 1;
  } while (DAT_009a73d8);
  
  iVar2 = 0x14;
  DAT_009a73da = 2;
  pcVar4 = s_capescortersldr1pln_0051a334;
  bVar7 = (bool)DAT_009a73d8;
  do {
    if (iVar2 == 0) break;
    iVar2 = iVar2 - 1;
    bVar7 = *pcVar5 == *pcVar4;
    pcVar5 = pcVar5 + 1;
    pcVar4 = pcVar4 + 1;
  } while (bVar7);
  
  if (bVar7) {
    sVar1 = FUN_0045d1c0(uVar3);
  }
  else {
    sVar1 = FUN_0045deb0(uVar3);
  }
  return sVar1 != -1;
}
