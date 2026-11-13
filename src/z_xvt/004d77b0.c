/* SPDX-License-Identifier: MIT */
/* z_xvt 004d77b0 */
extern int DAT_00758618;
extern int DAT_00757cee;
extern int DAT_00757e3e;
extern int DAT_0081b740;
extern const char s__c_s__c_s_00531334[];

void __cdecl FUN_004dcfd0(int *param_1, int param_2, int param_3, int param_4, int param_5);
char * __cdecl FUN_004dcfa0(unsigned int param_1);
void __cdecl FUN_004daac0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
void __cdecl FUN_004cde00(void);
void __cdecl FUN_004d5180(int *param_1);
void __cdecl FUN_004d51b0(int *param_1);
int __cdecl _sprintf(char *dst, const char *fmt, ...);
void __cdecl FUN_004dd030(int *param_1, int param_2, int param_3);

int __stdcall FUN_004d77b0(void)
{
    int local_20[4];
    int local_10[4];
    int *piVar2;
    int iVar3;
    int iVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    char *pcVar1;

    FUN_004dcfd0(local_20, 0x58, 0xda, 0x1ae, 0xe9);
    piVar2 = local_20;
    uVar6 = 0xffff;
    iVar4 = 1;
    iVar3 = 0;
    pcVar1 = FUN_004dcfa0(0xbc);
    FUN_004daac0(0xc, pcVar1, piVar2, iVar3, iVar4, uVar6);
    if (DAT_00758618 != -1) {
        iVar3 = 0;
        FUN_004cde00();
        piVar2 = &DAT_00757cee;
        FUN_004dcfd0(local_20, 0x58, 0xee, 0x102, 0xfc);
        do {
            if (*piVar2 != 0) {
                iVar3 = iVar3 + 1;
                FUN_004d5180(local_10);
                FUN_004d51b0(local_20);
                iVar4 = (int)piVar2 - 0xe;
                uVar5 = 4;
                pcVar1 = FUN_004dcfa0(piVar2[1] + 0x8e);
                _sprintf((char *)&DAT_0081b740, s__c_s__c_s_00531334, 6, pcVar1, uVar5, iVar4);
                FUN_004daac0(0xc, (char *)&DAT_0081b740, local_20, 0, 1, 0xffff);
                FUN_004d51b0(local_10);
                if (iVar3 == 4) {
                    FUN_004dcfd0(local_20, 0x103, 0xee, 0x1ae, 0xfc);
                } else {
                    FUN_004dd030(local_20, 0, 0xf);
                }
            }
            piVar2 = (int *)((int)piVar2 + 0x2a);
        } while (piVar2 < &DAT_00757e3e);
    }
    return 1;
}
