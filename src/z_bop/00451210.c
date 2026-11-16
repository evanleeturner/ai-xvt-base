/* SPDX-License-Identifier: MIT */
/* z_bop 00451210 */
extern short DAT_009d8b80[];
extern char DAT_009d8b86;
extern char DAT_009d8b88;
extern char DAT_009d8b8a;
extern int DAT_009ecc34;
extern short DAT_009a6ff0;
extern int DAT_00556aa4;
extern short DAT_00a08ba4;
extern char DAT_009d77f8;
extern unsigned char DAT_005233c8;
extern int DAT_009a806c;
extern int DAT_009a7bac;
extern int DAT_009a8c14;
extern int DAT_005233d8;
extern int DAT_005235dc;
extern int DAT_009d1148;
extern int DAT_009a7ec0;
extern int DAT_009a7b44;
extern void (*DAT_009d1300)(void);
extern unsigned char DAT_009a6ff9;
extern short DAT_005569f8;
extern int DAT_009a7390;
extern int DAT_009d12f8;
extern int DAT_009a8d98;
extern int DAT_009a8d94;
extern short DAT_00556a50;
extern int DAT_009ec470;
extern int DAT_009ecc38;
extern int DAT_009d12f4;
extern int DAT_009d6934;
extern char DAT_009e968a;
extern char DAT_009e9a77;
extern unsigned int DAT_009d7694;
extern char DAT_009e9680;
extern char DAT_009ec468;

void FUN_00450c30(void);
void FUN_00450c90(unsigned short a);
void FUN_004a95c0(unsigned int a);
void FUN_0040e0c0(int a, int b, int c, int d);
void FUN_004a9520(short a, short b, short c, short d);
/* Ghidra showed two extra phantom args (unaff_EBX, unaff_retaddr); not real parameters. */
unsigned int FUN_00451f50(unsigned int a, int b, int c, int d);

void FUN_00451210(void)
{
    char cVar1;
    short sVar2;
    unsigned int uVar3;
    char *pcVar4;
    int iVar5;

    if (((((DAT_009d8b80[(int)((unsigned int)DAT_009ecc34 * 8U)] == 0) && (DAT_009a6ff0 != -1)) ||
          (DAT_00556aa4 != 0)) && (DAT_00a08ba4 == 0))) {
        if (DAT_009d77f8 == '\0') {
            FUN_004a95c0((unsigned int)DAT_005233c8);
            FUN_0040e0c0(DAT_009a806c, DAT_009a7bac, DAT_009a8c14,
                         (int)((unsigned int)DAT_005233d8 * (unsigned int)DAT_009a7bac));
            FUN_004a9520((short)DAT_005235dc, (short)DAT_009d1148,
                         (short)DAT_009a7ec0, (short)DAT_009a7b44);
            (*DAT_009d1300)();
            FUN_0040e0c0(0, 0x140, 200, 0);
            DAT_009a6ff0 = -1;
        } else {
            FUN_00450c30();
            FUN_00450c90((unsigned short)DAT_009a6ff9);
        }
    }
    if (((*(short *)(&DAT_009d8b86 + (int)((unsigned int)DAT_009ecc34 * 0x10U)) == 0) && (DAT_005569f8 != -1)) ||
        (DAT_00556aa4 != 0)) {
        FUN_004a95c0((unsigned int)DAT_005233c8);
        FUN_0040e0c0(DAT_009a806c, DAT_009a7bac, DAT_009a8c14,
                     (int)((unsigned int)DAT_005233d8 * (unsigned int)DAT_009a7bac));
        FUN_004a9520((short)DAT_009a7390, (short)DAT_009d12f8,
                     (short)DAT_009a8d98, (short)DAT_009a8d94);
        (*DAT_009d1300)();
        FUN_0040e0c0(0, 0x140, 200, 0);
        DAT_005569f8 = -1;
    }
    if (((*(short *)(&DAT_009d8b88 + (int)((unsigned int)DAT_009ecc34 * 0x10U)) == 0) && (DAT_00556a50 != -1)) ||
        (DAT_00556aa4 != 0)) {
        FUN_004a95c0((unsigned int)DAT_005233c8);
        FUN_0040e0c0(DAT_009a806c, DAT_009a7bac, DAT_009a8c14,
                     (int)((unsigned int)DAT_005233d8 * (unsigned int)DAT_009a7bac));
        FUN_004a9520((short)DAT_009ec470, (short)DAT_009ecc38,
                     (short)DAT_009d12f4, (short)DAT_009d6934);
        (*DAT_009d1300)();
        FUN_0040e0c0(0, 0x140, 200, 0);
        DAT_00556a50 = -1;
    }
    if (*(short *)(&DAT_009d8b8a + (int)((unsigned int)DAT_009ecc34 * 0x10U)) == 0) {
        iVar5 = (int)((unsigned int)DAT_009ecc34 * 0x5bdU);
        __builtin_memcpy(&sVar2, &DAT_009e968a + iVar5, 2);
        if (sVar2 != -1) {
            uVar3 = FUN_00451f50((unsigned int)(unsigned short)sVar2, DAT_009ecc34, 0, 0);
            if (uVar3 != DAT_009d7694) {
                __builtin_memcpy(&sVar2, &DAT_009e968a + iVar5, 2);
                uVar3 = FUN_00451f50((unsigned int)(unsigned short)sVar2, DAT_009ecc34, 0, 1);
                if (uVar3 != 0) {
                    cVar1 = (&DAT_009e9a77)[iVar5];
                    if (((cVar1 == '\x13') || (cVar1 == '\0')) || (cVar1 == '\x14')) {
                        __builtin_memcpy(&sVar2, &DAT_009e968a + iVar5, 2);
                        DAT_009d7694 = FUN_00451f50((unsigned int)(unsigned short)sVar2,
                                                    DAT_009ecc34, 1, 0);
                        *(short *)(&DAT_009d8b8a + (int)((unsigned int)DAT_009ecc34 * 0x10U)) = 0x49c;
                    }
                }
            }
        }
    }
    pcVar4 = &DAT_009e9680;
    do {
        int iTmp;
        __builtin_memcpy(&iTmp, pcVar4 + 0x59d, 4);
        if ((*pcVar4 != '\0') && (iTmp == 0)) {
            pcVar4[0x36] = '\0';
        }
        pcVar4 = pcVar4 + 0x5bd;
    } while (pcVar4 < &DAT_009ec468);
    if (DAT_00556aa4 != 0) {
        DAT_00556aa4 = 0;
    }
    return;
}
