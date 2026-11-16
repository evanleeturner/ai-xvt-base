/* SPDX-License-Identifier: MIT */
/* z_bop 0042e4d0 */
extern int DAT_00523414;
extern int DAT_00523410;
extern int DAT_009ecc34;
extern char DAT_009d80c9;
extern short DAT_00a0fe80[];
extern char DAT_00bb7371;
extern char DAT_00bb7372;
extern char DAT_00bb737a;
extern char DAT_00bb737b;
extern char DAT_009e9670[];
extern int DAT_009a1fe8;
extern char DAT_00a0adf0[];

void __cdecl FUN_004a9380(int param_1);
uint __cdecl FUN_0042e810(uint param_1, uint param_2);
int __cdecl FUN_0042e9a0(uint param_1, int *param_2);
uint __cdecl FUN_004a9300(uint param_1, int param_2);
void __cdecl FUN_0042d120(char *param_1, int param_2, int param_3, uint param_4, uint param_5, int param_6);

undefined4 __cdecl FUN_0042e4d0(uint param_1, uint param_2, int param_3)
{
    char cVar1;
    int iVar2;
    uint uVar3;
    int iVar4 = 0;
    uint uVar5;
    uint local_4;
    int tmp = 0;
    unsigned short us;

    if (DAT_00523414 == 0) {
        if (DAT_00523410 != 0) {
            return 0;
        }
    }
    else {
        if (DAT_00523410 == 0) {
            return 0;
        }
        if (DAT_00523414 == 2) {
            return 0;
        }
    }
    if (param_3 != DAT_009ecc34) {
        return 0;
    }
    if (DAT_009d80c9 == '\0') {
        return 0;
    }
    if (DAT_00a0fe80[param_1] == 0) {
        return 0;
    }
    if (param_2 == 0xffffffff) {
        if (DAT_00bb7372 == '\0') {
            return 0;
        }
        if (DAT_00bb737b == '\0') {
            return 0;
        }
    }
    else {
        if (DAT_00bb7371 == '\0') {
            return 0;
        }
        if (DAT_00bb737a == '\0') {
            return 0;
        }
    }
    if ((0x55 < param_1) && (param_1 < 0x5a)) {
        __builtin_memcpy(&iVar2, DAT_009e9670 + (int)((unsigned int)param_3 * 0x5bd), 4);
        if (iVar2 == -1) {
            return 0;
        }
        cVar1 = *(char *)((unsigned int)iVar2 * 0x23 + 3 + (unsigned int)DAT_009a1fe8);
        if ((cVar1 != '\x01') && (cVar1 != '\x02')) {
            return 0;
        }
    }
    if (param_1 == 0x51) {
        FUN_004a9380(0x50);
    }
    if (param_1 == 0x54) {
        FUN_004a9380(0x53);
    }
    local_4 = FUN_0042e810(param_2, param_1);
    if (local_4 != 0) {
        iVar2 = FUN_0042e9a0(param_2, (int *)&local_4);
        uVar5 = 0x7c;
        if (local_4 < 0x7d) {
            uVar5 = local_4;
        }
        if (param_2 != 0xffffffff) {
            iVar4 = (int)(param_2 * 0x23 + (unsigned int)DAT_009a1fe8);
            __builtin_memcpy(&tmp, (char *)iVar4 + 0x1b, 4);
        }
        if ((param_2 == 0xffffffff) || (tmp == DAT_009ecc34)) {
            uVar5 = ((0u - (uint)(param_1 == 0x25)) & 2) + 0x7d;
        }
        else {
            __builtin_memcpy(&iVar4, (char *)iVar4 + 0x1f, 4);
            if (iVar4 != 0) {
                __builtin_memcpy(&us, (char *)iVar4 + 0x85, 2);
                __builtin_memcpy(&tmp, (char *)((unsigned int)us * 0x23 + 0x1b + (unsigned int)DAT_009a1fe8), 4);
                if (tmp == DAT_009ecc34) {
                    uVar5 = 0x7d;
                }
            }
        }
        uVar3 = FUN_004a9300(param_1, 0x100);
        if (((uVar3 != 0) && (0x47 < param_1)) && (param_1 < 0x50)) {
            return 0;
        }
        FUN_0042d120(DAT_00a0adf0 + param_1 * 6, 1, 0, uVar5, local_4, iVar2);
    }
    return 1;
}
