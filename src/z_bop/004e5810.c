/* SPDX-License-Identifier: MIT */
/* z_bop 004e5810 */
extern int DAT_00b69e30;
extern int DAT_00aa6114;
extern int DAT_00aa6118;
extern int DAT_00aa6120;
extern int DAT_00aa6124;
extern int DAT_00aa612c;
extern int DAT_00aa6130;
extern int DAT_00aa6134;
extern int DAT_0052c208;
extern int DAT_00bb2810;
extern char DAT_00bb73a6;
extern char DAT_00b69d20[];
extern const char s__c_s__c_d__c_s__c_d_0052c308[];

/* Packed globals: addresses not 4-aligned, accessed via memcpy */
extern unsigned char DAT_00b6a5aa[4];
extern unsigned char DAT_00bb0106[];
extern unsigned char DAT_00bb0112[];
extern unsigned char DAT_00bb011e[];
extern unsigned char DAT_00bb10a6[];
extern unsigned char DAT_00bb10b2[];
extern unsigned char DAT_00bb10be[];

void __cdecl FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
char * __cdecl FUN_004ddc10(unsigned int param_1);
unsigned int __cdecl FUN_004db7d0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
int __cdecl FUN_004dbfa0(char *param_1, int param_2);
int __cdecl _sprintf(char *param_1, const char *param_2, ...);
void __cdecl FUN_004db640(int param_1, char *param_2, int *param_3, int param_4);
int __cdecl FUN_004dbbd0(int param_1, int param_2, int *param_3, int param_4, int param_5, int param_6);
int __cdecl FUN_004d9d70(int *param_1, int param_2, int param_3, int param_4, int param_5, int param_6, int param_7);

unsigned int __stdcall FUN_004e5810(void)
{
    int bVar1;
    char *pcVar2;
    char *pcVar3;
    int iVar4;
    int iVar5;
    unsigned int uVar6;
    int *piVar7;
    unsigned int uVar8;
    int iVar9;
    int iVar10;
    unsigned int uVar11;
    unsigned int uVar12;
    unsigned int uVar13;
    int local_10[4];
    int sel;
    int tmp;
    int val;
    int next4;

    FUN_004d6460(local_10, 0x58, 0x8a, 0x1ae, 0x99);
    __builtin_memcpy(&sel, DAT_00b6a5aa, 4);
    if (sel == 2) {
        uVar6 = 0x1d7;
    }
    else if (sel == 4) {
        bVar1 = 0;
        if (DAT_00b69e30 == 2) {
            iVar4 = *(int *)(unsigned int)((unsigned int)DAT_00aa6114 + 0x140u + (unsigned int)DAT_00aa6134 * 0x148u);
            __builtin_memcpy(&iVar4, DAT_00bb0106 + (unsigned int)iVar4 * 0xa0u, 4);
joined_r0x004e58cb:
            if ((iVar4 != 0) && (DAT_00bb73a6 != '\0')) {
LAB_004e58ea:
                bVar1 = 1;
            }
        }
        else {
            if (DAT_00b69e30 == 4) {
                iVar4 = *(int *)(unsigned int)((unsigned int)DAT_00aa6114 + 0x140u + (unsigned int)DAT_00aa6134 * 0x148u);
                __builtin_memcpy(&iVar4, DAT_00bb10a6 + (unsigned int)iVar4 * 0xa0u, 4);
                goto joined_r0x004e58cb;
            }
            if ((DAT_00aa6120 != 0) && (DAT_00aa6124 != 0)) goto LAB_004e58ea;
        }
        iVar4 = 0;
        uVar6 = 0xffff;
        iVar10 = 1;
        if (bVar1) {
            piVar7 = local_10;
            iVar9 = 0;
            pcVar3 = FUN_004ddc10(0x1d8);
            FUN_004db7d0(0xf, pcVar3, piVar7, iVar9, iVar10, uVar6);
            iVar10 = 0xf;
            pcVar3 = FUN_004ddc10(0x1d8);
            iVar10 = FUN_004dbfa0(pcVar3, iVar10);
            local_10[0] = (int)((unsigned int)local_10[0] + (unsigned int)iVar10);
            if (DAT_00b69e30 == 2) {
                iVar10 = 0;
                iVar5 = 0;
                iVar9 = *(int *)(unsigned int)((unsigned int)DAT_00aa6114 + 0x140u + (unsigned int)DAT_00aa6134 * 0x148u);
                __builtin_memcpy(&tmp, DAT_00bb0112 + (unsigned int)iVar9 * 0xa0u, 4);
                if (-1 < tmp) {
                    do {
                        if (9 < iVar5) break;
                        __builtin_memcpy(&val, DAT_00bb011e + ((unsigned int)iVar5 + (unsigned int)iVar9 * 0x28u) * 4u, 4);
                        if (val == 0) {
                            iVar4 = (int)((unsigned int)iVar4 + 1u);
                        }
                        else if (val == 1) {
                            iVar10 = (int)((unsigned int)iVar10 + 1u);
                        }
                        iVar5 = (int)((unsigned int)iVar5 + 1u);
                        __builtin_memcpy(&tmp, DAT_00bb0112 + (unsigned int)iVar9 * 0xa0u, 4);
                    } while (iVar5 <= tmp);
                }
            }
            else {
                iVar10 = DAT_00aa612c;
                iVar4 = DAT_00aa6130;
                if (DAT_00b69e30 == 4) {
                    iVar5 = 0;
                    iVar4 = 0;
                    iVar10 = 0;
                    iVar9 = *(int *)(unsigned int)((unsigned int)DAT_00aa6114 + 0x140u + (unsigned int)DAT_00aa6134 * 0x148u);
                    __builtin_memcpy(&tmp, DAT_00bb10b2 + (unsigned int)iVar9 * 0xa0u, 4);
                    if (-1 < tmp) {
                        do {
                            if (9 < iVar5) break;
                            __builtin_memcpy(&val, DAT_00bb10be + ((unsigned int)iVar5 + (unsigned int)iVar9 * 0x28u) * 4u, 4);
                            if (val == 0) {
                                iVar4 = (int)((unsigned int)iVar4 + 1u);
                            }
                            else if (val == 1) {
                                iVar10 = (int)((unsigned int)iVar10 + 1u);
                            }
                            iVar5 = (int)((unsigned int)iVar5 + 1u);
                            __builtin_memcpy(&tmp, DAT_00bb10b2 + (unsigned int)iVar9 * 0xa0u, 4);
                        } while (iVar5 <= tmp);
                    }
                }
            }
            uVar11 = 1;
            pcVar3 = FUN_004ddc10(0x157);
            uVar8 = 3;
            uVar12 = 1;
            pcVar2 = FUN_004ddc10(0x156);
            _sprintf(DAT_00b69d20, s__c_s__c_d__c_s__c_d_0052c308, 5, pcVar2, uVar12, iVar4, uVar8,
                     pcVar3, uVar11, iVar10);
            FUN_004db640(0xc, DAT_00b69d20, local_10, 0xffff);
            goto LAB_004e5aa1;
        }
        uVar6 = 0x1d8;
    }
    else if (sel == 5) {
        uVar6 = 0x309;
    }
    else {
        uVar6 = 0xc0;
    }
    piVar7 = local_10;
    uVar13 = 0xffff;
    iVar10 = 1;
    iVar4 = 0;
    pcVar3 = FUN_004ddc10(uVar6);
    FUN_004db7d0(0xf, pcVar3, piVar7, iVar4, iVar10, uVar13);
LAB_004e5aa1:
    FUN_004d6460(local_10, 0x58, 0x9d, 0x1a4, 0x12d);
    iVar4 = FUN_004dbbd0(0xc, DAT_00aa6118, local_10, 0xffff, 4, 0x1000);
    uVar12 = 0x1ae;
    next4 = (int)((unsigned int)iVar4 + 1u);
    if (9 < next4) {
        FUN_004d6460(local_10, 0x1a5, 0x9d, 0x1ae, 0x12d);
        DAT_0052c208 = FUN_004d9d70(local_10, DAT_0052c208, next4, 0, 5, DAT_00bb2810, 9);
        uVar12 = 0x1a4;
    }
    FUN_004d6460(local_10, 0x58, 0x9d, uVar12, 0x12d);
    FUN_004dbbd0(0xc, DAT_00aa6118, local_10, 0xffff, 4, DAT_0052c208);
    return 1;
}
