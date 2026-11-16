/* SPDX-License-Identifier: MIT */
/* z_bop 004cf980 */
extern int *DAT_00acebc5;               /* 0x00acebc5, unaligned */
extern int DAT_00acf145;                /* 0x00acf145, unaligned */
extern unsigned int DAT_00b53571;       /* 0x00b53571, unaligned */
extern unsigned int DAT_00b53579;       /* 0x00b53579, unaligned */
extern unsigned char DAT_00acf571[];    /* queue entry +0x00 (uint) */
extern unsigned char DAT_00acf575[];    /* queue entry +0x04 (uint) */
extern unsigned char DAT_00acf579[];    /* queue entry +0x08 (uint) */
extern unsigned char DAT_00acf57d[];    /* queue entry +0x0c */
extern unsigned char DAT_00acf57e[];    /* queue entry +0x0d */
extern unsigned char DAT_00acf57f[];    /* queue entry +0x0e */
extern unsigned char DAT_00acf580[];    /* queue entry +0x0f */
extern unsigned char DAT_00acf581[];    /* queue entry +0x10, data */

extern char s__RSB__u__0052b984[];
extern char s__RSG__u__0052b978[];
extern char s__RSS__u__0052b96c[];

int __cdecl _sprintf(char *_Dest, const char *_Format, ...);
/* Ghidra shows no parameters, but the original pushes 4 bytes and the
   caller cleans them (stack-check finding at 004cfa63). */
int __cdecl FUN_0046fad0(unsigned int param_1);

unsigned char __cdecl FUN_004cf980(int param_1, int param_2, unsigned int param_3,
                                   unsigned int *param_4, unsigned int param_5)
{
    unsigned char bVar1;
    int iVar2;
    unsigned int uVar3;
    unsigned char *puVar4;
    int iVar5;
    unsigned char *puVar6;
    unsigned char *puVar7;
    char *_Format;
    int local_504;
    char local_500[256];
    unsigned char local_400[0x400];
    int *pGlobThis;
    int iGlobCur;
    unsigned int uGlobA;
    unsigned int uGlobB;
    unsigned short usTmp;

    local_504 = 0;
    __builtin_memcpy(&pGlobThis, &DAT_00acebc5, sizeof(pGlobThis));
    if (pGlobThis != (int *)0x0) {
        uVar3 = *param_4;
        if ((uVar3 < 0x3c) || (0x3f < uVar3)) {
            bVar1 = 1;
        } else {
            bVar1 = 0;
        }
        if (param_2 == 0) {
            _Format = s__RSB__u__0052b984;
        } else if (param_2 == 2) {
            _Format = s__RSG__u__0052b978;
        } else {
            _Format = s__RSS__u__0052b96c;
        }
        _sprintf(local_500, _Format, param_3);
        puVar4 = local_400 + 3;
        iVar5 = 3;
        usTmp = (unsigned short)((unsigned short)((unsigned char)uVar3 & 0x7f) |
                                 (unsigned short)(((unsigned short)param_3 & 0x7f) << 8) | 0x80);
        __builtin_memcpy(local_400, &usTmp, 2);
        local_400[2] = (unsigned char)param_2;
        if (bVar1 && (iVar2 = FUN_0046fad0(uVar3), iVar2 == 0)) {
            usTmp = (unsigned short)((unsigned short)param_5 - 4);
            __builtin_memcpy(local_400 + 3, &usTmp, 2);
            puVar4 = local_400 + 5;
            iVar5 = 5;
        }
        puVar7 = puVar4;
        puVar6 = (unsigned char *)(param_4 + 1);
        for (uVar3 = (param_5 - 4) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
            __builtin_memcpy(puVar7, puVar6, 4);
            puVar6 = puVar6 + 4;
            puVar7 = puVar7 + 4;
        }
        iVar5 = (int)((param_5 - 4) + (unsigned int)iVar5);
        for (uVar3 = (param_5 - 4) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
            *puVar7 = *puVar6;
            puVar6 = puVar6 + 1;
            puVar7 = puVar7 + 1;
        }
        if (bVar1) {
            iVar5 = iVar5 + 1;
            puVar4[param_5 - 4] = 0x39;
        }
        __builtin_memcpy(&iGlobCur, &DAT_00acf145, sizeof(iGlobCur));
        if (iGlobCur == param_1) {
            __builtin_memcpy(&uGlobA, &DAT_00b53579, sizeof(uGlobA));
            if (uGlobA < 0x400) {
                puVar6 = (unsigned char *)param_4;
                __builtin_memcpy(&uGlobA, &DAT_00b53571, sizeof(uGlobA));
                puVar4 = &DAT_00acf581[uGlobA * 0x210];
                for (uVar3 = param_5 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
                    __builtin_memcpy(puVar4, puVar6, 4);
                    puVar6 = puVar6 + 4;
                    puVar4 = puVar4 + 4;
                }
                for (uVar3 = param_5 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
                    *puVar4 = *puVar6;
                    puVar6 = puVar6 + 1;
                    puVar4 = puVar4 + 1;
                }
                __builtin_memcpy(&uGlobA, &DAT_00b53571, sizeof(uGlobA));
                __builtin_memcpy(&iGlobCur, &DAT_00acf145, sizeof(iGlobCur));
                __builtin_memcpy(&DAT_00acf571[uGlobA * 0x210], &iGlobCur, 4);
                __builtin_memcpy(&DAT_00acf575[uGlobA * 0x210], &param_5, 4);
                DAT_00acf57e[uGlobA * 0x210] = (unsigned char)param_2;
                DAT_00acf57f[uGlobA * 0x210] = (unsigned char)param_3;
                DAT_00acf580[uGlobA * 0x210] = 1;
                uGlobB = 0;
                __builtin_memcpy(&DAT_00acf579[uGlobA * 0x210], &uGlobB, 4);
                DAT_00acf57d[uGlobA * 0x210] = 0;
                __builtin_memcpy(&uGlobB, &DAT_00b53579, sizeof(uGlobB));
                uGlobB = uGlobB + 1;
                __builtin_memcpy(&DAT_00b53579, &uGlobB, sizeof(uGlobB));
                __builtin_memcpy(&uGlobA, &DAT_00b53571, sizeof(uGlobA));
                uGlobA = uGlobA + 1;
                __builtin_memcpy(&DAT_00b53571, &uGlobA, sizeof(uGlobA));
                if (0x3ff < uGlobA) {
                    uGlobB = 0;
                    __builtin_memcpy(&DAT_00b53571, &uGlobB, sizeof(uGlobB));
                }
            }
        } else {
            __builtin_memcpy(&pGlobThis, &DAT_00acebc5, sizeof(pGlobThis));
            __builtin_memcpy(&iGlobCur, &DAT_00acf145, sizeof(iGlobCur));
            local_504 = ((int (__stdcall *)(int *, int, int, int, void *, int))
                         *(void **)((unsigned char *)*pGlobThis + 0x68))
                        (pGlobThis, iGlobCur, param_1, 0, local_400, iVar5);
        }
        return local_504 == 0;
    }
    return 1;
}
