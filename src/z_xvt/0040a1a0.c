/* SPDX-License-Identifier: MIT */
/* z_xvt 0040a1a0 */
extern int DAT_00525554;
extern int DAT_00525550;
extern int DAT_0071ecf4;
extern char DAT_0070a189;
extern unsigned short DAT_00740a80[];
extern char DAT_0085e792;
extern char DAT_0085e79b;
extern char DAT_0085e791;
extern char DAT_0085e79a;
extern unsigned char DAT_0071b730[];
extern int DAT_006d4988;
extern char DAT_0073ba30[];

unsigned int __cdecl FUN_0040a4b0(unsigned int param_1, unsigned int param_2);
int __cdecl FUN_0040a640(unsigned int param_1, int *param_2);
unsigned int __cdecl FUN_00439830(unsigned int param_1, int param_2);
void __cdecl FUN_00490a50(char *param_1, int param_2, int param_3, unsigned int param_4,
                          unsigned int param_5, int param_6);

unsigned int __cdecl FUN_0040a1a0(unsigned int param_1, unsigned int param_2, int param_3)
{
  char cVar1;
  int iVar2;
  unsigned int uVar3;
  int iVar4;
  unsigned int uVar5;
  unsigned int local_4;
  unsigned short uTmp;

  if (DAT_00525554 == 0) {
    if (DAT_00525550 != 0) {
      return 0;
    }
  }
  else {
    if (DAT_00525550 == 0) {
      return 0;
    }
    if (DAT_00525554 == 2) {
      return 0;
    }
  }
  if (DAT_0071ecf4 != param_3) {
    return 0;
  }
  if (DAT_0070a189 == '\0') {
    return 0;
  }
  if (DAT_00740a80[param_1] == 0) {
    return 0;
  }
  if (param_2 == 0xffffffff) {
    if (DAT_0085e792 == '\0') {
      return 0;
    }
    if (DAT_0085e79b == '\0') {
      return 0;
    }
  }
  else {
    if (DAT_0085e791 == '\0') {
      return 0;
    }
    if (DAT_0085e79a == '\0') {
      return 0;
    }
  }
  if ((0x52 < param_1) && (param_1 < 0x57)) {
    __builtin_memcpy(&iVar2, DAT_0071b730 + (unsigned int)param_3 * 0x5bdU, 4);
    if (iVar2 == -1) {
      return 0;
    }
    cVar1 = *(char *)(unsigned int)((unsigned int)iVar2 * 0x23U + 3U + (unsigned int)DAT_006d4988);
    if ((cVar1 != '\x01') && (cVar1 != '\x02')) {
      return 0;
    }
  }
  local_4 = FUN_0040a4b0(param_2, param_1);
  if (local_4 == 0) {
    return 1;
  }
  iVar2 = FUN_0040a640(param_2, (int *)&local_4);
  uVar5 = 0x7c;
  if (local_4 < 0x7d) {
    uVar5 = local_4;
  }
  if ((param_2 == 0xffffffff) ||
      (iVar4 = (int)(param_2 * 0x23U + (unsigned int)DAT_006d4988),
       (__builtin_memcpy(&uVar3, (char *)(unsigned int)((unsigned int)iVar4 + 0x1bU), 4),
        (int)uVar3 == DAT_0071ecf4))) {
    uVar5 = ((0U - (unsigned int)(param_1 == 0x25)) & 2U) + 0x7dU;
  }
  else {
    __builtin_memcpy(&iVar4, (char *)(unsigned int)((unsigned int)iVar4 + 0x1fU), 4);
    if (iVar4 != 0) {
      __builtin_memcpy(&uTmp, (char *)(unsigned int)((unsigned int)iVar4 + 0x83U), 2);
      __builtin_memcpy(&uVar3,
                       (char *)(unsigned int)((unsigned int)uTmp * 0x23U + 0x1bU +
                                              (unsigned int)DAT_006d4988),
                       4);
      if ((int)uVar3 == DAT_0071ecf4) {
        uVar5 = 0x7d;
      }
    }
  }
  uVar3 = FUN_00439830(param_1, 0x100);
  if (((uVar3 != 0) && (0x47 < param_1)) && (param_1 < 0x50)) {
    return 0;
  }
  FUN_00490a50(DAT_0073ba30 + param_1 * 6, 1, 0, uVar5, local_4, iVar2);
  return 1;
}
