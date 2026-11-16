/* SPDX-License-Identifier: MIT */
/* z_bop 004da380 */
extern int DAT_0052c018;
extern int DAT_0052c014;
extern int DAT_00665464;
extern int DAT_00665680;
extern int DAT_0052c000;
extern unsigned char *PTR_DAT_0052da78;
extern int DAT_0052dc84;

int FUN_004d6e30(int a, int b, int c);
void FUN_004de090(int *a, int *b);
int FUN_004d6b90(int *a, int b, int c);
unsigned char FUN_004dc220(void);
unsigned char FUN_004dc240(void);
unsigned int FUN_004dcb10(void);
void FUN_004dcb50(void);
int __isctype(int c, int mask);
int FUN_004dbfa0(char *s, int a);
void FUN_004d4cf0(int *r);
void FUN_004d6460(int *r, int a, int b, int c, int d);
void FUN_004d4d20(int *r);
void FUN_004db4e0(int a, char *s, int b, int c, int d);
int FUN_004d4de0(void);
void FUN_004d6480(int *r, int *src);
void FUN_004d67e0(int *r, int a, int b, short c, int d);

int FUN_004da380(int *param_1, char *param_2, int param_3, int param_4, int param_5, char *param_6)
{
    char cVar1;
    int iVar3;
    int iVar4;
    unsigned int uVar5;
    unsigned char bVar6;
    char *pcVar7;
    int local_2c;
    int local_28;
    int local_24[4];
    int local_14;
    int local_10[4];

    if (DAT_0052c018 == 0) {
        DAT_0052c014 = FUN_004d6e30(0xff, 0xff, 0xff);
        DAT_0052c018 = 1;
    }
    uVar5 = 0xffffffffU;
    pcVar7 = param_2;
    do {
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
    } while (cVar1 != '\0');
    DAT_00665464 = (int)(~uVar5 - 1);
    uVar5 = 0xffffffffU;
    pcVar7 = param_2;
    do {
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
    } while (cVar1 != '\0');
    DAT_00665680 = (int)(~uVar5 - 1);
    local_28 = 0;
    FUN_004de090(&local_2c, &local_14);
    iVar3 = FUN_004d6b90(param_1, local_2c, local_14);
    if ((iVar3 != 0) &&
        (FUN_004dc220() != 0 || FUN_004dc240() != 0)) {
        DAT_0052c000 = param_4;
    }
    if (DAT_0052c000 == param_4) {
        uVar5 = FUN_004dcb10();
        bVar6 = (unsigned char)uVar5;
        if (param_6 != (char *)0) {
            iVar3 = 0;
            cVar1 = *param_6;
            while (cVar1 != '\0') {
                if (param_6[iVar3] == bVar6) {
                    bVar6 = 0;
                    FUN_004dcb50();
                    break;
                }
                iVar3 = iVar3 + 1;
                cVar1 = param_6[iVar3];
            }
        }
        if (bVar6 != 0) {
            uVar5 = (unsigned int)bVar6;
            if (uVar5 != 0x1b) {
                FUN_004dcb50();
            }
            if (DAT_0052dc84 < 2) {
                uVar5 = *(unsigned short *)(PTR_DAT_0052da78 + uVar5 * 2) & 0x157;
            } else {
                uVar5 = (unsigned int)__isctype((int)uVar5, 0x157);
            }
            if ((uVar5 == 0) && (bVar6 != 0x20)) {
                if ((bVar6 == 0xd) || (bVar6 == 9)) {
                    local_28 = 1;
                    goto LAB_004da51b;
                }
                if ((bVar6 != 8) || (DAT_00665680 == 0)) goto LAB_004da51b;
                DAT_00665680 = (int)((unsigned int)DAT_00665680 - 1U);
                DAT_00665464 = (int)((unsigned int)DAT_00665464 - 1U);
            } else {
                if ((int)((unsigned int)param_3 - 1U) <= DAT_00665680) goto LAB_004da51b;
                param_2[DAT_00665680] = (char)bVar6;
                DAT_00665680 = (int)((unsigned int)DAT_00665680 + 1U);
                DAT_00665464 = (int)((unsigned int)DAT_00665464 + 1U);
            }
            param_2[DAT_00665680] = '\0';
        }
    }
LAB_004da51b:
    iVar3 = FUN_004dbfa0(param_2, param_5);
    local_2c = 0;
    if ((int)((unsigned int)(param_1[2] - *param_1) + 1U) < iVar3) {
        local_2c = (int)((((unsigned int)(param_1[2] - *param_1) - (unsigned int)param_5) - (unsigned int)iVar3) + 1U);
    }
    FUN_004d4cf0(local_24);
    FUN_004d6460(local_10, (int)((unsigned int)*param_1 + 2U), (int)((unsigned int)param_1[1] + 2U), param_1[2], param_1[3]);
    FUN_004d4d20(local_10);
    FUN_004db4e0(param_5, param_2, (int)((unsigned int)*param_1 + (unsigned int)local_2c + 2U), (int)((unsigned int)param_1[1] + 2U), DAT_0052c014);
    FUN_004d4d20(local_24);
    if (DAT_0052c000 == param_4) {
        cVar1 = param_2[DAT_00665464];
        param_2[DAT_00665464] = '\0';
        iVar3 = FUN_004dbfa0(param_2, param_5);
        param_2[DAT_00665464] = cVar1;
        iVar4 = FUN_004d4de0();
        if (iVar4 % 10 < 5) {
            FUN_004d6480(local_24, param_1);
            local_24[1] = (int)((unsigned int)local_24[1] + 1U);
            local_24[0] = (int)((unsigned int)local_24[0] + (unsigned int)local_2c + (unsigned int)iVar3 + 1U);
            local_24[2] = (int)((unsigned int)local_24[0] + 1U);
            local_24[3] = (int)((unsigned int)local_24[1] + (unsigned int)param_5);
            FUN_004d67e0(local_24, 0, 0, (short)DAT_0052c014, 1);
        }
    }
    return local_28;
}
