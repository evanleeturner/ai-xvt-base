/* SPDX-License-Identifier: MIT */
/* z_bop 0042f5d0 */
extern unsigned int DAT_00523410;
extern unsigned char DAT_009d80c9;
extern unsigned char DAT_00bb7372;
extern unsigned char DAT_00bb737b;
extern unsigned int DAT_009ecc34;
extern unsigned int DAT_009a1fe8;
extern unsigned char DAT_009e9670[];
extern unsigned char DAT_00a0b3d8[];
extern unsigned char DAT_00a0b420[];

unsigned int FUN_004a9300(unsigned int a, unsigned int b);
void FUN_004a9380(unsigned int a);
void FUN_0042d120(unsigned char *p, unsigned int a, unsigned int b, unsigned int c, unsigned int d, unsigned int e);
unsigned int FUN_00425d10(void);

void __stdcall FUN_0042f5d0(void)
{
    unsigned int iVar1;
    unsigned int uVar3;
    unsigned int uVar4;
    unsigned int uVar5;

    if (DAT_00523410 == 0 && DAT_009d80c9 != '\0' && DAT_00bb7372 != '\0' &&
        DAT_00bb737b != 0) {
        uVar5 = 0x7f;
        if (DAT_00bb737b < 10) {
            uVar5 = (unsigned int)DAT_00bb737b * 0xd;
        }
        __builtin_memcpy(&iVar1, DAT_009e9670 + DAT_009ecc34 * 0x5bd, 4);
        if (iVar1 == 0xffffffffu) {
            uVar5 = FUN_004a9300(0x3f, 0x100);
            if (uVar5 != 0) {
                FUN_004a9380(0x3f);
            }
            uVar5 = FUN_004a9300(0x40, 0x100);
            if (uVar5 != 0) {
                FUN_004a9380(0x40);
            }
            uVar5 = FUN_004a9300(0x41, 0x100);
            if (uVar5 != 0) {
                FUN_004a9380(0x41);
            }
            uVar5 = FUN_004a9300(0x42, 0x100);
            if (uVar5 != 0) {
                FUN_004a9380(0x42);
                return;
            }
        } else {
            unsigned int p;
            unsigned int f19;
            unsigned int f1d;

            __builtin_memcpy(&p, (unsigned char *)(iVar1 * 0x23 + 0x1f + DAT_009a1fe8), 4);
            __builtin_memcpy(&iVar1, (unsigned char *)(p + 0xa9), 4);
            __builtin_memcpy(&f19, (unsigned char *)(iVar1 + 0x19), 4);
            if (f19 == 0) {
                uVar3 = FUN_004a9300(0x3f, 0x100);
                if (uVar3 != 0) {
                    FUN_004a9380(0x3f);
                }
                uVar3 = FUN_004a9300(0x40, 0x100);
                if (uVar3 != 0) {
                    FUN_004a9380(0x40);
                }
                uVar3 = FUN_004a9300(0x41, 0x100);
                if (uVar3 != 0) {
                    FUN_004a9380(0x41);
                }
            } else {
                uVar3 = FUN_004a9300(0x3f, 0x100);
                if (uVar3 == 0) {
                    FUN_0042d120(DAT_00a0b3d8, 1, 1, 0x7d, uVar5, 0x40);
                }
                uVar3 = FUN_004a9300(0x40, 0x100);
                if (uVar3 == 0) {
                    uVar3 = FUN_004a9300(0x41, 0x100);
                    if (uVar3 == 0) {
                        if ((unsigned short)FUN_00425d10() < 0x1000) {
                            uVar3 = uVar5;
                            uVar4 = FUN_00425d10();
                            FUN_0042d120(DAT_00a0b3d8 + (uVar4 & 1) * 0x18, 1, 1, 0x7d, uVar3, 0x40);
                        }
                    }
                }
            }
            __builtin_memcpy(&f1d, (unsigned char *)(iVar1 + 0x1d), 4);
            if (f1d == 0) {
                uVar5 = FUN_004a9300(0x42, 0x100);
                if (uVar5 != 0) {
                    FUN_004a9380(0x42);
                }
            } else {
                uVar3 = FUN_004a9300(0x42, 0x100);
                if (uVar3 == 0) {
                    FUN_0042d120(DAT_00a0b420, 1, 1, 0x7d, uVar5 / 2, 0x40);
                    return;
                }
            }
        }
    }
    return;
}
