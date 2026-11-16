/* SPDX-License-Identifier: MIT */
/* z_xvt 004f2990 */
extern unsigned char DAT_0081bfba[4];   /* int at unaligned 0x81bfba */
extern unsigned char DAT_0081bfbe[];    /* int table at unaligned 0x81bfbe */
extern int DAT_00757cc0;
extern int DAT_00757cc8;
extern int DAT_00757ccc;
extern char *PTR_s_train_005330e0[];
extern char s__s__s_005331e4[];
extern char DAT_00523570[];
extern short DAT_00757672;
extern short DAT_0075766c;
extern short DAT_0075766e;
extern unsigned char DAT_007562dc[];
extern unsigned char DAT_00743880[];
extern unsigned char DAT_00753ae0[];
extern unsigned char DAT_00755760[];
extern unsigned char DAT_0075637a[];

/* Calling conventions of these callees are not confirmed by the evidence; default cdecl assumed. */
void FUN_004e43d0(int param_1);
int _sprintf(char *, const char *, ...);
void *FUN_004be160(char *param_1, char *param_2);
void FUN_004be2c0(void *param_1, void *param_2);
void FUN_004be1f0(void *param_1);
void FUN_004be320(void *param_1, void *param_2, int param_3);

void __stdcall FUN_004f2990(void)
{
    void *pFVar1;
    int iVar2;
    int *piVar3;
    unsigned char *puVar4;
    unsigned int *puVar5;
    unsigned char *puVar6;
    unsigned char *puVar7;
    short sStack_102;
    char acStack_100[256];
    int iIdx;
    int iTbl;

    __builtin_memcpy(&iIdx, DAT_0081bfba, sizeof(int));
    FUN_004e43d0(iIdx);
    if ((DAT_00757cc0 != 0) && (DAT_00757cc8 = 0, DAT_00757ccc != 0)) {
        piVar3 = (int *)(DAT_00757cc0 + 0x140);
        do {
            __builtin_memcpy(&iIdx, DAT_0081bfba, sizeof(int));
            __builtin_memcpy(&iTbl, DAT_0081bfbe + (unsigned int)iIdx * 4u, sizeof(int));
            if (*piVar3 == iTbl) break;
            piVar3 = piVar3 + 0x51;
            DAT_00757cc8 = DAT_00757cc8 + 1;
        } while ((unsigned int)DAT_00757cc8 < (unsigned int)DAT_00757ccc);
    }
    __builtin_memcpy(&iIdx, DAT_0081bfba, sizeof(int));
    _sprintf(acStack_100, s__s__s_005331e4, PTR_s_train_005330e0[iIdx],
             (char *)((unsigned int)DAT_00757cc8 * 0x144u + (unsigned int)DAT_00757cc0));
    pFVar1 = FUN_004be160(acStack_100, DAT_00523570);
    if (pFVar1 != (void *)0) {
        puVar5 = (unsigned int *)DAT_00743880;
        for (iVar2 = 0x4f7d; iVar2 != 0; iVar2 = iVar2 + -1) {
            *puVar5 = 0;
            puVar5 = puVar5 + 1;
        }
        FUN_004be2c0(pFVar1, &DAT_00757672);
        if (DAT_00757672 != 0xc) {
            FUN_004be1f0(pFVar1);
            return;
        }
        FUN_004be2c0(pFVar1, &DAT_0075766c);
        FUN_004be2c0(pFVar1, &DAT_0075766e);
        iVar2 = 0;
        FUN_004be320(pFVar1, DAT_007562dc, 0x9e);
        if (0 < DAT_0075766c) {
            puVar6 = DAT_00743880;
            do {
                iVar2 = iVar2 + 1;
                FUN_004be320(pFVar1, puVar6, 0x562);
                puVar6 = puVar6 + 0x562;
            } while (iVar2 < DAT_0075766c);
        }
        iVar2 = 0;
        if (0 < DAT_0075766e) {
            do {
                iVar2 = iVar2 + 1;
                FUN_004be2c0(pFVar1, &sStack_102);
                FUN_004be320(pFVar1, DAT_00753ae0 + sStack_102 * 0x72, 0x72);
            } while (iVar2 < DAT_0075766e);
        }
        puVar4 = DAT_00755760;
        do {
            iVar2 = 0;
            FUN_004be2c0(pFVar1, &sStack_102);
            puVar6 = puVar4;
            if (0 < sStack_102) {
                do {
                    iVar2 = iVar2 + 1;
                    FUN_004be320(pFVar1, puVar6, 0x2a);
                    puVar6 = puVar6 + 0x2a;
                } while (iVar2 < sStack_102);
            }
            puVar4 = puVar4 + 0x126;
        } while (puVar4 < DAT_007562dc);
        puVar7 = DAT_0075637a;
        do {
            FUN_004be2c0(pFVar1, &sStack_102);
            if (sStack_102 != 0) {
                FUN_004be320(pFVar1, puVar7, 0x1e5);
            }
            puVar7 = puVar7 + 0x1e5;
        } while (puVar7 < (unsigned char *)&DAT_0075766c);
        FUN_004be1f0(pFVar1);
    }
    return;
}
