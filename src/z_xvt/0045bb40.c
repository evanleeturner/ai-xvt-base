/* SPDX-License-Identifier: MIT */
/* z_xvt 0045bb40 */
extern char DAT_0071b740;
extern char DAT_0071e528;
extern int DAT_0071ecf4;
extern int DAT_0052ae18;

void * __fastcall FUN_0045a560(void *param_1, int param_2);
void __fastcall FUN_0045aa10(void);
void __fastcall FUN_00405cf0(int param_1);
void __fastcall FUN_004054e0(void);
void __fastcall FUN_004a6a80(void);
void __fastcall FUN_004a80d0(int param_1);
void __fastcall FUN_004a6f90(void);
void __fastcall FUN_004a7040(void);
void __fastcall FUN_004a7120(void);
void __fastcall FUN_0045f7a0(void);
void __fastcall FUN_004a7130(void);

void __fastcall FUN_0045bb40(void *param_1)
{
    int iVar1;
    char *pcVar2;
    int iVar3;

    pcVar2 = &DAT_0071b740;
    iVar3 = 0;
    iVar1 = DAT_0071ecf4;
    do {
        param_1 = (void *)(((unsigned int)param_1 & 0xFFFFFF00) | (unsigned char)*pcVar2);
        if ((*pcVar2 != '\0') && (iVar1 != iVar3)) {
            param_1 = FUN_0045a560(param_1, iVar3);
            iVar1 = DAT_0071ecf4;
        }
        pcVar2 += 0x5bd;
        iVar3++;
    } while (pcVar2 < &DAT_0071e528);
    FUN_0045a560(param_1, iVar1);
    FUN_0045aa10();
    FUN_00405cf0(DAT_0071ecf4);
    FUN_004054e0();
    FUN_004a6a80();
    FUN_004a80d0(DAT_0071ecf4);
    FUN_004a6f90();
    FUN_004a7040();
    FUN_004a7120();
    if (DAT_0052ae18 != 0) {
        FUN_0045f7a0();
        return;
    }
    FUN_004a7130();
    return;
}
