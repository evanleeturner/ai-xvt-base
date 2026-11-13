/* SPDX-License-Identifier: MIT */
/* z_bop 00435640 */
extern unsigned char DAT_009ed228;
extern unsigned char DAT_009ed22c;
extern unsigned char DAT_009ed22d;
extern char DAT_009d8e25[];
extern char DAT_009d8e29[];
extern int DAT_009d6990[];
extern int DAT_009e96dd[];
extern int DAT_009ecc34;
extern short DAT_00a07be0;

extern unsigned int FUN_00435a10(unsigned char, unsigned char, unsigned char);
extern int FUN_00450650(int, int);

int __cdecl FUN_00435640(unsigned short param_1, unsigned int param_2, int param_3, unsigned short param_4, int param_5, int param_6)
{
  int *piVar1;
  unsigned int uVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  unsigned int uVar6;
  char *pcVar7;
  int local_10;
  
  uVar2 = FUN_00435a10(DAT_009ed228, DAT_009ed22c, DAT_009ed22d);
  uVar6 = param_2 & 0xffff;
  iVar4 = 0;
  local_10 = 0;
  pcVar7 = DAT_009d8e25 + uVar6 * 0x566;
  do {
    if (((*pcVar7 == 0x02) &&
        (((((unsigned char)pcVar7[1] == param_1 && (pcVar7[2] == 0x12)) &&
          (DAT_009d8e29[iVar4 + param_6 + uVar6 * 0x566] != 0)) ||
         (((*pcVar7 == 0x02 && ((unsigned char)pcVar7[1] == param_1)) &&
          ((pcVar7[2] == 0x13 &&
           ((DAT_009d8e29[iVar4 + param_6 + uVar6 * 0x566] != 0 && (param_5 == 1)))))))))) &&
       (((unsigned int)(unsigned char)pcVar7[0xe] * 5 == 0 || (uVar2 <= (unsigned int)(unsigned char)pcVar7[0xe] * 5)))) {
      iVar5 = pcVar7[3] * 0xfa;
      if (10 < param_4) {
        param_4 = 9;
      }
      if (1 < param_4) {
        if (iVar5 < 1) {
          iVar3 = param_4 - 1;
        }
        else {
          iVar3 = 1 - (unsigned int)param_4;
        }
        iVar5 = iVar5 + iVar3 * (iVar5 / 10);
      }
      if (param_3 == -1) {
        DAT_009d6990[param_6] = DAT_009d6990[param_6] + iVar5;
      }
      else {
        piVar1 = (int *)((int)&DAT_009e96dd + param_3 * 0x5bd);
        *piVar1 = *piVar1 + iVar5;
      }
      local_10 = local_10 + iVar5;
    }
    pcVar7 = pcVar7 + 0x4e;
    iVar4 = iVar4 + 0x4e;
  } while (iVar4 < 0x270);
  if ((local_10 != 0) && (param_3 == DAT_009ecc34)) {
    DAT_00a07be0 = (short)local_10;
    if (local_10 < 1) {
      DAT_00a07be0 = -DAT_00a07be0;
      iVar4 = 200;
    }
    else {
      iVar4 = 199;
    }
    FUN_00450650(iVar4, param_3);
  }
  return local_10;
}
