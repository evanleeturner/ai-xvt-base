/* SPDX-License-Identifier: MIT */
/* z_bop 0049d440 */
extern unsigned int DAT_009e8f54;
extern unsigned int DAT_00527508;
extern int DAT_005233d8;
extern unsigned char *DAT_009994bc;
extern unsigned char DAT_00612ba0[];
extern char s_DEMO_TIE_00523448[];
extern char s_newpal_inv_00527720[];
extern char s_newpal_act_005236b0[];
extern unsigned char DAT_00523434;
extern unsigned char DAT_00523438;
extern unsigned char DAT_00523444[];
extern unsigned char DAT_00523445[];
extern unsigned char DAT_00523446[];
extern void *DAT_00a07bd8;
extern int DAT_00527ec8;
extern unsigned char DAT_009a7c80[];
extern unsigned char *DAT_009a8074;
extern void (*DAT_009d77f4)(unsigned char *, int, int);
extern char DAT_005233c8;
extern char DAT_00523408;
extern unsigned char DAT_009a7bc0;

int FUN_0049e720(char *param_1, void *param_2, int param_3, int param_4);
void FUN_0040e9e0(int param_1, int param_2, int param_3);
int _fwrite(void *param_1, int param_2, int param_3, void *param_4);
void FUN_0049e9c0(int param_1);
void FUN_0049c3c0(char *param_1, unsigned char *param_2);
void FUN_004451e0(int param_1, int param_2);
void FUN_0049d010(void);
void FUN_004afa90(void);
void FUN_004451f0(int param_1, int param_2, void *param_3);
unsigned int FUN_0040e940(unsigned char *param_1, int param_2, int param_3, int param_4);

void __stdcall FUN_0049d440(void)
{
    char cVar1;
    unsigned char uVar2;
    unsigned char bVar3;
    unsigned char *puVar4;
    int iVar5;
    unsigned int uVar6;
    int iVar7;
    char *pcVar8;
    unsigned char bStack_104[3];
    char local_100[12];

    DAT_009e8f54 = DAT_009e8f54 & DAT_00527508;
    if (DAT_005233d8 != 1) goto LAB_0049d5cc;
    uVar6 = 0xffffffff;
    DAT_009994bc = DAT_00612ba0;
    pcVar8 = s_DEMO_TIE_00523448;
    do {
        if (uVar6 == 0) break;
        uVar6 = uVar6 - 1;
        cVar1 = *pcVar8;
        pcVar8 = pcVar8 + 1;
    } while (cVar1 != '\0');
    uVar6 = ~uVar6;
    uVar2 = DAT_00523444[uVar6];
    bStack_104[1] = DAT_00523445[uVar6];
    bStack_104[2] = DAT_00523446[uVar6];
    DAT_00523444[uVar6] = 0x69;
    DAT_00523445[uVar6] = 0x6e;
    DAT_00523446[uVar6] = 0x76;
    iVar5 = FUN_0049e720(s_DEMO_TIE_00523448, &DAT_00523434, 0, 0);
    if (iVar5 == 0) {
        local_100[0] = s_newpal_inv_00527720[0];
        local_100[1] = s_newpal_inv_00527720[1];
        local_100[2] = s_newpal_inv_00527720[2];
        local_100[3] = s_newpal_inv_00527720[3];
        local_100[4] = s_newpal_inv_00527720[4];
        local_100[5] = s_newpal_inv_00527720[5];
        local_100[6] = s_newpal_inv_00527720[6];
        local_100[7] = s_newpal_inv_00527720[7];
        local_100[8] = s_newpal_inv_00527720[8];
        local_100[9] = s_newpal_inv_00527720[9];
        local_100[10] = s_newpal_inv_00527720[10];
        iVar5 = FUN_0049e720(local_100, &DAT_00523434, 0, 0);
        if (iVar5 != 0) {
            FUN_0049e9c0(0);
            pcVar8 = local_100;
            goto LAB_0049d5aa;
        }
        FUN_0040e9e0((int)DAT_009994bc, 0x40, 0x100);
        iVar5 = FUN_0049e720(s_DEMO_TIE_00523448, &DAT_00523438, 0, 1);
        if (iVar5 != 0) {
            _fwrite(DAT_009994bc, 0x100, 0x100, DAT_00a07bd8);
            FUN_0049e9c0(1);
        }
    } else {
        FUN_0049e9c0(0);
        pcVar8 = s_DEMO_TIE_00523448;
LAB_0049d5aa:
        FUN_0049c3c0(pcVar8, DAT_009994bc);
    }
    DAT_00523444[uVar6] = uVar2;
    DAT_00523445[uVar6] = bStack_104[1];
    DAT_00523446[uVar6] = bStack_104[2];
LAB_0049d5cc:
    if ((DAT_009e8f54 != 0) && (DAT_005233d8 == 1)) {
        FUN_004451e0(0xc0, 8);
    }
    DAT_00527ec8 = 1;
    FUN_0049d010();
    DAT_00527ec8 = 0;
    FUN_004afa90();
    if ((DAT_009e8f54 != 0) && (DAT_005233d8 == 1)) {
        FUN_004451f0(0xc0, 8, DAT_009a7c80);
        uVar6 = 0xffffffff;
        pcVar8 = s_DEMO_TIE_00523448;
        do {
            if (uVar6 == 0) break;
            uVar6 = uVar6 - 1;
            cVar1 = *pcVar8;
            pcVar8 = pcVar8 + 1;
        } while (cVar1 != '\0');
        uVar6 = ~uVar6;
        uVar2 = DAT_00523444[uVar6];
        bStack_104[1] = DAT_00523445[uVar6];
        bStack_104[2] = DAT_00523446[uVar6];
        DAT_00523444[uVar6] = 0x70;
        DAT_00523445[uVar6] = 0x61;
        DAT_00523446[uVar6] = 0x6c;
        iVar5 = FUN_0049e720(s_DEMO_TIE_00523448, &DAT_00523438, 0, 1);
        if (iVar5 != 0) {
            _fwrite(DAT_009a7c80, 0x240, 1, DAT_00a07bd8);
            FUN_0049e9c0(1);
        }
        DAT_009994bc = DAT_00612ba0;
        FUN_0040e9e0((int)DAT_00612ba0, 0x40, 0x100);
        DAT_00523444[uVar6] = 0x69;
        DAT_00523445[uVar6] = 0x6e;
        DAT_00523446[uVar6] = 0x76;
        iVar5 = FUN_0049e720(s_DEMO_TIE_00523448, &DAT_00523438, 0, 1);
        if (iVar5 != 0) {
            _fwrite(DAT_009994bc, 0x100, 0x100, DAT_00a07bd8);
            FUN_0049e9c0(1);
        }
        puVar4 = DAT_009a8074;
        DAT_00523444[uVar6] = uVar2;
        DAT_00523445[uVar6] = bStack_104[1];
        DAT_00523446[uVar6] = bStack_104[2];
        FUN_0049c3c0(s_newpal_act_005236b0, puVar4);
        iVar5 = 0;
        do {
            bVar3 = DAT_009a8074[iVar5];
            DAT_009a8074[iVar5] = (unsigned char)DAT_009a8074[0x2fd - iVar5] >> 2;
            DAT_009a8074[0x2fd - iVar5] = bVar3 >> 2;
            bVar3 = DAT_009a8074[iVar5 + 1];
            DAT_009a8074[iVar5 + 1] = (unsigned char)DAT_009a8074[0x2fe - iVar5] >> 2;
            DAT_009a8074[0x2fe - iVar5] = bVar3 >> 2;
            bVar3 = DAT_009a8074[iVar5 + 2];
            DAT_009a8074[iVar5 + 2] = (unsigned char)DAT_009a8074[0x2ff - iVar5] >> 2;
            iVar7 = iVar5 + 3;
            DAT_009a8074[0x2ff - iVar5] = bVar3 >> 2;
            iVar5 = iVar7;
        } while (iVar7 < 0x180);
        (*DAT_009d77f4)(DAT_009a8074, 0, 0x100);
    }
    bStack_104[0] = 0;
    bStack_104[1] = 0;
    bStack_104[2] = 2;
    uVar6 = FUN_0040e940(bStack_104, (int)&DAT_009a7bc0, 0, 0x100);
    DAT_005233c8 = (char)uVar6;
    DAT_00523408 = (char)uVar6;
    return;
}
