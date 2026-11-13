/* SPDX-License-Identifier: MIT */
/* z_bop 004cc930 */
extern char DAT_00b69aa6;
extern char DAT_0052b6f4;
extern char DAT_0051a854;
extern char s_b_movies_imp1snd_smk_0052b6dc[];

extern void __stdcall FUN_004d3350(void);
extern void __stdcall FUN_004d3400(void);
extern void * __stdcall FID_conflict___wfopen(char *, char *);
extern void __stdcall _fclose(void *);

int __stdcall FUN_004cc930(void)
{
    void *pFVar1;
    int iVar2;
    char *pcVar3;
    char *pcVar4;
    char local_50[80];

    if (DAT_00b69aa6 == '\0') {
        return 0;
    }
    FUN_004d3350();
    pcVar3 = &DAT_0052b6f4;
    pcVar4 = local_50;
    for (iVar2 = 6; iVar2 != 0; iVar2 = iVar2 - 1) {
        *(int *)pcVar4 = *(int *)pcVar3;
        pcVar3 += 4;
        pcVar4 += 4;
    }
    *pcVar4 = *pcVar3;
    local_50[0] = DAT_00b69aa6;
    pFVar1 = FID_conflict___wfopen(local_50, &DAT_0051a854);
    if (pFVar1 == (void *)0x0) {
        FUN_004d3400();
        return 0;
    }
    _fclose(pFVar1);
    pcVar3 = s_b_movies_imp1snd_smk_0052b6dc;
    pcVar4 = local_50;
    for (iVar2 = 5; iVar2 != 0; iVar2 = iVar2 - 1) {
        *(int *)pcVar4 = *(int *)pcVar3;
        pcVar3 += 4;
        pcVar4 += 4;
    }
    *(short *)pcVar4 = *(short *)pcVar3;
    local_50[0] = DAT_00b69aa6;
    pFVar1 = FID_conflict___wfopen(local_50, &DAT_0051a854);
    if (pFVar1 == (void *)0x0) {
        FUN_004d3400();
        return 0;
    }
    FUN_004d3400();
    _fclose(pFVar1);
    return 1;
}
