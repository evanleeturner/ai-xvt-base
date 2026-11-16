/* SPDX-License-Identifier: MIT */
/* z_xvt 004f25e0 */
/* Prototypes for every callee */
extern void   FUN_004e43d0(int param_1);
extern int    _sprintf(char *buf, const char *fmt, ...);
extern void  *FUN_004be160(char *param_1, void *param_2);
extern void   FUN_004be2c0(void *param_1, void *param_2);
extern void   FUN_004be1f0(void *param_1);
extern void   FUN_004be320(void *param_1, void *param_2, int param_3);
extern void   FUN_004be290(void *param_1, char *param_2);
extern void   FUN_004be220(void *param_1, int param_2, int param_3);

/* Globals. Those at addresses not aligned to their width are byte arrays,
   accessed only through __builtin_memcpy. */
extern unsigned char DAT_0081bfba[4];   /* int, address mod 4 = 2 */
extern unsigned char DAT_0081bfb6[4];   /* unsigned int, address mod 4 = 2 */
extern unsigned char DAT_0081bfbe[];    /* int array, address mod 4 = 2 */
extern int           DAT_00757cc0;      /* used as a base address (pointer held as int) */
extern int           DAT_00757cc8;
extern int           DAT_00757ccc;
extern char         *PTR_s_train_005330e0[];
extern char          s__s__s_005331e4[];
extern undefined     DAT_00523570;      /* file mode string, type unresolved */
extern undefined4    DAT_00743880[];
extern short         DAT_00757672;
extern short         DAT_0075766c;
extern short         DAT_0075766e;
extern undefined     DAT_007562dc;
extern undefined     DAT_00753ae0[];
extern undefined     DAT_00755760[];
extern undefined2    DAT_0075637a;
extern undefined4    DAT_00650478[];
extern int           DAT_00650434;
extern unsigned char DAT_006507a2[];    /* 32 packed pointers, address mod 4 = 2 */
extern unsigned char DAT_00650822[];    /* 32 packed pointers, address mod 4 = 2 */
extern unsigned char DAT_006508a2[];    /* end marker, address mod 4 = 2 */

void __stdcall FUN_004f25e0(void)
{
    void *pFVar1;
    int iVar2;
    int *piVar3;
    unsigned int uVar4;
    undefined *puVar5;
    undefined2 *puVar6;
    undefined4 *puVar7;
    undefined *puVar8;
    undefined4 *puVar9;
    undefined4 *puVar10;
    unsigned char *pbVar11;
    int iTrain;
    int iTmp;
    unsigned int uTmp;
    char cStack_439;
    short sStack_438;
    short sStack_436;
    int iStack_434;
    int iStack_430;
    char acStack_42c[256];
    undefined4 auStack_32c[203];

    __builtin_memcpy(&iTrain, DAT_0081bfba, 4);
    FUN_004e43d0(iTrain);
    if (DAT_00757cc0 != 0) {
        DAT_00757cc8 = 0;
        if (DAT_00757ccc != 0) {
            piVar3 = (int *)(DAT_00757cc0 + 0x140);
            do {
                __builtin_memcpy(&iTrain, DAT_0081bfba, 4);
                __builtin_memcpy(&iTmp, DAT_0081bfbe + (unsigned int)iTrain * 4u, 4);
                if (*piVar3 == iTmp) {
                    break;
                }
                piVar3 = piVar3 + 0x51;
                DAT_00757cc8 = DAT_00757cc8 + 1;
            } while ((unsigned int)DAT_00757cc8 < (unsigned int)DAT_00757ccc);
        }
    }
    __builtin_memcpy(&iTrain, DAT_0081bfba, 4);
    _sprintf(acStack_42c, s__s__s_005331e4, PTR_s_train_005330e0[iTrain],
             (int)((unsigned int)DAT_00757cc8 * 0x144u + (unsigned int)DAT_00757cc0));
    pFVar1 = FUN_004be160(acStack_42c, &DAT_00523570);
    if (pFVar1 != (void *)0) {
        puVar7 = DAT_00743880;
        for (iVar2 = 0x4f7d; iVar2 != 0; iVar2 = iVar2 - 1) {
            *puVar7 = 0;
            puVar7 = puVar7 + 1;
        }
        FUN_004be2c0(pFVar1, &DAT_00757672);
        if (DAT_00757672 != 0xc) {
            FUN_004be1f0(pFVar1);
            return;
        }
        FUN_004be2c0(pFVar1, &DAT_0075766c);
        FUN_004be2c0(pFVar1, &DAT_0075766e);
        iVar2 = 0;
        FUN_004be320(pFVar1, &DAT_007562dc, 0x9e);
        if (0 < DAT_0075766c) {
            puVar7 = DAT_00743880;
            do {
                iVar2 = iVar2 + 1;
                FUN_004be320(pFVar1, puVar7, 0x562);
                puVar7 = (undefined4 *)((int)puVar7 + 0x562);
            } while (iVar2 < DAT_0075766c);
        }
        iVar2 = 0;
        if (0 < DAT_0075766e) {
            do {
                iVar2 = iVar2 + 1;
                FUN_004be2c0(pFVar1, &sStack_438);
                FUN_004be320(pFVar1, DAT_00753ae0 + sStack_438 * 0x72, 0x72);
            } while (iVar2 < DAT_0075766e);
        }
        puVar8 = DAT_00755760;
        do {
            iVar2 = 0;
            FUN_004be2c0(pFVar1, &sStack_438);
            puVar5 = puVar8;
            if (0 < sStack_438) {
                do {
                    iVar2 = iVar2 + 1;
                    FUN_004be320(pFVar1, puVar5, 0x2a);
                    puVar5 = puVar5 + 0x2a;
                } while (iVar2 < sStack_438);
            }
            puVar8 = puVar8 + 0x126;
        } while (puVar8 < &DAT_007562dc);
        puVar6 = &DAT_0075637a;
        do {
            FUN_004be2c0(pFVar1, &sStack_438);
            if (sStack_438 != 0) {
                FUN_004be320(pFVar1, puVar6, 0x1e5);
            }
            puVar6 = (undefined2 *)((int)puVar6 + 0x1e5);
        } while ((undefined *)puVar6 < (undefined *)&DAT_0075766c);
        iStack_430 = 0;
        do {
            uVar4 = 0;
            iStack_434 = 0;
            FUN_004be320(pFVar1, auStack_32c, 0x32a);
            do {
                FUN_004be290(pFVar1, &cStack_439);
                __builtin_memcpy(&uTmp, DAT_0081bfb6, 4);
                if ((uVar4 == uTmp) && (cStack_439 != '\0')) {
                    puVar7 = auStack_32c;
                    puVar9 = DAT_00650478;
                    for (iVar2 = 0xca; iVar2 != 0; iVar2 = iVar2 - 1) {
                        *puVar9 = *puVar7;
                        puVar7 = puVar7 + 1;
                        puVar9 = puVar9 + 1;
                    }
                    *(undefined2 *)puVar9 = *(undefined2 *)puVar7;
                    iStack_434 = 1;
                    DAT_00650434 = iStack_430;
                }
                uVar4 = uVar4 + 1;
            } while (uVar4 < 10);
            pbVar11 = DAT_006507a2;
            do {
                __builtin_memcpy(&puVar9, pbVar11, 4);
                if (iStack_434 != 0) {
                    puVar10 = puVar9;
                    for (iVar2 = 10; iVar2 != 0; iVar2 = iVar2 - 1) {
                        *puVar10 = 0;
                        puVar10 = puVar10 + 1;
                    }
                }
                FUN_004be2c0(pFVar1, &sStack_436);
                if (sStack_436 != 0) {
                    if (iStack_434 == 0) {
                        FUN_004be220(pFVar1, (int)sStack_436, 1);
                    } else {
                        FUN_004be320(pFVar1, puVar9, (int)sStack_436);
                    }
                }
                if (iStack_434 != 0) {
                    *(undefined1 *)((int)sStack_436 + (int)puVar9) = 0;
                }
                pbVar11 = pbVar11 + 4;
            } while (pbVar11 < DAT_00650822);
            pbVar11 = DAT_00650822;
            do {
                __builtin_memcpy(&puVar9, pbVar11, 4);
                if (iStack_434 != 0) {
                    puVar10 = puVar9;
                    for (iVar2 = 0x50; iVar2 != 0; iVar2 = iVar2 - 1) {
                        *puVar10 = 0;
                        puVar10 = puVar10 + 1;
                    }
                }
                FUN_004be2c0(pFVar1, &sStack_436);
                if (sStack_436 != 0) {
                    if (iStack_434 == 0) {
                        FUN_004be220(pFVar1, (int)sStack_436, 1);
                    } else {
                        FUN_004be320(pFVar1, puVar9, (int)sStack_436);
                    }
                }
                if (iStack_434 != 0) {
                    *(undefined1 *)((int)sStack_436 + (int)puVar9) = 0;
                }
                pbVar11 = pbVar11 + 4;
            } while (pbVar11 < DAT_006508a2);
            iStack_430 = iStack_430 + 1;
        } while (iStack_430 < 8);
        FUN_004be1f0(pFVar1);
    }
    return;
}
