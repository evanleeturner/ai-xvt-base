/* SPDX-License-Identifier: MIT */
/* z_xvt 0044c560 */
extern unsigned short DAT_00732a3c;
extern int DAT_006d4980;
extern int DAT_006d4988;
extern char DAT_007092d4;

int __cdecl FUN_0042b150(int param_1);

bool __cdecl FUN_0044c560(unsigned short param_1, short param_2)
{
  char cVar1;
  int iVar2;
  int iVar3;
  unsigned short uVar4;
  unsigned short uVar5;
  unsigned int uVar6;

  uVar5 = 0;
  uVar4 = DAT_00732a3c;
  if ((int)(unsigned int)DAT_00732a3c < DAT_006d4980) {
    do {
      iVar2 = (unsigned int)uVar4 * 0x23 + DAT_006d4988;
      if (*(char *)(iVar2 + 3) != '\0') {
        iVar2 = *(int *)(*(int *)(iVar2 + 0x1f) + 0xa7);
        iVar3 = FUN_0042b150(iVar2);
        if ((((param_1 == *(unsigned short *)(iVar3 + 0x18)) && (param_1 != uVar4)) &&
            (*(unsigned short *)(iVar2 + 0x342) != *(unsigned short *)(iVar3 + 0x18))) &&
           (((cVar1 = *(char *)(iVar3 + 0x34), cVar1 == '\v' || (cVar1 == '\f')) ||
            (cVar1 == '\x17')))) {
          uVar5 = uVar5 + 1;
        }
      }
      uVar4 = uVar4 + 1;
    } while ((int)(unsigned int)uVar4 < DAT_006d4980);
  }
  uVar6 = (unsigned int)param_1;
  if ((int)uVar6 < DAT_006d4980) {
    if (param_2 == 1) {
      uVar4 = 100;
    }
    else {
      cVar1 = *(char *)(uVar6 * 0x23 + 2 + DAT_006d4988);
      if ((cVar1 == '\x04') || (cVar1 == '\x05')) {
        uVar4 = 6;
      }
      else {
        uVar4 = (-(unsigned short)(cVar1 == '\x03') & 2) + 2;
      }
    }
  }
  else {
    uVar4 = 2;
  }
  if (*(int *)(uVar6 * 0x23 + 0x1b + DAT_006d4988) != -1) {
    if (DAT_007092d4 == '\0') {
      uVar4 = (-(unsigned short)(param_2 == 1) & 2) + 2;
    }
    else if (DAT_007092d4 == '\x01') {
      uVar4 = (-(unsigned short)(param_2 == 1) & 5) + 3;
    }
    else if (DAT_007092d4 == '\x02') {
      uVar4 = (-(unsigned short)(param_2 == 1) & 0x60) + 4;
    }
  }
  return uVar5 < uVar4;
}
