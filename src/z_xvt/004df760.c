/* SPDX-License-Identifier: MIT */
/* z_xvt 004df760 */
extern unsigned char DAT_00759657[4];   /* int at odd address 0x759657: packed, read via memcpy */
extern unsigned char DAT_0075965b;
extern unsigned char DAT_007596d3[];
extern unsigned int DAT_00532c68[];

int __cdecl FUN_004df760(unsigned char param_1, unsigned char param_2, unsigned char param_3)
{
    int mode;
    int iVar1;
    int iVar2;
    unsigned int uVar3;
    int iVar4;
    int iVar5;
    int iVar6;
    unsigned int uVar7;

    __builtin_memcpy(&mode, DAT_00759657, 4);

    if (mode == 8) {
        uVar7 = 0x7fffffff;
        iVar6 = 1;
        iVar1 = 1;
        while (1) {
            iVar4 = (int)((unsigned int)DAT_007596d3[iVar6 * 4] - (unsigned int)param_1);
            if (iVar4 < 0) {
                iVar4 = (int)(0u - (unsigned int)iVar4);
            }
            iVar5 = (int)((unsigned int)DAT_007596d3[iVar6 * 4 + 1] - (unsigned int)param_2);
            if (iVar5 < 0) {
                iVar5 = (int)(0u - (unsigned int)iVar5);
            }
            iVar2 = (int)((unsigned int)DAT_007596d3[iVar6 * 4 + 2] - (unsigned int)param_3);
            if (iVar2 < 0) {
                iVar2 = (int)(0u - (unsigned int)iVar2);
            }
            uVar3 = DAT_00532c68[iVar2] + DAT_00532c68[iVar5] + DAT_00532c68[iVar4];
            if (uVar3 == 0) {
                break;
            }
            if (uVar3 < uVar7) {
                iVar1 = iVar6;
                uVar7 = uVar3;
            }
            iVar6 = iVar6 + 1;
            if (0xff < iVar6) {
                return iVar1;
            }
        }
        return iVar6;
    }
    if (mode != 0x10) {
        return mode;
    }
    if (DAT_0075965b != 0) {
        return (int)((((unsigned int)(param_1 >> 3) * 0x20u + (unsigned int)(param_2 >> 3)) * 0x20u) + (unsigned int)(param_3 >> 3));
    }
    return (int)((unsigned int)(param_3 >> 3) + (((unsigned int)(param_1 >> 3) * 0x40u + (unsigned int)(param_2 >> 2)) * 0x20u));
}
