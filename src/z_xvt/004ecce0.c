/* SPDX-License-Identifier: MIT */
/* z_xvt 004ecce0 */
extern int DAT_00757cb4;
extern int DAT_00757ca4;
extern int DAT_00743dc2;
extern int DAT_00757cb0;
extern int DAT_007438d2;
extern int DAT_00757cac;
extern int DAT_005333fc;
extern int DAT_0053340c;
extern int DAT_007579d0;
extern int DAT_00757870;
extern int DAT_00757cee;
extern int DAT_00757680;
extern int DAT_007579f8;
extern int DAT_00757cf6;
extern int DAT_00757cfa;
extern int DAT_00743dc3;

uint __cdecl FUN_004ecce0(int param_1)
{
  byte bVar1;
  uint uVar2;
  int *piVar3;
  int iVar4;
  int iVar5;
  int *piVar6;
  int *local_8;
  int local_4;
  
  if (param_1 == -1) {
    if (DAT_00757cb4 == 0) {
      if (DAT_00757ca4 != 0) {
        return (uint)(byte)(&DAT_00743dc2)[DAT_00757ca4 + DAT_00757cb0 * 0x562];
      }
      return (uint)(byte)(&DAT_007438d2)[DAT_00757cb0 * 0x562];
    }
    if (DAT_00757cac == 0) {
      return (uint)(byte)(&DAT_007438d2)[DAT_00757cb0 * 0x562];
    }
    bVar1 = (&DAT_00743dc2)[DAT_00757cb0 * 0x562];
    uVar2 = DAT_00757cac;
    if (bVar1 != 0) {
      if (bVar1 < 3) {
        return (&DAT_005333fc)[DAT_00757cac];
      }
      if (bVar1 == 3) {
        return (&DAT_0053340c)[DAT_00757cac];
      }
      return DAT_00757cac;
    }
  }
  else {
    iVar4 = 0;
    piVar6 = &DAT_007579d0;
    local_8 = &DAT_00757870;
    do {
      iVar5 = 0;
      if (0 < *piVar6) {
        piVar3 = local_8;
        do {
          if (*piVar3 == *(int *)((int)&DAT_00757cee + param_1 * 0x2a)) {
            local_4 = *(int *)(&DAT_00757680 + (iVar5 + iVar4) * 4);
            break;
          }
          piVar3 = piVar3 + 1;
          iVar5 = iVar5 + 1;
        } while (iVar5 < *piVar6);
      }
      iVar4 = iVar4 + 8;
      local_8 = local_8 + 8;
      piVar6 = piVar6 + 1;
    } while (piVar6 < &DAT_007579f8);
    uVar2 = *(uint *)((int)&DAT_00757cf6 + param_1 * 0x2a);
    if (uVar2 == 0) {
      uVar2 = *(uint *)((int)&DAT_00757cfa + param_1 * 0x2a);
      if ((uVar2 != 0xffffffff) && (uVar2 < 10)) {
        return (uint)(byte)(&DAT_00743dc3)[uVar2 + local_4 * 0x562];
      }
      uVar2 = (uint)(byte)(&DAT_007438d2)[local_4 * 0x562];
    }
  }
  return uVar2;
}
