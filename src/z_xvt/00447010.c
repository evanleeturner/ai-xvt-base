/* SPDX-License-Identifier: MIT */
/* z_xvt 00447010 */
void __cdecl FUN_004a1d70(int, int);
void __cdecl FUN_00447740(int, unsigned int);
void __cdecl FUN_00447240(int, unsigned int);

extern int DAT_0071b730;
extern int DAT_006d4988;
extern char DAT_0071b758;
extern short DAT_0071e6be;
extern char DAT_0071b757;
extern int DAT_0071bcbc;
extern int DAT_0071ecf4;
extern short DAT_00732950;
extern short DAT_00732952;
extern int DAT_00525564;
extern unsigned char DAT_0052b0b4;

void __cdecl FUN_00447010(int param_1)
{
  int iVar1;
  int iVar2;
  int iVar3;
  unsigned int uVar4;
  short sVar5;

  iVar1 = *(int *)((int)&DAT_0071b730 + param_1 * 0x5bd);
  iVar3 = param_1 * 0x5bd;
  if (iVar1 == -1) {
    return;
  }
  iVar2 = *(int *)(*(int *)(iVar1 * 0x23 + 0x1f + DAT_006d4988) + 0xa7);
  if ((*(int *)(iVar2 + 0x1d) != 0) &&
     ((*(char *)(iVar2 + 0x23c) != '\x01' || (*(short *)(iVar2 + 0x23e) == 0)))) {
    FUN_004a1d70(0x15f, param_1);
    return;
  }
  if ((&DAT_0071b758)[iVar3] != '\0') {
    if ((short)((DAT_0071e6be >> 1) + DAT_0071e6be) <=
        *(short *)(iVar2 + 0x22d + (unsigned int)(unsigned char)(&DAT_0071b757)[iVar3] * 2)) {
      return;
    }
    if ((*(unsigned char *)(iVar2 + 0x1df) & 8) != 0) {
      FUN_00447740(iVar1, (unsigned int)(unsigned char)(&DAT_0071b757)[iVar3]);
      iVar1 = *(int *)(*(int *)(iVar1 * 0x23 + 0x1f + DAT_006d4988) + 0xa7);
      if ((unsigned int)*(unsigned char *)(iVar1 + 0x34a +
                         (unsigned int)(unsigned char)(&DAT_0052b0b4)
                                     [(unsigned int)(unsigned char)(&DAT_0071b757)[iVar3] +
                                      (unsigned int)*(unsigned char *)(iVar1 + 4) * 0xef] * 4) +
          (unsigned int)*(unsigned char *)(iVar1 + 0x346 +
                         (unsigned int)(unsigned char)(&DAT_0052b0b4)
                                     [(unsigned int)(unsigned char)(&DAT_0071b757)[iVar3] +
                                      (unsigned int)*(unsigned char *)(iVar1 + 4) * 0xef] * 4) != 0) {
        return;
      }
      (&DAT_0071b758)[iVar3] = 0;
      (&DAT_0071b757)[iVar3] = 0;
      *(short *)(iVar1 + 0x21c) = 0x76;
      *(int *)(iVar1 + 0x220) = *(int *)((int)&DAT_0071bcbc + iVar3) + 0x76;
      return;
    }
    if (param_1 != DAT_0071ecf4) {
      return;
    }
    DAT_00732950 = 0x5d;
    DAT_00732952 = 0x57;
    FUN_004a1d70(0x56, param_1);
    return;
  }
  uVar4 = (unsigned int)(unsigned char)(&DAT_0071b757)[iVar3];
  sVar5 = *(short *)(iVar2 + 0x21c + uVar4 * 2);
  if (DAT_00525564 != 0) {
    if (sVar5 != 0) {
      if (*(int *)((int)&DAT_0071bcbc + iVar3) <= *(int *)(iVar2 + 0x220 + uVar4 * 4)) {
        sVar5 = DAT_0071e6be * 2;
        goto LAB_004470d8;
      }
      sVar5 = 0;
    }
    *(int *)(iVar2 + 0x220 + uVar4 * 4) = *(int *)((int)&DAT_0071bcbc + iVar3);
  }
LAB_004470d8:
  if (sVar5 < (short)((DAT_0071e6be >> 1) + DAT_0071e6be)) {
    if ((*(unsigned char *)(iVar2 + 0x1df) & 0x10) != 0) {
      FUN_00447240(*(int *)((int)&DAT_0071b730 + iVar3), (unsigned int)(unsigned char)(&DAT_0071b757)[iVar3]);
      return;
    }
    if (param_1 == DAT_0071ecf4) {
      DAT_00732952 = 0x57;
      DAT_00732950 = (unsigned char)(&DAT_0071b757)[iVar3] + 0x5b;
      FUN_004a1d70(0x56, param_1);
      return;
    }
  }
  return;
}
