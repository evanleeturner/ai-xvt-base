/* SPDX-License-Identifier: MIT */
/* z_xvt 004fc650 */
extern unsigned int DAT_0081bcd0;
extern unsigned char DAT_0081bfb6;
extern unsigned char DAT_007438d8[];
extern unsigned char DAT_0081f7ea[];
extern unsigned char DAT_0081faaa[];
extern char DAT_0081b740[];
extern char DAT_005321b8[];
extern char s__d__d__005321a8[];

void FUN_004dcfd0(int *param_1, int param_2, int param_3, int param_4, int param_5);
char *FUN_004dcfa0(int param_1);
void FUN_004da920(int param_1, char *param_2, int *param_3, unsigned int param_4);
void FUN_004da7c0(int param_1, char *param_2, int param_3, int param_4, unsigned int param_5);
int _sprintf(char *, const char *, ...);

unsigned int FUN_004fc650(void)
{
    char *pcVar1;
    int *piVar2;
    int iVar3;
    int iVar4;
    int iVar5;
    unsigned int uVar6;
    int local_20;
    int local_1c;
    int local_18;
    int local_14;
    int local_10[4];
    unsigned char *p;

    FUN_004dcfd0(local_10, 0x54, 0x5a, 0x1b2, 0x6a);
    piVar2 = local_10;
    uVar6 = 0xffff;
    pcVar1 = FUN_004dcfa0(300);
    FUN_004da920(0xf, pcVar1, piVar2, uVar6);
    iVar4 = 0x72;
    iVar3 = 0x58;
    uVar6 = DAT_0081bcd0;
    pcVar1 = FUN_004dcfa0(0xae);
    FUN_004da7c0(0xc, pcVar1, iVar3, iVar4, uVar6);
    iVar4 = 0x72;
    iVar3 = 0xf8;
    uVar6 = DAT_0081bcd0;
    pcVar1 = FUN_004dcfa0(0x13d);
    FUN_004da7c0(0xc, pcVar1, iVar3, iVar4, uVar6);
    iVar4 = 0x72;
    iVar3 = 0x134;
    uVar6 = DAT_0081bcd0;
    pcVar1 = FUN_004dcfa0(0x143);
    FUN_004da7c0(0xc, pcVar1, iVar3, iVar4, uVar6);
    iVar5 = 0x72;
    iVar4 = 0x170;
    iVar3 = 0;
    uVar6 = DAT_0081bcd0;
    pcVar1 = FUN_004dcfa0(0x144);
    FUN_004da7c0(0xc, pcVar1, iVar4, iVar5, uVar6);
    local_20 = 0;
    local_1c = 0;
    local_18 = 0;
    local_14 = 0;
    iVar4 = 0x86;
    p = DAT_0081f7ea;
    do {
        int m4, m3, m1, a0, a1, a2, a3, a4;
        __builtin_memcpy(&m3, p - 12, 4);
        if (m3 != 0) {
            __builtin_memcpy(&m4, p - 16, 4);
            if (DAT_007438d8[(int)((unsigned int)m4 * 0x562u)] == DAT_0081bfb6) {
                __builtin_memcpy(&a0, p, 4);
                __builtin_memcpy(&a1, p + 4, 4);
                __builtin_memcpy(&a2, p + 8, 4);
                __builtin_memcpy(&a3, p + 12, 4);
                __builtin_memcpy(&a4, p + 16, 4);
                __builtin_memcpy(&m1, p - 4, 4);
                iVar3 = (int)((unsigned int)iVar3 + (unsigned int)a0);
                local_20 = (int)((unsigned int)local_20 + (unsigned int)a1);
                local_1c = (int)((unsigned int)local_1c + (unsigned int)a3);
                local_18 = (int)((unsigned int)local_18 + (unsigned int)a2);
                local_14 = (int)((unsigned int)local_14 + (unsigned int)a4);
                FUN_004da7c0(0xc, (char *)(p - 0x1e), 0x58, iVar4, 0xffff);
                _sprintf(DAT_0081b740, DAT_005321b8, m1);
                FUN_004da7c0(0xc, DAT_0081b740, 0xf8, iVar4, 0xffff);
                _sprintf(DAT_0081b740, s__d__d__005321a8, a0, a1);
                FUN_004da7c0(0xc, DAT_0081b740, 0x134, iVar4, 0xffff);
                _sprintf(DAT_0081b740, DAT_005321b8, a2);
                FUN_004da7c0(0xc, DAT_0081b740, 0x170, iVar4, 0xffff);
            }
            iVar4 = (int)((unsigned int)iVar4 + 0x14u);
        }
        p = p + 0x58;
    } while (p < DAT_0081faaa);
    iVar5 = 0x132;
    iVar4 = 0x58;
    uVar6 = DAT_0081bcd0;
    pcVar1 = FUN_004dcfa0(0x11c);
    FUN_004da7c0(0xc, pcVar1, iVar4, iVar5, uVar6);
    _sprintf(DAT_0081b740, DAT_005321b8, iVar3);
    FUN_004da7c0(0xc, DAT_0081b740, 0x120, 0x132, 0xffff);
    iVar4 = 0x146;
    iVar3 = 0x58;
    uVar6 = DAT_0081bcd0;
    pcVar1 = FUN_004dcfa0(0x141);
    FUN_004da7c0(0xc, pcVar1, iVar3, iVar4, uVar6);
    _sprintf(DAT_0081b740, DAT_005321b8, local_20);
    FUN_004da7c0(0xc, DAT_0081b740, 0x120, 0x146, 0xffff);
    iVar4 = 0x15a;
    iVar3 = 0x58;
    uVar6 = DAT_0081bcd0;
    pcVar1 = FUN_004dcfa0(0x11e);
    FUN_004da7c0(0xc, pcVar1, iVar3, iVar4, uVar6);
    _sprintf(DAT_0081b740, DAT_005321b8, local_1c);
    FUN_004da7c0(0xc, DAT_0081b740, 0x120, 0x15a, 0xffff);
    iVar4 = 0x16e;
    iVar3 = 0x58;
    uVar6 = DAT_0081bcd0;
    pcVar1 = FUN_004dcfa0(0x11f);
    FUN_004da7c0(0xc, pcVar1, iVar3, iVar4, uVar6);
    _sprintf(DAT_0081b740, DAT_005321b8, local_18);
    FUN_004da7c0(0xc, DAT_0081b740, 0x120, 0x16e, 0xffff);
    iVar4 = 0x182;
    iVar3 = 0x58;
    uVar6 = DAT_0081bcd0;
    pcVar1 = FUN_004dcfa0(0x122);
    FUN_004da7c0(0xc, pcVar1, iVar3, iVar4, uVar6);
    _sprintf(DAT_0081b740, DAT_005321b8, local_14);
    FUN_004da7c0(0xc, DAT_0081b740, 0x120, 0x182, 0xffff);
    return 1;
}
