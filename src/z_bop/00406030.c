/* SPDX-License-Identifier: MIT */
/* z_bop 00406030 */
extern int DAT_00a08104;
extern int DAT_009a1fe8;
extern int DAT_009ecc34;
extern char DAT_0051c5aa[];
extern char DAT_009e9697[];

extern int __cdecl FUN_00406290(int param_1, unsigned int uVar7, unsigned int byte_val, unsigned int param_2);
extern unsigned int __cdecl FUN_00484d80(unsigned int arg);
extern void __cdecl FUN_00450650(unsigned int arg1, int arg2);

void __cdecl FUN_00406030(int param_1, unsigned int param_2)
{
  short local_short;
  unsigned char bVar2;
  unsigned char bVar3;
  char cVar4;
  int bVar5;
  int iVar6;
  unsigned int uVar7;
  short sVar8;

  DAT_00a08104 = *(int *)(*(int *)(DAT_009a1fe8 + 0x1f + param_1 * 0x23) + 0xa9);
  bVar2 = *(unsigned char *)(param_2 + 0x235 + DAT_00a08104);
  iVar6 = param_2 + DAT_00a08104;
  sVar8 = 0;
  bVar5 = 0;
  bVar3 = DAT_0051c5aa[param_2 + (unsigned int)*(unsigned char *)(DAT_00a08104 + 4) * 0x101];
  uVar7 = (unsigned int)bVar3;

  if ((bVar2 & 0x7f) == 3) {
    iVar6 = FUN_00406290(param_1, uVar7, (unsigned int)*(unsigned char *)(iVar6 + 0x233), param_2);
    if (iVar6 == -1) {
      if (*(char *)(DAT_00a08104 + 0x372 + uVar7 * 4) != '\0') {
        bVar5 = 1;
      }
    }
    else {
      sVar8 = 1;
    }
    uVar7 = (unsigned int)(unsigned short)(bVar3 + 1);
    iVar6 = FUN_00406290(param_1, uVar7, (unsigned int)*(unsigned char *)(DAT_00a08104 + 0x233 + param_2), param_2);
    if (iVar6 != -1) {
      sVar8 = sVar8 + 1;
      goto LAB_004061a9;
    }
    cVar4 = *(char *)(DAT_00a08104 + 0x372 + uVar7 * 4);
  }
  else if ((bVar2 & 0x80) == 0) {
    iVar6 = FUN_00406290(param_1, uVar7, (unsigned int)*(unsigned char *)(iVar6 + 0x233), param_2);
    if (iVar6 != -1) {
      sVar8 = 1;
      goto LAB_004061a9;
    }
    cVar4 = *(char *)(DAT_00a08104 + 0x372 + uVar7 * 4);
  }
  else {
    uVar7 = (unsigned int)(unsigned short)(bVar3 + 1);
    iVar6 = FUN_00406290(param_1, uVar7, (unsigned int)*(unsigned char *)(iVar6 + 0x233), param_2);
    if (iVar6 != -1) {
      sVar8 = 1;
      goto LAB_004061a9;
    }
    cVar4 = *(char *)(DAT_00a08104 + 0x372 + uVar7 * 4);
  }

  if (cVar4 != '\0') {
    bVar5 = 1;
  }

LAB_004061a9:
  __builtin_memcpy(&local_short, (void *)(DAT_00a08104 + 0x237 + param_2 * 2), 2);
  local_short += 0x1d8;
  __builtin_memcpy((void *)(DAT_00a08104 + 0x237 + param_2 * 2), &local_short, 2);

  if ((*(int *)(DAT_009a1fe8 + 0x1b + param_1 * 0x23) == DAT_009ecc34) && (!bVar5)) {
    uVar7 = FUN_00484d80(((DAT_00a08104 & 0xFFFFFF00) | *(unsigned char *)(DAT_009e9697[DAT_009ecc34 * 0x5bd] + 0x233 + DAT_00a08104)) & 0xFFFF00FF);
    if (sVar8 == 0) {
      FUN_00450650((uVar7 & 0xffff) + 0x26, DAT_009ecc34);
      return;
    }
    if (sVar8 == 1) {
      FUN_00450650((uVar7 & 0xffff) + 0x30, DAT_009ecc34);
      return;
    }
    if (sVar8 == 2) {
      FUN_00450650((uVar7 & 0xffff) + 0x3a, DAT_009ecc34);
    }
  }
  return;
}
