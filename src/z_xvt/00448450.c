/* SPDX-License-Identifier: MIT */
/* z_xvt 00448450 */
extern unsigned char *DAT_006d4988;
extern unsigned char *DAT_006db798;
extern int DAT_0071b704;
extern int DAT_00732904;
extern int DAT_00732a24;
extern unsigned int DAT_00732a28;
extern unsigned int DAT_00732a2c;
extern int DAT_0071ecf4;
extern unsigned char DAT_00524870[];
extern unsigned char DAT_0070ad45[];
extern unsigned char DAT_0070ad47[];
extern unsigned char DAT_0070ad48[];
extern unsigned char DAT_00525bf7[];
extern unsigned short DAT_00525aae[];
extern unsigned short DAT_00525a7e[];
extern unsigned short DAT_00525ade[];
extern unsigned short DAT_00525b0e[];
extern unsigned char DAT_0071b776[];
extern unsigned char DAT_0071b777[];
extern unsigned char DAT_0071bcdd[];
extern unsigned char DAT_0071b730[];

unsigned short FUN_00422710(int param_1);
unsigned int FUN_00478320(unsigned int param_1, unsigned int param_2);
void FUN_00421c70(unsigned int param_1, int param_2);
unsigned char FUN_00478470(void);
void FUN_004a1d70(int param_1, int param_2);
void FUN_0040b730(int param_1, unsigned int param_2, int param_3, int param_4,
                  unsigned int param_5, unsigned int param_6);

unsigned short __cdecl FUN_00448450(unsigned short param_1, unsigned short param_2)
{
    short sVar1;
    unsigned int iVar2;
    unsigned int uVar3;
    unsigned char bVar4;
    unsigned short uVar5;
    unsigned int uVar6;
    unsigned int iVar7;
    unsigned int iVar8;
    unsigned int uVar9;
    unsigned short uVar10;
    unsigned char *p;
    unsigned char *q;
    unsigned short us;
    unsigned int ui;
    int si;
    short ss;

    iVar2 = ((unsigned int)param_1 * 8u - (unsigned int)param_1) * 5u;
    uVar6 = (unsigned int)DAT_006d4988[0x16 + iVar2];
    iVar7 = uVar6 * 0x566u;
    bVar4 = DAT_00524870[DAT_0070ad45[iVar7]];
    if (bVar4 == 0) {
        return 0xffff;
    }
    uVar5 = FUN_00422710(7);
    if (uVar5 == 0xffff) {
        uVar5 = (unsigned short)((short)DAT_0071b704 + 0x80);
        while ((int)(unsigned int)uVar5 < DAT_00732904) {
            iVar8 = (unsigned int)uVar5 * 0x23u + (unsigned int)DAT_006d4988;
            __builtin_memcpy(&p, (unsigned char *)iVar8 + 0x1f, 4);
            if (!(DAT_00525bf7[*((unsigned char *)iVar8 + 3)] != 0 ||
                  *(char *)(p + 0x87) != (char)DAT_0070ad48[iVar7])) {
                break;
            }
            uVar5 = (unsigned short)(uVar5 + 1);
        }
    }
    uVar6 = (unsigned int)uVar5;
    if (DAT_00732904 == (int)uVar6) {
        return 0xffff;
    }
    iVar8 = uVar6 * 0x23u;

    __builtin_memcpy(&p, DAT_006d4988 + 0x1f + iVar8, 4);
    *p = 1;
    DAT_006d4988[2 + iVar8] = 7;
    DAT_006d4988[3 + iVar8] = bVar4;
    __builtin_memcpy(&p, DAT_006d4988 + 0x1f + iVar8, 4);
    us = 1;
    __builtin_memcpy(p + 0x81, &us, 2);
    __builtin_memcpy(&p, DAT_006d4988 + 0x1f + iVar8, 4);
    __builtin_memcpy(p + 0x83, &param_1, 2);
    __builtin_memcpy(&p, DAT_006d4988 + 0x1f + iVar8, 4);
    p[0x85] = DAT_006d4988[3 + iVar2];
    __builtin_memcpy(&p, DAT_006d4988 + 0x1f + iVar8, 4);
    p[0x86] = DAT_0070ad47[iVar7];
    uVar9 = (unsigned int)bVar4;
    us = 0;
    __builtin_memcpy(DAT_006d4988 + 0x12 + iVar8, &us, 2);
    __builtin_memcpy(DAT_006d4988 + 0x14 + iVar8, &us, 2);
    __builtin_memcpy(DAT_006d4988 + 0x10 + iVar8, &us, 2);
    __builtin_memcpy(&p, DAT_006d4988 + 0x1f + iVar8, 4);
    us = DAT_00525aae[uVar9];
    __builtin_memcpy(p + 0x79, &us, 2);
    __builtin_memcpy(&p, DAT_006d4988 + 0x1f + iVar8, 4);
    __builtin_memcpy(&us, p + 0x79, 2);
    __builtin_memcpy(DAT_006db798 + 6 + (uVar6 - (unsigned int)DAT_0071b704) * 8u, &us, 2);
    iVar7 = (unsigned int)DAT_006d4988;
    __builtin_memcpy(&p, DAT_006d4988 + 0x1f + iVar8, 4);
    us = DAT_00525a7e[uVar9];
    __builtin_memcpy(p + 0x7d, &us, 2);
    sVar1 = (short)DAT_00525ade[uVar9];
    uVar6 = FUN_00478320((((iVar7 >> 0x10) & 0xffffu) << 0x10) |
                         (unsigned int)DAT_00525b0e[uVar9], 0xec);
    __builtin_memcpy(&p, DAT_006d4988 + 0x1f + iVar8, 4);
    ss = (short)((unsigned int)(int)sVar1 * 0xecu + (unsigned int)(int)(short)uVar6);
    __builtin_memcpy(p + 0x7f, &ss, 2);

    FUN_00421c70((unsigned int)param_1, 0);

    uVar3 = DAT_00732a28;
    iVar7 = (unsigned int)DAT_00732a24 + 0x180u;
    __builtin_memcpy(&p, DAT_006d4988 + 0x1f + iVar8, 4);
    ui = DAT_00732a2c;
    __builtin_memcpy(p + 6, &ui, 4);
    __builtin_memcpy(&p, DAT_006d4988 + iVar8 + 0x1f, 4);
    __builtin_memcpy(&ui, p + 6, 4);
    __builtin_memcpy(DAT_006d4988 + iVar8 + 4, &ui, 4);
    __builtin_memcpy(&p, DAT_006d4988 + 0x1f + iVar8, 4);
    __builtin_memcpy(p + 10, &uVar3, 4);
    __builtin_memcpy(&p, DAT_006d4988 + iVar8 + 0x1f, 4);
    __builtin_memcpy(&ui, p + 10, 4);
    __builtin_memcpy(DAT_006d4988 + iVar8 + 8, &ui, 4);
    __builtin_memcpy(&p, DAT_006d4988 + 0x1f + iVar8, 4);
    __builtin_memcpy(p + 0xe, &iVar7, 4);
    __builtin_memcpy(&p, DAT_006d4988 + iVar8 + 0x1f, 4);
    __builtin_memcpy(&ui, p + 0xe, 4);
    __builtin_memcpy(DAT_006d4988 + iVar8 + 0xc, &ui, 4);

    uVar10 = (unsigned short)(uVar5 - (short)DAT_0071b704);
    iVar7 = (unsigned int)uVar10 * 8u;
    bVar4 = FUN_00478470();
    DAT_006db798[iVar7 + 1] = (unsigned char)((bVar4 & 3) + 3);
    __builtin_memcpy(DAT_006db798 + iVar7 + 4, &param_2, 2);
    DAT_006db798[iVar7] = 0xff;
    __builtin_memcpy(&p, DAT_006d4988 + 0x1f + iVar8, 4);
    q = DAT_006db798 + iVar7;
    __builtin_memcpy(p + 0xa3, &q, 4);

    __builtin_memcpy(&si, DAT_006d4988 + (unsigned int)param_2 * 0x23u + 0x1b, 4);
    if (si != -1) {
        __builtin_memcpy(&us, DAT_006db798 + iVar7 + 4, 2);
        __builtin_memcpy(&si, DAT_006d4988 + (unsigned int)us * 0x23u + 0x1b, 4);
        if (si != -1) {
            iVar7 = (unsigned int)si;
            iVar2 = iVar7 * 0x5bdu;
            if (DAT_0071b776[iVar2] == 0) {
                DAT_0071b776[iVar2] = 1;
                iVar8 = (unsigned int)DAT_0071ecf4;
                us = (unsigned short)(uVar10 + (short)DAT_0071b704);
                __builtin_memcpy(&DAT_0071b777[iVar2], &us, 2);
                ui = 0x588;
                __builtin_memcpy(&DAT_0071bcdd[iVar2], &ui, 4);
                if (iVar7 == iVar8) {
                    FUN_004a1d70(0x73, (int)iVar7);
                    __builtin_memcpy(&ui, &DAT_0071b730[iVar2], 4);
                    FUN_0040b730(DAT_0071ecf4, 0xffffffffu, 0xc, -1, ui, 0xffff);
                }
            }
        }
    }
    return uVar5;
}
