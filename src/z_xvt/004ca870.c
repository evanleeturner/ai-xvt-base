/* SPDX-License-Identifier: MIT */
/* z_xvt 004ca870 */
extern undefined4 _DAT_0064f060;
extern undefined4 _DAT_0064f064;
extern undefined4 _DAT_0064f068;
extern undefined4 _DAT_0064f06c;
extern undefined4 DAT_005202a0;
extern undefined4 DAT_005202a4;
extern undefined4 DAT_005202a8;
extern undefined4 DAT_005202ac;
extern undefined4 DAT_005202b0;
extern undefined4 DAT_005202b4;
extern undefined4 DAT_005202b8;
extern undefined4 DAT_005202bc;
extern undefined4 DAT_005202c0;
extern undefined4 DAT_005202c4;
extern undefined4 DAT_005202c8;
extern undefined4 DAT_005202cc;
extern undefined4 DAT_005202d0;
extern undefined4 DAT_005202d4;
extern undefined4 DAT_005202d8;
extern undefined4 DAT_005202dc;

long long __fastcall FUN_004ca870(undefined4 param_1, uint param_2, undefined4 param_3)
{
  switch (param_3) {
    case 0:
      _DAT_0064f060 = DAT_005202a0;
      _DAT_0064f064 = DAT_005202a4;
      _DAT_0064f068 = DAT_005202a8;
      _DAT_0064f06c = DAT_005202ac;
      return 0x64f0600064f060LL;
    case 1:
      _DAT_0064f060 = DAT_005202c0;
      _DAT_0064f064 = DAT_005202c4;
      _DAT_0064f068 = DAT_005202c8;
      _DAT_0064f06c = DAT_005202cc;
      return 0x64f0600064f060LL;
    case 2:
      _DAT_0064f060 = DAT_005202b0;
      _DAT_0064f064 = DAT_005202b4;
      _DAT_0064f068 = DAT_005202b8;
      _DAT_0064f06c = DAT_005202bc;
      return 0x64f0600064f060LL;
    case 3:
      _DAT_0064f060 = DAT_005202d0;
      _DAT_0064f064 = DAT_005202d4;
      _DAT_0064f068 = DAT_005202d8;
      _DAT_0064f06c = DAT_005202dc;
      return 0x64f0600064f060LL;
    default:
      return (unsigned long long)param_2 << 0x20;
  }
}
