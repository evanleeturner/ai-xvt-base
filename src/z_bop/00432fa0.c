/* SPDX-License-Identifier: MIT */
/* z_bop 00432fa0 */
extern unsigned char DAT_00524170[];
extern unsigned char DAT_00524171[];
extern unsigned char DAT_009d8c82[];
extern unsigned char DAT_009d8c84[];
extern unsigned char DAT_009d8c87[];
extern unsigned char DAT_009d8c88[];
extern unsigned char DAT_009d8c89[];
extern unsigned char DAT_009d8c8f[];
extern unsigned char DAT_009d8cd2[];
extern unsigned char DAT_009d9156[];
extern unsigned char DAT_009d9192[];
extern unsigned char DAT_00521f1a[];
extern unsigned char DAT_00521f1b[];
extern unsigned char DAT_005241e8[];
extern unsigned char DAT_005241f4[];

unsigned short __cdecl FUN_00432fa0(unsigned short param_1, unsigned short param_2, unsigned short param_3)
{
    unsigned int iVar1;
    unsigned short uVar2;
    unsigned int uVar3;
    int local_val;

    uVar2 = 0;
    iVar1 = (unsigned int)param_1 * 0x566;
    uVar3 = (unsigned int)DAT_00524170[DAT_009d8c82[iVar1]];
    switch (param_2) {
    case 0:
        return 0;
    case 1:
        if (param_3 == param_1) {
            return 1;
        }
        break;
    case 2:
        if ((unsigned int)DAT_00524171[param_3] == uVar3) {
            return 1;
        }
        break;
    case 3:
        if (DAT_00521f1b[uVar3 * 0x1a] == DAT_005241e8[param_3]) {
            return 1;
        }
        break;
    case 4:
        if (DAT_00521f1a[uVar3 * 0x1a] == DAT_005241f4[param_3]) {
            return 1;
        }
        break;
    case 5:
        if ((unsigned int)DAT_009d8c87[iVar1] == param_3) {
            return 1;
        }
        break;
    case 6:
        if ((unsigned int)DAT_009d8cd2[iVar1] == param_3) {
            return 1;
        }
        break;
    case 7:
        uVar2 = 0;
        __builtin_memcpy(&local_val, &DAT_009d9192[iVar1], 4);
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
        if ((unsigned int)DAT_009d8c8f[iVar1] == param_3) {
            return 1;
        }
        break;
    case 9:
        if ((unsigned int)DAT_009d8c89[iVar1] == param_3) {
            return 1;
        }
        break;
    case 10:
        if ((unsigned int)DAT_009d8c84[iVar1] == param_3) {
            return 1;
        }
        break;
    case 0xb:
        return 1;
    case 0xc:
        if ((unsigned int)DAT_009d8c88[iVar1] == param_3) {
            return 1;
        }
        break;
    case 0xf:
        if (param_3 != param_1) {
            return 1;
        }
        break;
    case 0x10:
        if ((unsigned int)DAT_00524171[param_3] != uVar3) {
            return 1;
        }
        break;
    case 0x11:
        if (DAT_00521f1b[uVar3 * 0x1a] != DAT_005241e8[param_3]) {
            return 1;
        }
        break;
    case 0x12:
        if (DAT_00521f1a[uVar3 * 0x1a] != DAT_005241f4[param_3]) {
            return 1;
        }
        break;
    case 0x13:
        if ((unsigned int)DAT_009d8c87[iVar1] != param_3) {
            return 1;
        }
        break;
    case 0x14:
        if ((unsigned int)DAT_009d8c8f[iVar1] != param_3) {
            return 1;
        }
        break;
    case 0x15:
        if ((unsigned int)DAT_009d8c88[iVar1] != param_3) {
            return 1;
        }
        break;
    case 0x17:
        if ((unsigned int)DAT_009d9156[iVar1] == param_3) {
            return 1;
        }
        break;
    case 0x18:
        if ((unsigned int)DAT_009d9156[iVar1] != param_3) {
            uVar2 = 1;
        }
    }
    return uVar2;
}
