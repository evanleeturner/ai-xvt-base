/* SPDX-License-Identifier: MIT */
/* z_bop 004f70f0 */
extern int DAT_00b6a5aa;
extern int DAT_00b6a5ae;
extern unsigned int DAT_00aa6114;
extern unsigned int DAT_00aa6134;
extern unsigned int DAT_00aa6138;
extern char *PTR_s_train_0052c1e8[];
extern char s__s__s_0052c300[];
extern char DAT_0051a854[];
extern unsigned int DAT_00a91cc0[];
extern short DAT_00aa5ab2;
extern short DAT_00aa5aac;
extern short DAT_00aa5aae;
extern unsigned char DAT_00aa471c[];
extern unsigned char DAT_00aa1f20[];
extern unsigned char DAT_00aa3ba0[];
extern unsigned char DAT_00aa47ba[];

void FUN_004e5000(int);
int _sprintf(char *, const char *, ...);
void *FUN_004cc4a0(char *, char *);
void FUN_004cc660(void *, void *);
void FUN_004cc590(void *);
void FUN_004cc6c0(void *, void *, int);

void __stdcall FUN_004f70f0(void)
{
    void *pFVar1;
    int iVar2;
    int *piVar3;
    unsigned char *puVar4;
    unsigned int *puVar5;
    unsigned char *puVar6;
    unsigned char *puVar7;
    short local_102;
    char local_100[256];
    int tmpA;
    int tmpB;

    __builtin_memcpy(&tmpA, &DAT_00b6a5aa, 4);
    FUN_004e5000(tmpA);
    if ((DAT_00aa6114 != 0) && (DAT_00aa6134 = 0, DAT_00aa6138 != 0)) {
        piVar3 = (int *)(DAT_00aa6114 + 0x140);
        do {
            __builtin_memcpy(&tmpA, &DAT_00b6a5aa, 4);
            __builtin_memcpy(&tmpB, (unsigned char *)&DAT_00b6a5aa + 4 + tmpA * 4, 4);
            if (*piVar3 == tmpB) break;
            piVar3 = piVar3 + 0x52;
            DAT_00aa6134 = DAT_00aa6134 + 1;
        } while (DAT_00aa6134 < DAT_00aa6138);
    }
    __builtin_memcpy(&tmpA, &DAT_00b6a5aa, 4);
    _sprintf(local_100, s__s__s_0052c300, PTR_s_train_0052c1e8[tmpA],
             DAT_00aa6134 * 0x148u + DAT_00aa6114);
    pFVar1 = FUN_004cc4a0(local_100, DAT_0051a854);
    if (pFVar1 != (void *)0) {
        puVar5 = DAT_00a91cc0;
        for (iVar2 = 0x4f7d; iVar2 != 0; iVar2 = iVar2 + -1) {
            *puVar5 = 0;
            puVar5 = puVar5 + 1;
        }
        FUN_004cc660(pFVar1, &DAT_00aa5ab2);
        if (((DAT_00aa5ab2 != 0xe) && (DAT_00aa5ab2 != 0xd)) && (DAT_00aa5ab2 != 0xc)) {
            FUN_004cc590(pFVar1);
            return;
        }
        FUN_004cc660(pFVar1, &DAT_00aa5aac);
        FUN_004cc660(pFVar1, &DAT_00aa5aae);
        iVar2 = 0;
        FUN_004cc6c0(pFVar1, DAT_00aa471c, 0x9e);
        if (0 < DAT_00aa5aac) {
            puVar5 = DAT_00a91cc0;
            do {
                iVar2 = iVar2 + 1;
                FUN_004cc6c0(pFVar1, puVar5, 0x562);
                puVar5 = (unsigned int *)((int)puVar5 + 0x562);
            } while (iVar2 < DAT_00aa5aac);
        }
        iVar2 = 0;
        if (0 < DAT_00aa5aae) {
            do {
                iVar2 = iVar2 + 1;
                FUN_004cc660(pFVar1, &local_102);
                FUN_004cc6c0(pFVar1, DAT_00aa1f20 + local_102 * 0x72, 0x72);
            } while (iVar2 < DAT_00aa5aae);
        }
        puVar4 = DAT_00aa3ba0;
        do {
            iVar2 = 0;
            FUN_004cc660(pFVar1, &local_102);
            puVar6 = puVar4;
            if (0 < local_102) {
                do {
                    iVar2 = iVar2 + 1;
                    FUN_004cc6c0(pFVar1, puVar6, 0x2a);
                    puVar6 = puVar6 + 0x2a;
                } while (iVar2 < local_102);
            }
            puVar4 = puVar4 + 0x126;
        } while (puVar4 < DAT_00aa471c);
        puVar7 = DAT_00aa47ba;
        do {
            FUN_004cc660(pFVar1, &local_102);
            if (local_102 != 0) {
                FUN_004cc6c0(pFVar1, puVar7, 0x1e5);
            }
            puVar7 = puVar7 + 0x1e5;
        } while (puVar7 < (unsigned char *)&DAT_00aa5aac);
        FUN_004cc590(pFVar1);
    }
    return;
}
