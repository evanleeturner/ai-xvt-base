/* SPDX-License-Identifier: MIT */
/* z_bop 004515d0 */
extern char DAT_009e9a77[];
extern int DAT_009ecc34;
extern unsigned char DAT_009e9670[];
extern int DAT_005233ec;
extern short DAT_00a09288[];
extern short DAT_00a0928a[];
extern short DAT_00a09290[];
extern short DAT_00a09292[];
extern unsigned short DAT_00a08374;
extern void (*DAT_009d1300)(void);
extern char DAT_005235e0;
extern int DAT_009a8d90;
extern int DAT_009a7ba0;
extern int DAT_00a0829c;
extern char DAT_009a1ed0[];
extern char s__d__d_005240f4[];
extern int DAT_00a080f8;
extern unsigned char DAT_009ec464;
extern int DAT_009a806c;
extern int DAT_009a7bac;
extern int DAT_009a8c14;
extern int DAT_005233d8;
extern int DAT_005235dc;
extern unsigned char DAT_009ed232;
extern int DAT_009d1148;
extern int DAT_009a7ec0;
extern int DAT_009a7b44;
extern int DAT_009a8bfc;
extern unsigned char DAT_005233c8;
extern int DAT_00a0a120;
extern int DAT_009ec45c;
extern int DAT_00a0a124;

int _sprintf(char *, const char *, ...);
void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

void FUN_004aba50(void);
void FUN_0040e0c0(int, int, int, int);
void FUN_004a95c0(unsigned int);
void FUN_004a9640(char);
void FUN_0043c290(unsigned short, unsigned char);
void FUN_004a9500(short, short);
void FUN_004a9520(short, short, short, short);
void FUN_004a9590(unsigned int);
void FUN_004a9a50(char *);
void FUN_004abe50(void);
void FUN_004a95f0(unsigned int);
void FUN_004a9960(int);
void FUN_004a9ac0(int);

void __stdcall FUN_004515d0(void)
{
    int iVar1;
    unsigned char bVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    int iVar7;
    int iVar8;

    iVar1 = (int)((unsigned int)DAT_009ecc34 * 0x5bdU);
    if ((DAT_009e9a77[iVar1] != '\0') && (DAT_009e9a77[iVar1] != '\x13')) {
        return;
    }
    FUN_004aba50();
    FUN_0040e0c0(0, 0x140, 200, 0);
    FUN_004a95c0(0x2c);
    FUN_004a9640('\0');
    if (DAT_005233ec == 0x13) {
        bVar4 = 7;
    } else {
        bVar4 = 3;
    }
    __builtin_memcpy(&uVar6, DAT_009e9670 + (int)((unsigned int)DAT_009ecc34 * 0x5bdU), 4);
    FUN_0043c290((unsigned short)uVar6, bVar4);
    FUN_004a9500(DAT_00a09288[(unsigned int)DAT_00a08374 * 6],
                 DAT_00a0928a[(unsigned int)DAT_00a08374 * 6]);
    iVar1 = (int)((unsigned int)DAT_00a08374 * 6);
    FUN_004a9520(DAT_00a09288[iVar1], DAT_00a0928a[iVar1],
                 (short)(DAT_00a09290[iVar1] + DAT_00a09288[iVar1]),
                 (short)(DAT_00a09292[iVar1] + DAT_00a0928a[iVar1]));
    (*DAT_009d1300)();
    if ((((DAT_005235e0 != '\0') && (DAT_009a8d90 != 0)) && (DAT_009a7ba0 != 0)) &&
        (DAT_00a0829c != 0)) {
        FUN_004a9590(0x4e);
        iVar7 = (int)(0xec / (long long)DAT_009a8d90);
        if ((DAT_009a7ba0 == (int)0x80000000 && DAT_00a0829c == -1)) {
            RaiseException(0xC0000094UL, 0, 0, NULL);
        }
        iVar8 = DAT_009a7ba0 / DAT_00a0829c;
        if (iVar8 == 0) {
            RaiseException(0xC0000094UL, 0, 0, NULL);
        }
        _sprintf(DAT_009a1ed0, s__d__d_005240f4, iVar7, (int)(0xec / (long long)iVar8));
    }
    FUN_004a9a50(DAT_009a1ed0);
    FUN_004abe50();
    if (DAT_00a080f8 < 2) {
        return;
    }
    DAT_009ec464 = 0;
    FUN_004a9640('\0');
    FUN_0040e0c0(DAT_009a806c, DAT_009a7bac, DAT_009a8c14,
                 (int)((unsigned int)DAT_005233d8 * (unsigned int)DAT_009a7bac));
    FUN_004a9520((short)((short)DAT_005235dc + (unsigned short)DAT_009ed232 * -2),
                 (short)DAT_009d1148,
                 (short)((short)DAT_009a7ec0 + (unsigned short)DAT_009ed232 * 2 + 1),
                 (short)DAT_009a7b44);
    FUN_004a9500((short)((short)DAT_005235dc + (unsigned short)DAT_009ed232 * -2),
                 (short)DAT_009d1148);
    switch (DAT_009a8bfc) {
    case 0:
        FUN_004a95c0((unsigned int)DAT_005233c8);
        FUN_004a9590((unsigned int)DAT_005233c8);
        FUN_004a95f0((unsigned int)DAT_005233c8);
        goto switchD_004517f0_default;
    case 1:
        FUN_004a95c0(0x41);
        uVar5 = 0x4e;
        break;
    case 2:
        FUN_004a95c0(0x41);
        uVar5 = 0x4b;
        break;
    case 3:
        FUN_004a95c0(0x41);
        uVar5 = 0x4a;
        break;
    default:
        goto switchD_004517f0_default;
    }
    FUN_004a9590(uVar5);
switchD_004517f0_default:
    FUN_004a9960(DAT_00a0a120);
    switch (DAT_009ec45c) {
    case 0:
        FUN_004a95c0((unsigned int)DAT_005233c8);
        FUN_004a9590((unsigned int)DAT_005233c8);
        FUN_004a95f0((unsigned int)DAT_005233c8);
        goto switchD_0045186e_default;
    case 1:
        FUN_004a95c0(0x41);
        uVar5 = 0x4e;
        break;
    case 2:
        FUN_004a95c0(0x41);
        uVar5 = 0x4a;
        break;
    case 3:
        FUN_004a95c0(0x41);
        uVar5 = 0x4b;
        break;
    default:
        goto switchD_0045186e_default;
    }
    FUN_004a9590(uVar5);
switchD_0045186e_default:
    FUN_004a9ac0(DAT_00a0a124);
    FUN_004a95f0(0x40);
    FUN_0040e0c0(0, 0x140, 200, 0);
    return;
}
