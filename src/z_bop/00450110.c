/* SPDX-License-Identifier: MIT */
/* z_bop 00450110 */
extern int DAT_009ecc34;
extern char DAT_009e9680;
extern char DAT_009ec468;

void * FUN_0044eb60(void *, int);
void FUN_004aba50(void);
void FUN_00438df0(int);
void FUN_004abe50(void);
void * FUN_004ac260(void);
void FUN_0044f140(void);
void FUN_004abee0(void);

void __fastcall FUN_00450110(void *param_1)
{
    int iVar1;
    int iVar2;
    char *pcVar3;
    void *this_ptr;

    iVar2 = 0;
    pcVar3 = &DAT_009e9680;
    iVar1 = DAT_009ecc34;
    do {
        param_1 = (void *)(((unsigned int)param_1 & 0xFFFFFF00) | (unsigned char)*pcVar3);
        if ((*pcVar3 != '\0') && (iVar2 != iVar1)) {
            param_1 = FUN_0044eb60(param_1, iVar2);
            iVar1 = DAT_009ecc34;
        }
        pcVar3 += 0x5bd;
        iVar2 += 1;
    } while (pcVar3 < &DAT_009ec468);
    FUN_0044eb60(param_1, iVar1);
    FUN_004aba50();
    iVar2 = 0;
    pcVar3 = &DAT_009e9680;
    FUN_00438df0(DAT_009ecc34);
    FUN_004abe50();
    this_ptr = FUN_004ac260();
    iVar1 = DAT_009ecc34;
    do {
        this_ptr = (void *)(((unsigned int)this_ptr & 0xFFFFFF00) | (unsigned char)*pcVar3);
        if ((*pcVar3 != '\0') && (iVar2 != iVar1)) {
            this_ptr = FUN_0044eb60(this_ptr, iVar2);
            iVar1 = DAT_009ecc34;
        }
        pcVar3 += 0x5bd;
        iVar2 += 1;
    } while (pcVar3 < &DAT_009ec468);
    FUN_0044eb60(this_ptr, iVar1);
    FUN_0044f140();
    FUN_004abee0();
    FUN_004ac260();
    return;
}
