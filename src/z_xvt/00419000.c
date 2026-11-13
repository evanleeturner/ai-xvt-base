/* SPDX-License-Identifier: MIT */
/* z_xvt 00419000 */
extern ushort DAT_0071e520;
extern uint DAT_007327f0;
extern ushort DAT_00524368[];
extern ushort DAT_0052436a[];

void __cdecl FUN_00419000(uint param_1, uint param_2, ushort *param_3, ushort *param_4)
{
    int bVar1;
    int iVar2;
    uint uVar3;
    ushort uVar4;
    uint uVar5;
    uint uVar6;

    uVar6 = 0;
    DAT_0071e520 = 0;
    if (param_2 == param_1) {
        iVar2 = 0x100;
        DAT_007327f0 = param_1;
    }
    else {
        bVar1 = (int)param_1 <= (int)param_2;
        uVar3 = param_2;
        if (bVar1) {
            uVar3 = param_1;
            param_1 = param_2;
        }
        DAT_0071e520 = (ushort)bVar1;
        DAT_007327f0 = param_1;
        if (param_1 == 0) {
            uVar6 = uVar3 >> 0x10;
            iVar2 = 0;
        }
        else {
            uVar6 = uVar3;
            uVar5 = param_1;
            if ((param_1 & 0xff000000) == 0) {
                uVar5 = param_1 << 8;
                uVar6 = uVar3 << 8;
                if ((uVar5 & 0xff000000) == 0) {
                    uVar5 = param_1 << 0x10;
                    uVar6 = uVar3 << 0x10;
                }
            }
            if (uVar6 == uVar5) {
                uVar6 = uVar6 >> 0x10;
                iVar2 = 0x100;
            }
            else {
                /* divisor (uVar5 >> 16) is never 0: uVar5 is normalized with a nonzero top byte */
                uVar6 = uVar6 / (uVar5 >> 0x10);
                iVar2 = (int)(uVar6 & 0xffff) >> 8;
                uVar6 = (uVar6 & 0xff) << 8;
            }
        }
    }
    *param_4 = (ushort)iVar2;
    uVar4 = DAT_0052436a[iVar2];
    *param_3 = uVar4;
    uVar4 = (ushort)(uVar4 - DAT_00524368[(uint)*param_4]);
    *param_3 = uVar4;
    uVar4 = (ushort)(((uint)uVar4 * (uVar6 & 0xff00)) >> 0x10);
    *param_3 = uVar4;
    uVar4 = (ushort)(DAT_00524368[(uint)*param_4] + uVar4);
    *param_3 = uVar4;
    if (DAT_0071e520 != 0) {
        uVar4 = (ushort)(0 - (uint)uVar4);
        *param_3 = uVar4;
        *param_3 = (ushort)(uVar4 + 0x4000);
    }
    return;
}
