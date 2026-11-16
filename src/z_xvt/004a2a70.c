/* SPDX-License-Identifier: MIT */
/* z_xvt 004a2a70 */
extern short DAT_0070ac40;
extern short DAT_0070ac46;
extern short DAT_0070ac48;
extern short DAT_0070ac4a;
extern int DAT_0071ecf4;
extern short DAT_006d9990;
extern int DAT_00645b8c;
extern short DAT_006559b4;
extern unsigned char DAT_0070a178;
extern short DAT_006d99e4;
extern unsigned char DAT_006d9999;
extern unsigned char DAT_00525508;
extern int DAT_006da9ec;
extern int DAT_006da538;
extern int DAT_006db594;
extern int DAT_00525518;
extern int DAT_0052571c;
extern int DAT_00703ad0;
extern int DAT_006da840;
extern int DAT_006da4e4;
extern void (*DAT_00703c80)(void);
extern short DAT_00645ae0;
extern int DAT_006d9d30;
extern int DAT_00703c78;
extern int DAT_006db714;
extern int DAT_006db710;
extern short DAT_00645b38;
extern int DAT_0071e530;
extern int DAT_0071ecf8;
extern int DAT_00703c74;
extern int DAT_007092b4;
extern unsigned char DAT_0071b74a;
extern char DAT_0071bb37;
extern unsigned int DAT_0070a014;
extern char DAT_0071b740;
extern char DAT_0071e528;

void FUN_004a2440(unsigned short param_1);
void FUN_0048eda0(unsigned int param_1);
void FUN_00461f20(int param_1, int param_2, int param_3, int param_4);
void FUN_0048ed00(short param_1, short param_2, short param_3, short param_4);
unsigned int FUN_004a3830(unsigned int param_1, int param_2, int param_3, int param_4);

void __stdcall FUN_004a2a70(void)
{
    char cVar1;
    short sVar2;
    unsigned int uVar3;
    char *pcVar4;
    int iVar5;
    unsigned short uVar6;
    unsigned short uVar7;
    unsigned int *puVar8;
    unsigned int *puVar9;
    short sTmp;
    int iVal;

    if (((((*(short *)((char *)&DAT_0070ac40 + (unsigned int)DAT_0071ecf4 * 0x10u) == 0) && (DAT_006d9990 != -1)) || (DAT_00645b8c != 0)) &&
        (DAT_006559b4 == 0))) {
        if (DAT_0070a178 == 0) {
            FUN_0048eda0((unsigned int)DAT_00525508);
            FUN_00461f20(DAT_006da9ec, DAT_006da538, DAT_006db594, (int)((unsigned int)DAT_00525518 * (unsigned int)DAT_006da538));
            FUN_0048ed00((short)DAT_0052571c, (short)DAT_00703ad0, (short)DAT_006da840, (short)DAT_006da4e4);
            (*DAT_00703c80)();
            FUN_00461f20(0, 0x140, 200, 0);
            DAT_006d9990 = -1;
        }
        else {
            uVar6 = 0;
            do {
                uVar7 = (unsigned short)(uVar6 + 1);
                puVar8 = (unsigned int *)((char *)&DAT_006d99e4 + (unsigned int)uVar6 * 0x54u);
                puVar9 = (unsigned int *)((char *)&DAT_006d9990 + (unsigned int)uVar6 * 0x54u);
                for (iVar5 = 0x15; iVar5 != 0; iVar5 = iVar5 - 1) {
                    *puVar9 = *puVar8;
                    puVar8 = puVar8 + 1;
                    puVar9 = puVar9 + 1;
                }
                uVar6 = uVar7;
            } while (uVar7 < DAT_0070a178);
            DAT_0070a178 = (unsigned char)(DAT_0070a178 - 1);
            FUN_004a2440((unsigned short)DAT_006d9999);
        }
    }
    if (((*(short *)((char *)&DAT_0070ac46 + (unsigned int)DAT_0071ecf4 * 0x10u) == 0) && (DAT_00645ae0 != -1)) ||
        (DAT_00645b8c != 0)) {
        FUN_0048eda0((unsigned int)DAT_00525508);
        FUN_00461f20(DAT_006da9ec, DAT_006da538, DAT_006db594, (int)((unsigned int)DAT_00525518 * (unsigned int)DAT_006da538));
        FUN_0048ed00((short)DAT_006d9d30, (short)DAT_00703c78, (short)DAT_006db714, (short)DAT_006db710);
        (*DAT_00703c80)();
        FUN_00461f20(0, 0x140, 200, 0);
        DAT_00645ae0 = -1;
    }
    if (((*(short *)((char *)&DAT_0070ac48 + (unsigned int)DAT_0071ecf4 * 0x10u) == 0) && (DAT_00645b38 != -1)) ||
        (DAT_00645b8c != 0)) {
        FUN_0048eda0((unsigned int)DAT_00525508);
        FUN_00461f20(DAT_006da9ec, DAT_006da538, DAT_006db594, (int)((unsigned int)DAT_00525518 * (unsigned int)DAT_006da538));
        FUN_0048ed00((short)DAT_0071e530, (short)DAT_0071ecf8, (short)DAT_00703c74, (short)DAT_007092b4);
        (*DAT_00703c80)();
        FUN_00461f20(0, 0x140, 200, 0);
        DAT_00645b38 = -1;
    }
    if ((((*(short *)((char *)&DAT_0070ac4a + (unsigned int)DAT_0071ecf4 * 0x10u) == 0) &&
          (__builtin_memcpy(&sVar2, (char *)&DAT_0071b74a + (unsigned int)DAT_0071ecf4 * 0x5bdu, 2), sVar2 != -1)) &&
         (uVar3 = FUN_004a3830((((unsigned int)DAT_0071ecf4 * 0x16fu) >> 0x10 << 0x10) | (unsigned int)(unsigned short)sVar2,
                               DAT_0071ecf4, 0, 0), uVar3 != DAT_0070a014)) &&
        (__builtin_memcpy(&sTmp, (char *)&DAT_0071b74a + (unsigned int)DAT_0071ecf4 * 0x5bdu, 2),
         uVar3 = FUN_004a3830((((unsigned int)DAT_0071ecf4 >> 0x10) << 0x10) | (unsigned int)(unsigned short)sTmp,
                              DAT_0071ecf4, 0, 1), uVar3 != 0)) {
        cVar1 = *((char *)&DAT_0071bb37 + (unsigned int)DAT_0071ecf4 * 0x5bdu);
        if (((cVar1 == '\x13') || (cVar1 == '\0')) || (cVar1 == '\x14')) {
            __builtin_memcpy(&sTmp, (char *)&DAT_0071b74a + (unsigned int)DAT_0071ecf4 * 0x5bdu, 2);
            DAT_0070a014 = FUN_004a3830((((unsigned int)DAT_0071ecf4 * 0x5bdu) >> 0x10 << 0x10) | (unsigned int)(unsigned short)sTmp,
                                        DAT_0071ecf4, 1, 0);
            *(short *)((char *)&DAT_0070ac4a + (unsigned int)DAT_0071ecf4 * 0x10u) = 0x49c;
        }
    }
    pcVar4 = &DAT_0071b740;
    do {
        if (*pcVar4 != '\0') {
            __builtin_memcpy(&iVal, pcVar4 + 0x59d, 4);
            if (iVal == 0) {
                pcVar4[0x36] = '\0';
            }
        }
        pcVar4 = pcVar4 + 0x5bd;
    } while (pcVar4 < &DAT_0071e528);
    if (DAT_00645b8c != 0) {
        DAT_00645b8c = 0;
    }
    return;
}
