/* SPDX-License-Identifier: MIT */
/* z_xvt 0049ec30 */
extern unsigned char DAT_005247e8[];
extern unsigned char DAT_005247e9[];
extern unsigned char DAT_00524860[];
extern unsigned char DAT_0052486c[];
extern unsigned char DAT_0052686a[];
extern unsigned char DAT_0052686b[];
extern unsigned char DAT_0070ad42[];
extern unsigned char DAT_0070ad44[];
extern unsigned char DAT_0070ad47[];
extern unsigned char DAT_0070ad48[];
extern unsigned char DAT_0070ad49[];
extern unsigned char DAT_0070ad4f[];
extern unsigned char DAT_0070ad92[];
extern unsigned char DAT_0070b216[];
extern unsigned char DAT_0070b252[];

unsigned short __cdecl FUN_0049ec30(unsigned short param_1, unsigned short param_2, unsigned short param_3)
{
    int iVar1;
    unsigned short uVar2;
    unsigned int uVar3;
    int local_val;

    uVar2 = 0;
    iVar1 = (int)((unsigned int)param_1 * 0x566u);
    uVar3 = (unsigned int)DAT_005247e8[DAT_0070ad42[iVar1]];
    switch (param_2) {
    case 0:
        return 0;
    case 1:
        if (param_3 == param_1) {
            return 1;
        }
        break;
    case 2:
        if ((unsigned int)DAT_005247e9[param_3] == uVar3) {
            return 1;
        }
        break;
    case 3:
        if (DAT_0052686b[uVar3 * 0x16] == DAT_00524860[param_3]) {
            return 1;
        }
        break;
    case 4:
        if (DAT_0052686a[uVar3 * 0x16] == DAT_0052486c[param_3]) {
            return 1;
        }
        break;
    case 5:
        if ((unsigned int)DAT_0070ad47[iVar1] == (unsigned int)param_3) {
            return 1;
        }
        break;
    case 6:
        if ((unsigned int)DAT_0070ad92[iVar1] == (unsigned int)param_3) {
            return 1;
        }
        break;
    case 7:
        uVar2 = 0;
        __builtin_memcpy(&local_val, DAT_0070b252 + iVar1, sizeof(local_val));
        if (param_3 == 9) {
            if (local_val != -1) {
                return 1;
            }
        } else {
            if (param_3 != 10) {
                return 1;
            }
            if (local_val == -1) {
                return 1;
            }
        }
        break;
    case 8:
        if ((unsigned int)DAT_0070ad4f[iVar1] == (unsigned int)param_3) {
            return 1;
        }
        break;
    case 9:
        if ((unsigned int)DAT_0070ad49[iVar1] == (unsigned int)param_3) {
            return 1;
        }
        break;
    case 10:
        if ((unsigned int)DAT_0070ad44[iVar1] == (unsigned int)param_3) {
            return 1;
        }
        break;
    case 0xb:
        return 1;
    case 0xc:
        if ((unsigned int)DAT_0070ad48[iVar1] == (unsigned int)param_3) {
            return 1;
        }
        break;
    case 0xf:
        if (param_3 != param_1) {
            return 1;
        }
        break;
    case 0x10:
        if ((unsigned int)DAT_005247e9[param_3] != uVar3) {
            return 1;
        }
        break;
    case 0x11:
        if (DAT_0052686b[uVar3 * 0x16] != DAT_00524860[param_3]) {
            return 1;
        }
        break;
    case 0x12:
        if (DAT_0052686a[uVar3 * 0x16] != DAT_0052486c[param_3]) {
            return 1;
        }
        break;
    case 0x13:
        if ((unsigned int)DAT_0070ad47[iVar1] != (unsigned int)param_3) {
            return 1;
        }
        break;
    case 0x14:
        if ((unsigned int)DAT_0070ad4f[iVar1] != (unsigned int)param_3) {
            return 1;
        }
        break;
    case 0x15:
        if ((unsigned int)DAT_0070ad48[iVar1] != (unsigned int)param_3) {
            return 1;
        }
        break;
    case 0x17:
        if ((unsigned int)DAT_0070b216[iVar1] == (unsigned int)param_3) {
            return 1;
        }
        break;
    case 0x18:
        if ((unsigned int)DAT_0070b216[iVar1] != (unsigned int)param_3) {
            uVar2 = 1;
        }
    }
    return uVar2;
}
