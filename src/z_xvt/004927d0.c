/* SPDX-License-Identifier: MIT */
/* z_xvt 004927d0 */
extern unsigned char *DAT_006d9d56;
extern unsigned short DAT_006d9d60;
extern unsigned short DAT_006d9d62;
extern unsigned short DAT_0070b1da[];
extern char DAT_00703ca0[];
extern char DAT_005250b4;
extern char s_formevadeldr1pln_005250a0[];
extern char s_starshipformpln_00525090[];
void __stdcall FUN_0042a060(void);

void __stdcall FUN_004927d0(void)
{
  unsigned char bVar1;
  unsigned char bVar2;
  int iVar3;
  char *pcVar4;
  char *pcVar5;
  char *pcVar6;
  _Bool bVar7;

  bVar1 = DAT_006d9d56[0xb];
  DAT_006d9d56[0xc] = DAT_006d9d56[0xc] + 1;
  bVar2 = DAT_006d9d56[0xc];
  bVar7 = bVar2 == 0xb;
  if (bVar2 > 0xb || DAT_0070b1da[DAT_006d9d60 * 0x566 + bVar2 * 2] == 0) {
    DAT_006d9d56[0xc] = 4;
    pcVar5 = &DAT_00703ca0[bVar1 * 0x55];
    iVar3 = 0xc;
    pcVar4 = pcVar5;
    pcVar6 = &DAT_005250b4;
    do {
      if (iVar3 == 0) break;
      iVar3 = iVar3 - 1;
      bVar7 = *pcVar4 == *pcVar6;
      pcVar4 = pcVar4 + 1;
      pcVar6 = pcVar6 + 1;
    } while (bVar7);
    if (!bVar7) {
      iVar3 = 0x11;
      pcVar4 = pcVar5;
      pcVar6 = s_formevadeldr1pln_005250a0;
      do {
        if (iVar3 == 0) break;
        iVar3 = iVar3 - 1;
        bVar7 = *pcVar4 == *pcVar6;
        pcVar4 = pcVar4 + 1;
        pcVar6 = pcVar6 + 1;
      } while (bVar7);
      if (!bVar7) {
        iVar3 = 0x10;
        pcVar4 = s_starshipformpln_00525090;
        do {
          if (iVar3 == 0) break;
          iVar3 = iVar3 - 1;
          bVar7 = *pcVar5 == *pcVar4;
          pcVar5 = pcVar5 + 1;
          pcVar4 = pcVar4 + 1;
        } while (bVar7);
        if (!bVar7) goto LAB_00492875;
      }
    }
    pcVar5 = (char *)(DAT_006d9d62 + 5 + DAT_006d9d56);
    *pcVar5 = *pcVar5 + 1;
  }
LAB_00492875:
  *(unsigned short *)(DAT_006d9d56 + 0x18) = DAT_006d9d56[0xc] + 0x8000;
  *(unsigned short *)(DAT_006d9d56 + 0x1a) = 0;
  *(unsigned char *)(DAT_006d9d56 + 0x1e) = 0;
  FUN_0042a060();
  return;
}
