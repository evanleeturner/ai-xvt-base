/* SPDX-License-Identifier: MIT */
/* z_xvt 0040c290 */
extern byte DAT_0085e796;
extern byte DAT_00523ac8;
extern byte DAT_00523ad8;

extern ushort FUN_004784d0(void);
extern undefined4 FUN_0040c4c0(int param_1, char param_2, char param_3, char param_4, short param_5);

undefined4 __cdecl FUN_0040c290(int param_1)
{
  byte bVar1;
  byte bVar2;
  ushort uVar3;
  ushort uVar4;
  uint uVar5;
  
  if (DAT_0085e796 == '\0') {
    return 0;
  }
  bVar1 = (&DAT_00523ac8)[param_1];
  switch(param_1) {
  case 0:
    bVar2 = (&DAT_00523ad8)[param_1];
    if (bVar2 == 0) break;
    uVar4 = FUN_004784d0();
    uVar3 = uVar4 / bVar2;
    if (uVar3 == 0) break;
    uVar5 = (uint)uVar4 % (uint)uVar3;
    goto LAB_0040c386;
  case 1:
    bVar2 = (&DAT_00523ad8)[param_1];
    if (bVar2 != 0) {
      uVar4 = FUN_004784d0();
      uVar3 = uVar4 / bVar2;
      if (uVar3 != 0) {
        uVar5 = (uint)uVar4 % (uint)uVar3;
        goto LAB_0040c386;
      }
    }
    break;
  case 2:
  case 3:
  case 4:
  case 5:
  case 6:
  case 7:
  case 8:
  case 9:
    bVar2 = (&DAT_00523ad8)[param_1];
    if (bVar2 != 0) {
      uVar4 = FUN_004784d0();
      uVar3 = uVar4 / bVar2;
      if (uVar3 != 0) {
        uVar5 = (uint)uVar4 % (uint)uVar3;
        goto LAB_0040c386;
      }
    }
    break;
  default:
    goto switchD_0040c2ca_default;
  }
  uVar5 = 0;
LAB_0040c386:
  FUN_0040c4c0(uVar5 + bVar1 + 0x321,'\x03',(char)param_1,'\0',-1);
switchD_0040c2ca_default:
  return 1;
}
