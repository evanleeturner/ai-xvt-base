/* SPDX-License-Identifier: MIT */
/* z_xvt 004d6320 */
extern unsigned char DAT_00757e78[];
extern int DAT_0064f650;
extern int DAT_00758824;
extern int DAT_00859c2c;
extern int DAT_00532a34;
extern unsigned char DAT_00532a48[];
extern unsigned short DAT_0081bcd0;
extern unsigned short DAT_0081b854;
extern unsigned short DAT_0081bce8;
extern unsigned short DAT_0081b6fc;
extern char DAT_0081b740[];
extern char DAT_005321b8[];
extern int DAT_0081bcb0;
extern char DAT_0085e7c5;
extern char s_jewelsound_00531990[];
extern char s__s_v___d__d_00532a3c[];

DWORD __stdcall GetTickCount(void);
int __cdecl _sprintf(char *, const char *, ...);
void __cdecl FUN_004dcfd0(int *, int, int, int, int);
char * __cdecl FUN_004dcfa0(int);
void __cdecl FUN_004daac0(int, char *, int *, int, int, unsigned int);
void __cdecl FUN_004ddc70(int *, int *);
int __cdecl FUN_004d9010(int *, int, int, int, int, int, int);
void __cdecl FUN_004dd3e0(int *, int, int, short, int);
void __cdecl FUN_004d0e60(void *, int, int);
void __cdecl FUN_004dd000(int *, int *);
void __cdecl FUN_004c9300(int);
int __cdecl FUN_004dd7d0(int *, int, int);
void __cdecl FUN_004dd570(int *, int, int, short);
unsigned char __cdecl FUN_004db570(void);
unsigned char __cdecl FUN_004db590(void);
void __cdecl FUN_004de190(char *, int, int, int);
void __cdecl FUN_004dd030(int *, int, int);

int __cdecl FUN_004d6320(int param_1)
{
    unsigned char uVar1;
    char *pcVar2;
    int iVar3;
    unsigned char *pbVar4;
    unsigned char *pbVar5;
    int *piVar6;
    int iVar7;
    int iVar8;
    unsigned int uVar9;
    int aiRect_30[4];
#define iStack_30 aiRect_30[0]
#define iStack_2c aiRect_30[1]
    int iStack_20;
    DWORD DStack_1c;
    int iStack_18;
    int aiStack_14[5];
    unsigned int uTmp;

    if (param_1 == 0) {
        DAT_0064f650 = 0;
    }
    DStack_1c = GetTickCount();
    FUN_004dcfd0(&iStack_30, 0x58, 0x5e, 0x1a0, 0x6d);
    piVar6 = &iStack_30;
    uVar9 = 0xffff;
    iVar8 = 1;
    iVar7 = 0;
    pcVar2 = FUN_004dcfa0(0x10);
    FUN_004daac0(0xc, pcVar2, piVar6, iVar7, iVar8, uVar9);
    FUN_004ddc70(aiStack_14, &iStack_18);
    if (6 < DAT_00758824) {
        FUN_004dcfd0(&iStack_30, 0x1a5, 0x72, 0x1ae, 0xcc);
        DAT_0064f650 = FUN_004d9010(&iStack_30, DAT_0064f650, DAT_00758824, 0, 5, DAT_00859c2c, 5);
    }
    FUN_004dcfd0(&iStack_30, 0x58, 0x72, 0x1a3, 0x80);
    iStack_20 = -1;
    if (DAT_0064f650 < (int)((unsigned int)DAT_0064f650 + 6u)) {
        pbVar4 = DAT_00757e78 + (unsigned int)DAT_0064f650 * 0x3bu;
        iVar7 = DAT_0064f650;
        do {
            if (DAT_00758824 <= iVar7) {
                return iStack_20;
            }
            if (iVar7 == DAT_00532a34) {
                FUN_004dd3e0(&iStack_30, 0, 0, (short)DAT_00859c2c, 1);
            }
            if (*pbVar4 == 2) {
                __builtin_memcpy(&uTmp, pbVar4 - 8, 4);
                if (uTmp == 0) {
                    if (pbVar4[2] == 0) {
                        uVar9 = (unsigned int)DAT_0081bcd0;
                    } else {
                        uVar9 = (unsigned int)DAT_0081b854;
                    }
                } else {
                    uVar9 = 0xffff;
                    __builtin_memcpy(&uTmp, pbVar4 - 8, 4);
                    if (uTmp < 9) {
                        uVar9 = (unsigned int)DAT_0081bce8;
                    }
                }
            } else {
                uVar9 = (unsigned int)DAT_0081b6fc;
            }
            if (pbVar4[1] != 0) {
                FUN_004d0e60(DAT_00532a48, (int)((unsigned int)iStack_30 - 6u), (int)((unsigned int)iStack_2c + 1u));
            }
            if (*pbVar4 == 2) {
                pbVar5 = pbVar4 - 0x38;
            } else {
                iVar8 = (int)*pbVar4 - 1;
                _sprintf(DAT_0081b740, s__s_v___d__d_00532a3c, pbVar4 - 0x38, iVar8 / 100 + 1,
                         iVar8 % 100);
                pbVar5 = (unsigned char *)DAT_0081b740;
            }
            FUN_004daac0(0xc, (char *)pbVar5, &iStack_30, 0, 1, uVar9);
            __builtin_memcpy(&uTmp, pbVar4 - 8, 4);
            if (uTmp < 9) {
                FUN_004dd000(aiStack_14 + 1, &iStack_30);
                aiStack_14[1] = 0x11d;
                __builtin_memcpy(&uTmp, pbVar4 - 8, 4);
                _sprintf(DAT_0081b740, DAT_005321b8, uTmp);
                FUN_004daac0(0xc, DAT_0081b740, aiStack_14 + 1, 0, 1, uVar9);
                aiStack_14[1] = 0x176;
                __builtin_memcpy(&uTmp, pbVar4 - 4, 4);
                FUN_004c9300((int)((DStack_1c - uTmp) / 1000u));
                FUN_004daac0(0xc, DAT_0081b740, aiStack_14 + 1, 0, 1, uVar9);
            }
            iVar3 = FUN_004dd7d0(&iStack_30, aiStack_14[0], iStack_18);
            iVar8 = iStack_20;
            if (iVar3 != 0) {
                FUN_004dd570(&iStack_30, 0, 0, (short)DAT_0081bcb0);
                uVar1 = FUN_004db570();
                if (uVar1 != 0 || (uVar1 = FUN_004db590(), uVar1 != 0)) {
                    iVar8 = iVar7;
                    if (DAT_0085e7c5 != '\0') {
                        FUN_004de190(s_jewelsound_00531990, 1, 0, 0xff);
                    }
                }
            }
            iStack_20 = iVar8;
            pbVar4 = pbVar4 + 0x3b;
            iVar7 = iVar7 + 1;
            FUN_004dd030(&iStack_30, 0, 0xf);
        } while (iVar7 < (int)((unsigned int)DAT_0064f650 + 6u));
    }
    return iStack_20;
#undef iStack_30
#undef iStack_2c
}
