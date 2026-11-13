/* SPDX-License-Identifier: MIT */
/* z_bop 004d8350 */
extern int DAT_00aa6ae8;
extern int DAT_00aa6114;
extern int DAT_00aa6138;
extern int DAT_00aa6134;
extern char DAT_00b69d20[];
extern char *DAT_00aa6118;
extern int DAT_0052c208;
extern int DAT_00bb2810;
extern char s__s__c_s_0052bfcc[];

extern void __cdecl FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
extern char * __cdecl FUN_004ddc10(unsigned int param_1);
extern int __cdecl _sprintf(char *dst, const char *fmt, ...);
extern void __cdecl FUN_004db7d0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
extern int __cdecl FUN_004dbbd0(int param_1, char *param_2, int *param_3, unsigned int param_4, int param_5, int param_6);
extern int __cdecl FUN_004d9d70(int *param_1, int param_2, unsigned int param_3, int param_4, int param_5, int param_6, int param_7);

int __stdcall FUN_004d8350(void)
{
    int local_10[4];
    char *pcVar3;
    int *piVar6;
    int iVar2;
    int iVar8;
    unsigned int uVar9;
    unsigned int uVar4;
    char *pcVar5;
    char cVar1;
    unsigned int uVar7;

    FUN_004d6460(local_10, 0x58, 0x135, 0x1ae, 0x144);
    if (DAT_00aa6ae8 == -1) {
        piVar6 = local_10;
        uVar9 = 0xffff;
        iVar8 = 1;
        iVar2 = 0;
        pcVar3 = FUN_004ddc10(0xbb);
    }
    else {
        if (DAT_00aa6114 == 0 || DAT_00aa6138 <= DAT_00aa6134) {
            pcVar3 = FUN_004ddc10(0xbb);
            uVar9 = 0xffffffff;
            do {
                pcVar5 = pcVar3;
                if (uVar9 == 0) break;
                uVar9 = uVar9 - 1;
                pcVar5 = pcVar3 + 1;
                cVar1 = *pcVar3;
                pcVar3 = pcVar5;
            } while (cVar1 != '\0');
            uVar9 = ~uVar9;
            pcVar3 = pcVar5 + -uVar9;
            pcVar5 = DAT_00b69d20;
            for (uVar4 = uVar9 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
                *(unsigned int *)pcVar5 = *(unsigned int *)pcVar3;
                pcVar3 = pcVar3 + 4;
                pcVar5 = pcVar5 + 4;
            }
            for (uVar9 = uVar9 & 3; uVar9 != 0; uVar9 = uVar9 - 1) {
                *pcVar5 = *pcVar3;
                pcVar3 = pcVar3 + 1;
                pcVar5 = pcVar5 + 1;
            }
        }
        else {
            iVar2 = DAT_00aa6114 + DAT_00aa6134 * 0x148 + 0x40;
            uVar7 = 4;
            pcVar3 = FUN_004ddc10(0xbb);
            _sprintf(DAT_00b69d20, s__s__c_s_0052bfcc, pcVar3, uVar7, iVar2);
        }
        piVar6 = local_10;
        uVar9 = 0xffff;
        iVar8 = 1;
        iVar2 = 0;
        pcVar3 = DAT_00b69d20;
    }
    FUN_004db7d0(0xc, pcVar3, piVar6, iVar2, iVar8, uVar9);
    if (DAT_00aa6ae8 != -1 && DAT_00aa6118 != (char *)0) {
        FUN_004d6460(local_10, 0x58, 0x148, 0x1a4, 0x1aa);
        iVar2 = FUN_004dbbd0(0xc, DAT_00aa6118, local_10, 0xffff, 4, 0x1000);
        uVar7 = 0x1ae;
        if (6 < iVar2 + 1U) {
            FUN_004d6460(local_10, 0x1a5, 0x148, 0x1ae, 0x1aa);
            DAT_0052c208 = FUN_004d9d70(local_10, DAT_0052c208, iVar2 + 1U, 0, 5, DAT_00bb2810, 6);
            uVar7 = 0x1a4;
        }
        FUN_004d6460(local_10, 0x58, 0x148, uVar7, 0x1aa);
        FUN_004dbbd0(0xc, DAT_00aa6118, local_10, 0xffff, 4, DAT_0052c208);
    }
    return 1;
}
