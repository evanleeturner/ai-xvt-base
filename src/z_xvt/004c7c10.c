/* SPDX-License-Identifier: MIT */
/* z_xvt 004c7c10 */
extern char DAT_0081bcf0[];
extern char DAT_0081b740[];
extern char DAT_0081e082[];
extern unsigned char DAT_0081fb96[];
extern unsigned char DAT_0081fbfa[];
extern unsigned char DAT_0081fbe2[];
extern unsigned char DAT_0081fc2a[];
extern unsigned char DAT_0081fc12[];
extern char DAT_005321f4[];
extern char DAT_005321b8[];
extern char s__s___c_s__c_s_005321e4[];
extern char s_medlvl_d_005321f8[];
extern char s_citlvl_d_00532204[];
extern char s_rcitlvl_d_00532210[];

void FUN_004dcfd0(int *param_1, int param_2, int param_3, int param_4, int param_5);
char *FUN_004dcfa0(int param_1);
int _sprintf(char *buf, const char *fmt, ...);
void FUN_004da920(int param_1, char *param_2, int *param_3, unsigned int param_4);
void FUN_004d0e60(char *param_1, int param_2, int param_3);
void FUN_004da7c0(int param_1, char *param_2, int param_3, int param_4, unsigned int param_5);
void FUN_004dd030(int *param_1, int param_2, int param_3);
void FUN_004d9d20(int *param_1, char *param_2, char *param_3, int param_4);

unsigned int __stdcall FUN_004c7c10(void)
{
    char *pcVar1;
    int iVar2;
    int iVar3;
    int *piVar7;
    unsigned int uVar9;
    int local_10[4];
    int val;
    int local_idx;

    FUN_004dcfd0(local_10, 0x54, 0x5a, 0x194, 0x6a);
    if (DAT_0081bcf0[0] == '\0') {
        pcVar1 = FUN_004dcfa0(9);
        _sprintf(DAT_0081b740, DAT_005321f4, pcVar1);
    }
    else {
        pcVar1 = FUN_004dcfa0(9);
        _sprintf(DAT_0081b740, s__s___c_s__c_s_005321e4, pcVar1, 6, DAT_0081e082, 4, DAT_0081bcf0);
    }
    FUN_004da920(0xf, DAT_0081b740, local_10, 0xffff);
    if (DAT_0081bcf0[0] == '\0') {
        return 0;
    }
    FUN_004dcfd0(local_10, 0x10c, 0x76, 0x1a8, 0x84);
    piVar7 = local_10;
    uVar9 = 0xffff;
    pcVar1 = FUN_004dcfa0(0x16e);
    iVar3 = 0x85;
    FUN_004da920(0xc, pcVar1, piVar7, uVar9);
    iVar2 = 0;
    FUN_004dcfd0(local_10, 0x161, 0x85, 0x183, 0x92);
    do {
        __builtin_memcpy(&local_idx, DAT_0081fb96, 4);
        __builtin_memcpy(&val, DAT_0081fbfa + ((unsigned int)local_idx * 0x3a09u + (unsigned int)iVar2) * 4u, 4);
        if (val != 0) {
            _sprintf(DAT_0081b740, s_medlvl_d_005321f8, iVar2 + 1);
            FUN_004d0e60(DAT_0081b740, 0x161, iVar3);
            __builtin_memcpy(&local_idx, DAT_0081fb96, 4);
            __builtin_memcpy(&val, DAT_0081fbfa + ((unsigned int)local_idx * 0x3a09u + (unsigned int)iVar2) * 4u, 4);
            _sprintf(DAT_0081b740, DAT_005321b8, val);
            FUN_004da7c0(10, DAT_0081b740, 0x193, iVar3 + 1, 0xffff);
        }
        iVar3 = iVar3 + 0x16;
        iVar2 = iVar2 + 1;
        FUN_004dd030(local_10, 0, 0x16);
    } while (iVar2 < 6);
    FUN_004dcfd0(local_10, 0x61, 0x76, 0xfd, 0x84);
    piVar7 = local_10;
    uVar9 = 0xffff;
    pcVar1 = FUN_004dcfa0(0x16d);
    iVar3 = 0x85;
    FUN_004da920(0xc, pcVar1, piVar7, uVar9);
    iVar2 = 0;
    FUN_004dcfd0(local_10, 0xb6, 0x85, 0xd8, 0x92);
    do {
        __builtin_memcpy(&local_idx, DAT_0081fb96, 4);
        __builtin_memcpy(&val, DAT_0081fbe2 + ((unsigned int)local_idx * 0x3a09u + (unsigned int)iVar2) * 4u, 4);
        if (val != 0) {
            _sprintf(DAT_0081b740, s_medlvl_d_005321f8, iVar2 + 1);
            FUN_004d0e60(DAT_0081b740, 0xb6, iVar3);
            __builtin_memcpy(&local_idx, DAT_0081fb96, 4);
            __builtin_memcpy(&val, DAT_0081fbe2 + ((unsigned int)local_idx * 0x3a09u + (unsigned int)iVar2) * 4u, 4);
            _sprintf(DAT_0081b740, DAT_005321b8, val);
            FUN_004da7c0(10, DAT_0081b740, 0xe8, iVar3 + 1, 0xffff);
        }
        iVar3 = iVar3 + 0x16;
        iVar2 = iVar2 + 1;
        FUN_004dd030(local_10, 0, 0x16);
    } while (iVar2 < 6);
    FUN_004dcfd0(local_10, 0x10c, 0x10d, 0x1a8, 0x11b);
    piVar7 = local_10;
    uVar9 = 0xffff;
    pcVar1 = FUN_004dcfa0(0x170);
    iVar3 = 0x11c;
    FUN_004da920(0xc, pcVar1, piVar7, uVar9);
    iVar2 = 0;
    FUN_004dcfd0(local_10, 0x161, 0x11c, 0x183, 0x129);
    do {
        __builtin_memcpy(&local_idx, DAT_0081fb96, 4);
        __builtin_memcpy(&val, DAT_0081fc2a + ((unsigned int)local_idx * 0x3a09u + (unsigned int)iVar2) * 4u, 4);
        if (val != 0) {
            _sprintf(DAT_0081b740, s_medlvl_d_005321f8, iVar2 + 1);
            FUN_004d0e60(DAT_0081b740, 0x161, iVar3);
            __builtin_memcpy(&local_idx, DAT_0081fb96, 4);
            __builtin_memcpy(&val, DAT_0081fc2a + ((unsigned int)local_idx * 0x3a09u + (unsigned int)iVar2) * 4u, 4);
            _sprintf(DAT_0081b740, DAT_005321b8, val);
            FUN_004da7c0(10, DAT_0081b740, 0x193, iVar3 + 1, 0xffff);
        }
        iVar3 = iVar3 + 0x16;
        iVar2 = iVar2 + 1;
        FUN_004dd030(local_10, 0, 0x16);
    } while (iVar2 < 6);
    FUN_004dcfd0(local_10, 0x61, 0x10d, 0xfd, 0x11b);
    piVar7 = local_10;
    uVar9 = 0xffff;
    pcVar1 = FUN_004dcfa0(0x16f);
    iVar3 = 0x11c;
    FUN_004da920(0xc, pcVar1, piVar7, uVar9);
    iVar2 = 0;
    FUN_004dcfd0(local_10, 0xb6, 0x11c, 0xd8, 0x129);
    do {
        __builtin_memcpy(&local_idx, DAT_0081fb96, 4);
        __builtin_memcpy(&val, DAT_0081fc12 + ((unsigned int)local_idx * 0x3a09u + (unsigned int)iVar2) * 4u, 4);
        if (val != 0) {
            __builtin_memcpy(&local_idx, DAT_0081fb96, 4);
            if (local_idx == 0) {
                pcVar1 = s_rcitlvl_d_00532210;
            }
            else {
                pcVar1 = s_citlvl_d_00532204;
            }
            _sprintf(DAT_0081b740, pcVar1, iVar2 + 1);
            FUN_004d0e60(DAT_0081b740, 0xb6, iVar3);
            __builtin_memcpy(&local_idx, DAT_0081fb96, 4);
            __builtin_memcpy(&val, DAT_0081fc12 + ((unsigned int)local_idx * 0x3a09u + (unsigned int)iVar2) * 4u, 4);
            _sprintf(DAT_0081b740, DAT_005321b8, val);
            FUN_004da7c0(10, DAT_0081b740, 0xe8, iVar3 + 1, 0xffff);
        }
        iVar3 = iVar3 + 0x16;
        iVar2 = iVar2 + 1;
        FUN_004dd030(local_10, 0, 0x16);
    } while (iVar2 < 6);
    iVar2 = 0;
    FUN_004dcfd0(local_10, 0x161, 0x85, 0x183, 0x92);
    do {
        __builtin_memcpy(&local_idx, DAT_0081fb96, 4);
        __builtin_memcpy(&val, DAT_0081fbfa + ((unsigned int)local_idx * 0x3a09u + (unsigned int)iVar2) * 4u, 4);
        if (val != 0) {
            iVar3 = 0xc;
            pcVar1 = FUN_004dcfa0(iVar2 + 0x171);
            FUN_004d9d20(local_10, (char *)0, pcVar1, iVar3);
        }
        iVar2 = iVar2 + 1;
        FUN_004dd030(local_10, 0, 0x16);
    } while (iVar2 < 6);
    iVar2 = 0;
    FUN_004dcfd0(local_10, 0xb6, 0x85, 0xd8, 0x92);
    do {
        __builtin_memcpy(&local_idx, DAT_0081fb96, 4);
        __builtin_memcpy(&val, DAT_0081fbe2 + ((unsigned int)local_idx * 0x3a09u + (unsigned int)iVar2) * 4u, 4);
        if (val != 0) {
            iVar3 = 0xc;
            pcVar1 = FUN_004dcfa0(iVar2 + 0x171);
            FUN_004d9d20(local_10, (char *)0, pcVar1, iVar3);
        }
        iVar2 = iVar2 + 1;
        FUN_004dd030(local_10, 0, 0x16);
    } while (iVar2 < 6);
    iVar2 = 0;
    FUN_004dcfd0(local_10, 0x161, 0x11c, 0x183, 0x129);
    do {
        __builtin_memcpy(&local_idx, DAT_0081fb96, 4);
        __builtin_memcpy(&val, DAT_0081fc2a + ((unsigned int)local_idx * 0x3a09u + (unsigned int)iVar2) * 4u, 4);
        if (val != 0) {
            iVar3 = 0xc;
            pcVar1 = FUN_004dcfa0(iVar2 + 0x171);
            FUN_004d9d20(local_10, (char *)0, pcVar1, iVar3);
        }
        iVar2 = iVar2 + 1;
        FUN_004dd030(local_10, 0, 0x16);
    } while (iVar2 < 6);
    iVar2 = 0;
    FUN_004dcfd0(local_10, 0xb6, 0x11c, 0xd8, 0x129);
    do {
        __builtin_memcpy(&local_idx, DAT_0081fb96, 4);
        __builtin_memcpy(&val, DAT_0081fc12 + ((unsigned int)local_idx * 0x3a09u + (unsigned int)iVar2) * 4u, 4);
        if (val != 0) {
            iVar3 = 0xc;
            pcVar1 = FUN_004dcfa0(iVar2 + 0x286);
            FUN_004d9d20(local_10, (char *)0, pcVar1, iVar3);
        }
        iVar2 = iVar2 + 1;
        FUN_004dd030(local_10, 0, 0x16);
    } while (iVar2 < 6);
    return 1;
}
