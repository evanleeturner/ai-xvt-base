/* SPDX-License-Identifier: MIT */
/* z_xvt 0040b190 */
extern int DAT_00525550;
extern char DAT_0070a189;
extern char DAT_0085e792;
extern unsigned char DAT_0085e79b;
extern int DAT_0071ecf4;
extern unsigned char DAT_0071b730[];
extern int DAT_006d4988;
extern unsigned char DAT_0073c018[];
extern unsigned char DAT_0073c060[];

unsigned int FUN_00439830(int, int);
void FUN_004398b0(int);
void FUN_00490a50(unsigned char *, int, int, int, unsigned int, int);
unsigned short FUN_004784d0(void);

void __stdcall FUN_0040b190(void)
{
    int iVar1;
    int iTmp;
    int iField;
    unsigned short uVar2;
    unsigned int uVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    unsigned int uAddr;

    if ((((DAT_00525550 == 0) && (DAT_0070a189 != '\0')) && (DAT_0085e792 != '\0')) &&
        (DAT_0085e79b != 0)) {
        uVar5 = 0x7f;
        if (DAT_0085e79b < 10) {
            uVar5 = (unsigned int)DAT_0085e79b * 0xd;
        }
        __builtin_memcpy(&iVar1, DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bdU, 4);
        if (iVar1 == -1) {
            uVar5 = FUN_00439830(0x3f, 0x100);
            if (uVar5 != 0) {
                FUN_004398b0(0x3f);
            }
            uVar5 = FUN_00439830(0x40, 0x100);
            if (uVar5 != 0) {
                FUN_004398b0(0x40);
            }
            uVar5 = FUN_00439830(0x41, 0x100);
            if (uVar5 != 0) {
                FUN_004398b0(0x41);
            }
            uVar5 = FUN_00439830(0x42, 0x100);
            if (uVar5 != 0) {
                FUN_004398b0(0x42);
                return;
            }
        }
        else {
            uAddr = (unsigned int)iVar1 * 0x23U + 0x1fU + (unsigned int)DAT_006d4988;
            __builtin_memcpy(&iTmp, (void *)uAddr, 4);
            uAddr = (unsigned int)iTmp + 0xa7U;
            __builtin_memcpy(&iVar1, (void *)uAddr, 4);

            uAddr = (unsigned int)iVar1 + 0x19U;
            __builtin_memcpy(&iField, (void *)uAddr, 4);
            if (iField == 0) {
                uVar3 = FUN_00439830(0x3f, 0x100);
                if (uVar3 != 0) {
                    FUN_004398b0(0x3f);
                }
                uVar3 = FUN_00439830(0x40, 0x100);
                if (uVar3 != 0) {
                    FUN_004398b0(0x40);
                }
                uVar3 = FUN_00439830(0x41, 0x100);
                if (uVar3 != 0) {
                    FUN_004398b0(0x41);
                }
            }
            else {
                uVar3 = FUN_00439830(0x3f, 0x100);
                if (uVar3 == 0) {
                    FUN_00490a50(DAT_0073c018, 1, 1, 0x7d, uVar5, 0x40);
                }
                uVar3 = FUN_00439830(0x40, 0x100);
                if (uVar3 == 0) {
                    uVar3 = FUN_00439830(0x41, 0x100);
                    if (uVar3 == 0) {
                        uVar2 = FUN_004784d0();
                        if (uVar2 < 0x1000) {
                            uVar3 = uVar5;
                            uVar4 = FUN_004784d0();
                            FUN_00490a50(DAT_0073c018 + (uVar4 & 1) * 0x18, 1, 1, 0x7d, uVar3, 0x40);
                        }
                    }
                }
            }
            uAddr = (unsigned int)iVar1 + 0x1dU;
            __builtin_memcpy(&iField, (void *)uAddr, 4);
            if (iField == 0) {
                uVar5 = FUN_00439830(0x42, 0x100);
                if (uVar5 != 0) {
                    FUN_004398b0(0x42);
                }
            }
            else {
                uVar3 = FUN_00439830(0x42, 0x100);
                if (uVar3 == 0) {
                    FUN_00490a50(DAT_0073c060, 1, 1, 0x7d, uVar5 / 2, 0x40);
                    return;
                }
            }
        }
    }
    return;
}
