/* SPDX-License-Identifier: MIT */
/* z_bop 004bda30 */
extern void __cdecl FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
extern char * __cdecl FUN_004ddc10(int param_1);
extern void __cdecl FUN_004db640(int param_1, char *param_2, int *param_3, unsigned int param_4);
extern int __cdecl _sprintf(char *param_1, char *param_2, ...);
extern int __cdecl FUN_004dbfa0(char *param_1, int param_2);
extern void __cdecl FUN_004db4e0(int param_1, char *param_2, int param_3, int param_4, unsigned int param_5);
extern void __cdecl FUN_004d6550(int *param_1, int param_2, int param_3, int param_4);
extern int __cdecl FUN_004da380(int *param_1, char *param_2, int param_3, int param_4, int param_5, char *param_6);
extern void __cdecl FUN_004d64a0(int *param_1, int param_2, int param_3);

extern int DAT_00b69d20;
extern char DAT_0052adec;
extern char DAT_00bb73a9;
extern char DAT_00bb74c1;
extern int DAT_00b69cc4;
extern int DAT_0052c000;

void __stdcall FUN_004bda30(void)
{
    char *pcVar1;
    int iVar2;
    char *pcVar3;
    int iVar4;
    int iVar5;
    int *piVar6;
    unsigned int uVar7;
    int iVar8;
    int local_10[4];

    FUN_004d6460(local_10, 0x54, 0x5a, 0x25c, 0x6a);
    piVar6 = local_10;
    uVar7 = 0xffff;
    pcVar1 = FUN_004ddc10(0x31a);
    iVar4 = 0;
    iVar5 = 0;
    FUN_004db640(0xf, pcVar1, piVar6, uVar7);
    do {
        iVar5 = iVar5 + 1;
        iVar2 = iVar5;
        pcVar1 = FUN_004ddc10(0x31b);
        _sprintf((char *)&DAT_00b69d20, &DAT_0052adec, pcVar1, iVar2);
        iVar2 = FUN_004dbfa0((char *)&DAT_00b69d20, 0xc);
        if (iVar4 < iVar2) {
            iVar4 = iVar2;
        }
    } while (iVar5 < 4);
    iVar5 = 0x7e;
    pcVar1 = (char *)&DAT_00bb73a9;
    FUN_004d6460(local_10, iVar4 + 0x5d, 0x7e, 600, 0x92);
    iVar4 = 0;
    do {
        iVar2 = iVar4 + 1;
        iVar8 = iVar2;
        pcVar3 = FUN_004ddc10(0x31b);
        _sprintf((char *)&DAT_00b69d20, &DAT_0052adec, pcVar3, iVar8);
        FUN_004db4e0(0xc, (char *)&DAT_00b69d20, 0x58, iVar5, 0xffff);
        FUN_004d6550(local_10, 0, 0, DAT_00b69cc4);
        iVar4 = FUN_004da380(local_10, pcVar1, 0x2e, iVar4, 0xc, (char *)0x0);
        if ((iVar4 != 0) && (DAT_0052c000 = iVar2, 3 < iVar2)) {
            DAT_0052c000 = 0;
        }
        iVar5 = iVar5 + 0x23;
        pcVar1 = pcVar1 + 0x46;
        FUN_004d64a0(local_10, 0, 0x23);
        iVar4 = iVar2;
    } while (pcVar1 < &DAT_00bb74c1);
    return;
}
