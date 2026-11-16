/* SPDX-License-Identifier: MIT */
/* z_bop 005066a0 */
/* Prototypes for callees (signatures inferred from the decompile; unresolved types noted) */
extern void FUN_004ba3c0(void);
extern void FUN_004bf650(int a);
extern void FUN_004de100(void);
extern int  FUN_004cc9f0(int a);
extern void FUN_004d2e50(void);
extern char *FUN_004ddc10(int a);
extern int  FUN_004dcb90(char *a, char *b, char *c, char *d, char *e);
extern void FUN_004de110(void);
extern void FUN_004c9ee0(void);
extern void FUN_004cef70(int a, unsigned int *b, int c);
extern void FUN_004cd9d0(void);
extern void FUN_004dc330(void *a, void *b);   /* args are a function and a code label; real types unresolved */
extern int  FUN_004d32a0(void);
extern void FUN_004d3520(unsigned int a, unsigned int b, int c);
extern void FUN_004d5bc0(void);
extern void FUN_004d56b0(void);
extern void FUN_004d5b70(int a);
extern void FUN_004e5000(int a);
extern int  __cdecl _sprintf(char *buf, const char *fmt, ...);
extern void __cdecl _free(void *p);
extern int  FUN_004a9c00(char *a);
extern void FUN_004d5760(void);
extern void FUN_004df700(const char *a);
extern void FUN_004d3330(void);
extern void FUN_004d3020(int a, int b, int c);
extern void FUN_004d34a0(unsigned int a);

/* Callback targets (signatures unresolved) */
extern void FUN_004be1b0(void);
extern void LAB_004be050(void);
extern void FUN_004fe160(void);
extern void LAB_004fe100(void);

/* Globals */
extern int           DAT_00bb2818;
extern int           DAT_00b6a250;
extern int           DAT_00b69e30;
extern int           DAT_00b69cb0;
extern char          DAT_00bb737f;
extern unsigned char DAT_00bb737e;
extern unsigned int  DAT_00aa6af0;
extern int           DAT_00b6a5aa;   /* packed (unaligned) */
extern int           DAT_00b6a5ae;   /* packed (unaligned) array of int */
extern int           DAT_00b6bc32;   /* packed (unaligned) */
extern void         *DAT_00aa6114;
extern unsigned int  DAT_00aa6138;
extern unsigned int  DAT_00aa6144;
extern char         *PTR_s_train_0052c1e8[];
extern char          DAT_00b6d58a;
extern int           DAT_00b6a24c;
extern char          DAT_00b6a2e0;
extern char          DAT_00b6a5c6;
extern unsigned int  DAT_00b6a2c8;
extern char          DAT_0066da78;
extern char          DAT_00b6bc9e;
extern char          s___s__s____s____s___u___s__0__u_p_0052d598[];
extern char          s___s__s____s____s___u___s__0__u_n_0052d5cc[];
extern char          s_sfx_sfx_lst_0052b12c[];

unsigned int __cdecl FUN_005066a0(int param_1)
{
    char cVar1;
    int iVar2;
    char *pcVar3;
    char *pcVar4;
    char *pcVar5;
    char *pcVar6;
    char *pcVar7;
    unsigned int uVar8;
    unsigned int uVar9;
    int *piVar10;
    int iVar11;
    char *puVar12;
    int iTrain;
    int iTmp;

    iVar11 = 0;
    if (param_1 == 0) {
        FUN_004ba3c0();
        FUN_004bf650(0);
        while (1) {
            FUN_004de100();
            iVar2 = FUN_004cc9f0(DAT_00bb2818);
            if (iVar2 != 0) break;
            FUN_004d2e50();
            pcVar3 = FUN_004ddc10(0x13);
            pcVar4 = FUN_004ddc10(0x20b);
            pcVar5 = FUN_004ddc10(0x2c4);
            pcVar6 = FUN_004ddc10(0x2c3);
            pcVar7 = FUN_004ddc10(0x2c2);
            iVar11 = FUN_004dcb90(pcVar7, pcVar6, pcVar5, pcVar4, pcVar3);
            if (iVar11 == 0) {
                FUN_004de100();
                return 1;
            }
        }
        FUN_004de110();
        FUN_004c9ee0();
        if ((DAT_00b6a250 == 0) && (DAT_00b69e30 != 3)) {
            FUN_004de100();
            pcVar7 = (char *)0;
            pcVar6 = (char *)0;
            pcVar3 = FUN_004ddc10(0x2ef);
            pcVar4 = FUN_004ddc10(0x2ee);
            pcVar5 = FUN_004ddc10(0x2ed);
            FUN_004dcb90(pcVar5, pcVar4, pcVar3, pcVar6, pcVar7);
            if (DAT_00b69e30 != 2) {
                DAT_00aa6af0 = 0x46;
                FUN_004cef70(0, &DAT_00aa6af0, 4);
                FUN_004cd9d0();
            }
            DAT_00b69e30 = 0;
            FUN_004dc330((void *)FUN_004be1b0, (void *)&LAB_004be050);
            return 0;
        }
        if (DAT_00bb737f != '\0') {
            iVar2 = FUN_004d32a0();
            if (iVar2 == 0) {
                uVar8 = ((unsigned int)DAT_00bb737e * 0xffffu) / 9u;
                FUN_004d3520(uVar8, uVar8 >> 3, 1000);
            }
        }
        FUN_004d5bc0();
        FUN_004d56b0();
        FUN_004d5b70(1);
        __builtin_memcpy(&iTrain, &DAT_00b6a5aa, 4);
        FUN_004e5000(iTrain);
        if (DAT_00aa6114 != (void *)0) {
            uVar8 = 0;
            __builtin_memcpy(&iTrain, &DAT_00b6a5aa, 4);
            if (DAT_00aa6138 != 0) {
                piVar10 = (int *)((int)DAT_00aa6114 + 0x140);
                do {
                    __builtin_memcpy(&iTmp, (char *)&DAT_00b6a5ae + (unsigned int)iTrain * 4u, 4);
                    if (*piVar10 == iTmp) break;
                    piVar10 = piVar10 + 0x52;
                    uVar8 = uVar8 + 1;
                } while (uVar8 < DAT_00aa6138);
            }
            if (DAT_00b69cb0 == 0) {
                puVar12 = PTR_s_train_0052c1e8[iTrain];
                pcVar3 = s___s__s____s____s___u___s__0__u_p_0052d598;
            } else {
                puVar12 = PTR_s_train_0052c1e8[iTrain];
                pcVar3 = s___s__s____s____s___u___s__0__u_n_0052d5cc;
            }
            _sprintf(&DAT_0066da78, pcVar3, puVar12,
                     (void *)(uVar8 * 0x148u + (unsigned int)DAT_00aa6114),
                     &DAT_00b6d58a + (unsigned int)DAT_00b6a24c * 0x16u, &DAT_00b6a2e0,
                     DAT_00aa6144, &DAT_00b6a5c6, DAT_00b6a2c8);
            _free(DAT_00aa6114);
            DAT_00aa6114 = (void *)0;
            iVar11 = FUN_004a9c00(&DAT_0066da78);
        }
        FUN_004d5760();
        FUN_004d5b70(0);
        FUN_004df700(s_sfx_sfx_lst_0052b12c);
        FUN_004ba3c0();
        __builtin_memcpy(&iTmp, &DAT_00b6bc32, 4);
        pcVar3 = FUN_004ddc10((int)((unsigned int)iTmp + 0x7au));
        uVar8 = 0xffffffffu;
        do {
            pcVar4 = pcVar3;
            if (uVar8 == 0) break;
            uVar8 = uVar8 - 1;
            pcVar4 = pcVar3 + 1;
            cVar1 = *pcVar3;
            pcVar3 = pcVar4;
        } while (cVar1 != '\0');
        uVar8 = ~uVar8;
        pcVar3 = pcVar4 - uVar8;
        pcVar4 = &DAT_00b6bc9e;
        for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
            __builtin_memcpy(pcVar4, pcVar3, 4);
            pcVar3 = pcVar3 + 4;
            pcVar4 = pcVar4 + 4;
        }
        for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
            *pcVar4 = *pcVar3;
            pcVar3 = pcVar3 + 1;
            pcVar4 = pcVar4 + 1;
        }
        FUN_004bf650(0);
        FUN_004d2e50();
        FUN_004d3330();
        if (DAT_00bb737f != '\0') {
            FUN_004d3020(7, 0, 0);
            FUN_004d34a0(((unsigned int)DAT_00bb737e * 0xffffu) / 9u);
        }
    }
    if (iVar11 == 0) {
        FUN_004de100();
        FUN_004cd9d0();
        FUN_004dc330((void *)FUN_004be1b0, (void *)&LAB_004be050);
        return 0;
    }
    FUN_004dc330((void *)FUN_004fe160, (void *)&LAB_004fe100);
    return 0;
}
