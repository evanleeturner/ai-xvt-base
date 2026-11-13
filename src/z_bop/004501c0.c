/* SPDX-License-Identifier: MIT */
/* z_bop 004501c0 */
extern int DAT_009ecc34;
extern int DAT_00527e90;

void * __fastcall FUN_0044eb60(void *param_1, int param_2);
void FUN_0044f140(void);
void FUN_00411810(int param_1);
void FUN_00411540(void);
void FUN_004aba50(void);
void FUN_00438df0(int param_1);
void FUN_004abe50(void);
void FUN_004abee0(void);
void FUN_004ac250(void);
void FUN_0040b750(void);
void FUN_004ac260(void);

void __fastcall FUN_004501c0(void *param_1)
{
    int iVar1;
    int iVar2;
    char *pcVar3;

    iVar2 = 0;
    pcVar3 = (char *)0x009e9680;
    iVar1 = DAT_009ecc34;
    do {
        param_1 = (void *)(((unsigned int)param_1 & 0xFFFFFF00) | (unsigned char)*pcVar3);
        if (*pcVar3 != '\0' && iVar2 != iVar1) {
            param_1 = FUN_0044eb60(param_1, iVar2);
            iVar1 = DAT_009ecc34;
        }
        pcVar3 += 0x5bd;
        iVar2 += 1;
    } while (pcVar3 < (char *)0x009ec468);
    FUN_0044eb60(param_1, iVar1);
    FUN_0044f140();
    FUN_00411810(DAT_009ecc34);
    FUN_00411540();
    FUN_004aba50();
    FUN_00438df0(DAT_009ecc34);
    FUN_004abe50();
    FUN_004abee0();
    FUN_004ac250();
    if (DAT_00527e90 != 0) {
        FUN_0040b750();
        return;
    }
    FUN_004ac260();
    return;
}
