/* SPDX-License-Identifier: MIT */
/* z_xvt 004d75c0 */
extern unsigned int __cdecl FUN_004dcfd0(int *param_1, int param_2, int param_3, int param_4, int param_5);
extern char * __cdecl FUN_004dcfa0(unsigned int param_1);
extern int __cdecl _sprintf(char *dst, const char *fmt, ...);
extern unsigned int __cdecl FUN_004daac0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
extern int __cdecl FUN_004daee0(int param_1, char *param_2, int *param_3, unsigned int param_4, int param_5, int param_6);
extern unsigned int __cdecl FUN_004d9010(int *param_1, unsigned int param_2, unsigned int param_3, int param_4, int param_5, unsigned int param_6, int param_7);

extern int DAT_00758618;
extern int DAT_00757cc0;
extern int DAT_00757ccc;
extern int DAT_00757cc8;
extern char DAT_0081b740[];
extern char *DAT_00757cc4;
extern unsigned int DAT_005330fc;
extern unsigned int DAT_00859c2c;
extern const char s__s__c_s_00532aa4[];

int __stdcall FUN_004d75c0(void)
{
    int local_10[4];
    char *pcVar3;
    char *pcVar5;
    int iVar2;
    unsigned int uVar4;
    unsigned int uVar9;
    int iVar8;
    char cVar1;
    unsigned int uVar7;

    FUN_004dcfd0(local_10, 0x58, 0x135, 0x1ae, 0x144);
    if (DAT_00758618 == -1) {
        uVar9 = 0xffff;
        iVar8 = 1;
        iVar2 = 0;
        pcVar3 = FUN_004dcfa0(0xaf);
    }
    else {
        if ((DAT_00757cc0 == 0) || (DAT_00757ccc <= DAT_00757cc8)) {
            pcVar3 = FUN_004dcfa0(0xaf);
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
            pcVar5 = (char *)&DAT_0081b740;
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
            iVar2 = DAT_00757cc8 * 0x144 + DAT_00757cc0 + 0x40;
            uVar7 = 4;
            pcVar3 = FUN_004dcfa0(0xaf);
            _sprintf((char *)&DAT_0081b740, s__s__c_s_00532aa4, pcVar3, uVar7, iVar2);
        }
        uVar9 = 0xffff;
        iVar8 = 1;
        iVar2 = 0;
        pcVar3 = (char *)&DAT_0081b740;
    }
    FUN_004daac0(0xc, pcVar3, local_10, iVar2, iVar8, uVar9);
    if ((DAT_00758618 != -1) && (DAT_00757cc4 != (char *)0x0)) {
        FUN_004dcfd0(local_10, 0x58, 0x148, 0x1a4, 0x1aa);
        iVar2 = FUN_004daee0(0xc, DAT_00757cc4, local_10, 0xffff, 4, 0x400);
        uVar7 = 0x1ae;
        if (6 < iVar2 + 1U) {
            FUN_004dcfd0(local_10, 0x1a5, 0x148, 0x1ae, 0x1aa);
            DAT_005330fc = FUN_004d9010(local_10, DAT_005330fc, iVar2 + 1U, 0, 5, DAT_00859c2c, 6);
            uVar7 = 0x1a4;
        }
        FUN_004dcfd0(local_10, 0x58, 0x148, uVar7, 0x1aa);
        FUN_004daee0(0xc, DAT_00757cc4, local_10, 0xffff, 4, DAT_005330fc);
    }
    return 1;
}
