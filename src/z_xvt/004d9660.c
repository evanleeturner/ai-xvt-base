/* SPDX-License-Identifier: MIT */
/* z_xvt 004d9660 */
/* Prototypes */
extern int FUN_004df760(int a, int b, int c);
extern void FUN_004ddc70(int *a, int *b);
extern int FUN_004dd7d0(int *a, int b, int c);
extern unsigned char FUN_004db570(void);
extern unsigned char FUN_004db590(void);
extern unsigned int FUN_004dbe80(void);
extern void FUN_004dbec0(void);
extern unsigned int __isctype(unsigned int c, unsigned int mask);
extern int FUN_004db2f0(char *s, int a);
extern void FUN_004d5180(int *a);
extern void FUN_004dcfd0(unsigned int *a, int b, int c, int d, int e);
extern void FUN_004d51b0(int *a);
extern void FUN_004da7c0(int a, char *s, int b, int c, int d);
extern int FUN_004d5270(void);
extern void FUN_004dd000(int *a, int *b);
extern void FUN_004dd3e0(int *a, int b, int c, short d, int e);

/* Globals */
extern int DAT_00532af0;
extern int DAT_00532aec;
extern int DAT_0064f67c;
extern int DAT_0064f898;
extern int DAT_00532ad8;
extern int DAT_00534aac;
extern char *PTR_DAT_005348a0;

unsigned int __cdecl FUN_004d9660(int *param_1, char *param_2, int param_3, int param_4, int param_5, char *param_6)
{
    char cVar1;
    unsigned char uVar2;
    int iVar3;
    int iVar4;
    unsigned int uVar5;
    unsigned char bVar6;
    char *pcVar7;
    int iStack_2c;
    int iStack_28[4];
    unsigned int uStack_18;
    int iStack_14;
    unsigned int auStack_10[4];

    if (DAT_00532af0 == 0) {
        DAT_00532aec = FUN_004df760(0xff, 0xff, 0xff);
        DAT_00532af0 = 1;
    }
    uVar5 = 0xffffffff;
    pcVar7 = param_2;
    do {
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
    } while (cVar1 != '\0');
    DAT_0064f67c = (int)(~uVar5 - 1);
    uVar5 = 0xffffffff;
    pcVar7 = param_2;
    do {
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
    } while (cVar1 != '\0');
    DAT_0064f898 = (int)(~uVar5 - 1);
    uStack_18 = 0;
    FUN_004ddc70(&iStack_2c, &iStack_14);
    iVar3 = FUN_004dd7d0(param_1, iStack_2c, iStack_14);
    if ((iVar3 != 0) &&
        ((uVar2 = FUN_004db570(), (unsigned int)uVar2 != 0) ||
         (uVar2 = FUN_004db590(), (unsigned int)uVar2 != 0))) {
        DAT_00532ad8 = param_4;
    }
    if (param_4 == DAT_00532ad8) {
        uVar5 = FUN_004dbe80();
        bVar6 = (unsigned char)uVar5;
        if (param_6 != (char *)0) {
            iVar3 = 0;
            cVar1 = *param_6;
            while (cVar1 != '\0') {
                if (param_6[iVar3] == (char)bVar6) {
                    bVar6 = 0;
                    FUN_004dbec0();
                    break;
                }
                iVar4 = iVar3 + 1;
                iVar3 = iVar3 + 1;
                cVar1 = param_6[iVar4];
            }
        }
        if (bVar6 != 0) {
            uVar5 = (unsigned int)bVar6;
            if (uVar5 != 0x1b) {
                FUN_004dbec0();
            }
            if (DAT_00534aac < 2) {
                uVar5 = *(unsigned short *)(PTR_DAT_005348a0 + uVar5 * 2) & 0x157;
            } else {
                uVar5 = __isctype(uVar5, 0x157);
            }
            if ((uVar5 == 0) && (bVar6 != 0x20)) {
                if ((bVar6 == 0xd) || (bVar6 == 9)) {
                    uStack_18 = 1;
                    goto LAB_004d9808;
                }
                if ((bVar6 != 8) || (DAT_0064f898 == 0)) goto LAB_004d9808;
                DAT_0064f898 = (int)((unsigned int)DAT_0064f898 - 1u);
                DAT_0064f67c = (int)((unsigned int)DAT_0064f67c - 1u);
            } else {
                if ((int)((unsigned int)param_3 - 1u) <= DAT_0064f898) goto LAB_004d9808;
                param_2[DAT_0064f898] = (char)bVar6;
                DAT_0064f67c = (int)((unsigned int)DAT_0064f67c + 1u);
                DAT_0064f898 = (int)((unsigned int)DAT_0064f898 + 1u);
            }
            param_2[DAT_0064f898] = '\0';
        }
    }
LAB_004d9808:
    iVar3 = FUN_004db2f0(param_2, param_5);
    iStack_2c = 0;
    if ((int)((unsigned int)(param_1[2] - *param_1) + 1u) < iVar3) {
        iStack_2c = (int)((((unsigned int)(param_1[2] - *param_1) - (unsigned int)iVar3) - (unsigned int)param_5) + 1u);
    }
    FUN_004d5180(iStack_28);
    FUN_004dcfd0(auStack_10, (int)((unsigned int)*param_1 + 2u), (int)((unsigned int)param_1[1] + 2u), param_1[2], param_1[3]);
    FUN_004d51b0((int *)auStack_10);
    FUN_004da7c0(param_5, param_2, (int)((unsigned int)*param_1 + (unsigned int)iStack_2c + 2u), (int)((unsigned int)param_1[1] + 2u), DAT_00532aec);
    FUN_004d51b0(iStack_28);
    if (param_4 == DAT_00532ad8) {
        cVar1 = param_2[DAT_0064f67c];
        param_2[DAT_0064f67c] = '\0';
        iVar3 = FUN_004db2f0(param_2, param_5);
        param_2[DAT_0064f67c] = cVar1;
        iVar4 = FUN_004d5270();
        if (iVar4 % 10 < 5) {
            FUN_004dd000(iStack_28, param_1);
            iStack_28[0] = (int)((unsigned int)iStack_28[0] + (unsigned int)iStack_2c + (unsigned int)iVar3 + 1u);
            iStack_28[1] = (int)((unsigned int)iStack_28[1] + 1u);
            iStack_28[2] = (int)((unsigned int)iStack_28[0] + 1u);
            iStack_28[3] = (int)((unsigned int)param_5 + (unsigned int)iStack_28[1]);
            FUN_004dd3e0(iStack_28, 0, 0, (short)DAT_00532aec, 1);
        }
    }
    return uStack_18;
}
