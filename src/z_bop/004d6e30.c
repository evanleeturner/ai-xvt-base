/* SPDX-License-Identifier: MIT */
/* z_bop 004d6e30 */
extern unsigned char DAT_00aa7b1f[4];   /* int at unaligned address 0xaa7b1f, read via memcpy */
extern unsigned char DAT_00aa7b23;
extern unsigned char DAT_00aa7b9e[];
extern unsigned int DAT_0052bb40[];

int __cdecl FUN_004d6e30(unsigned char param_1, unsigned char param_2, unsigned char param_3)
{
    int iVar1;
    int iVar2;
    unsigned int uVar3;
    int iVar4;
    int iVar5;
    int iVar6;
    unsigned int uVar7;
    int local_mode;

    __builtin_memcpy(&local_mode, DAT_00aa7b1f, 4);

    if (local_mode == 8) {
        uVar7 = 0x7fffffff;
        iVar6 = 1;
        iVar1 = 1;
        while (1) {
            iVar4 = (int)DAT_00aa7b9e[iVar6 * 4] - (int)(unsigned int)param_1;
            if (iVar4 < 0) {
                iVar4 = -iVar4;
            }
            iVar5 = (int)DAT_00aa7b9e[iVar6 * 4 + 1] - (int)(unsigned int)param_2;
            if (iVar5 < 0) {
                iVar5 = -iVar5;
            }
            iVar2 = (int)DAT_00aa7b9e[iVar6 * 4 + 2] - (int)(unsigned int)param_3;
            if (iVar2 < 0) {
                iVar2 = -iVar2;
            }
            uVar3 = DAT_0052bb40[iVar2] + DAT_0052bb40[iVar5] + DAT_0052bb40[iVar4];
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
    if (local_mode != 0x10) {
        return local_mode;
    }
    if (DAT_00aa7b23 != 0) {
        return (int)((((unsigned int)(param_1 >> 3) * 0x20u + (unsigned int)(param_2 >> 3)) * 0x20u) + (unsigned int)(param_3 >> 3));
    }
    return (int)((unsigned int)(param_3 >> 3) + (((unsigned int)(param_1 >> 3) * 0x40u + (unsigned int)(param_2 >> 2)) * 0x20u));
}
