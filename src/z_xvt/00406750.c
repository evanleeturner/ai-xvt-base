/* SPDX-License-Identifier: MIT */
/* z_xvt 00406750 */
extern ushort DAT_0071b6b2;
extern ushort DAT_0070acec;
extern ushort DAT_007091ae;
extern ushort DAT_0070a170;
extern ushort DAT_006ffbf2;
extern ushort DAT_00732a20;
extern ushort DAT_006d4984;
extern uint DAT_006db598;
extern uint DAT_0070a144;
extern ushort DAT_0071f2e0;
extern ushort DAT_00703be4;
extern uint DAT_006db6d0;
extern uint DAT_00525518;
extern uint DAT_00525738;
extern ushort DAT_0071ed00;
extern ushort *DAT_00730884;
extern ushort DAT_006ffc00;
extern ushort DAT_00732e42;
extern uint DAT_00525734;

void __cdecl FUN_00406b50(ushort param_1, ushort *param_2);

void __cdecl FUN_00406750(ushort param_1)
{
  DAT_0071b6b2 = 0xffff;
  DAT_0070acec = DAT_007091ae;
  DAT_0070a170 = DAT_006ffbf2;
  DAT_00732a20 = DAT_006d4984;
  DAT_006db598 = DAT_0070a144;
  DAT_0071f2e0 = DAT_00703be4;
  DAT_006db6d0 = (uint)DAT_007091ae * DAT_00525518;
  DAT_00525738 = (uint)(DAT_0071ed00 == 0);
  DAT_00730884 = &DAT_006ffc00;
  DAT_00732e42 = 0x2000;
  if (DAT_00525738 != 1) {
    DAT_00732e42 = 0x2200;
  }
  if ((DAT_006ffc00 != param_1) || (DAT_00525734 == 0)) {
    FUN_00406b50(param_1, &DAT_006ffc00);
    DAT_00525734 = 1;
  }
  return;
}
