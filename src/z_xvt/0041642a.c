/* SPDX-License-Identifier: MIT */
/* z_xvt 0041642a */
extern short DAT_00730894;
extern unsigned short DAT_00732e58;
extern short DAT_0070acc0;
extern unsigned short DAT_005369d4;
extern unsigned short DAT_005369d8;
extern unsigned short DAT_005369d0;
extern unsigned short DAT_005369dc;
extern unsigned char DAT_006d4a4e;
extern unsigned short DAT_00732e52;
extern unsigned short DAT_006d9980;
extern unsigned short DAT_00732e8c;
extern unsigned short DAT_00732a34;
extern char DAT_0070a188;
extern char DAT_006d4998;
extern unsigned char DAT_0070acc2;
extern unsigned int DAT_00709ff4;
extern int DAT_006ffbf0;
extern char DAT_0071e524;
extern int DAT_0052552c;
extern unsigned int DAT_00525534;
extern unsigned int DAT_0052550c;
extern unsigned short DAT_00732e50;
extern unsigned short DAT_0071b012;
extern unsigned short DAT_006da9fa;
extern unsigned char DAT_007325e0[];

/* Placeholders for the register inputs (AL, ZF) the fragment expects from its caller. */
extern unsigned char in_AL_input;
extern unsigned char in_ZF_input;

void __cdecl FUN_004169b0(int param_1);
int  __cdecl FUN_00461ff0(int param_1);
void __stdcall FUN_004651f0(void);

void __stdcall FUN_0041642a(void)
{
    unsigned char bVar1;
    unsigned char bVar2;
    unsigned short *puVar3;
    unsigned char in_AL = in_AL_input;
    unsigned char *pbVar4;
    int iVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    unsigned int uVar8;
    unsigned int uVar9;
    unsigned char in_ZF = in_ZF_input;
    unsigned int *puVar10;
    char cStack00000004;
    int _cStack00000004;
    int iStack00000008;
    unsigned int uStack0000000c;
    unsigned int uStack00000010;

    if (in_ZF) {
        if ((DAT_00730894 != 0) && ((short)DAT_00732e58 < DAT_0070acc0)) {
            DAT_005369d4 = DAT_0070acc0;
            DAT_005369d8 = DAT_00732e58;
            DAT_005369d0 = DAT_006d4a4e + DAT_00732e52;
            DAT_005369dc = DAT_00732e52;
            if ((int)(unsigned int)DAT_00732e58 < (int)(short)DAT_006d9980) {
                DAT_005369d8 = DAT_006d9980;
            }
            if ((int)(unsigned int)DAT_00732e52 < (int)(short)DAT_00732e8c) {
                DAT_005369dc = DAT_00732e8c;
            }
            if ((int)(short)DAT_00732a34 < (int)(unsigned int)DAT_005369d0) {
                DAT_005369d0 = DAT_00732a34;
            }
            if (DAT_005369dc < DAT_005369d0) {
                FUN_004169b0(0);
            }
        }
        DAT_00732e58 = DAT_006d9980;
        DAT_00732e52 = DAT_00732e52 + DAT_006d4a4e + 1;
        return;
    }
    if (0x1f < in_AL) {
        if ((((DAT_0070a188 != '\0') && (DAT_006d4998 == '\0')) && (0x60 < in_AL)) && (in_AL < 0x7b)) {
            in_AL = in_AL - 0x20;
        }
        pbVar4 = (unsigned char *)((unsigned int)(unsigned char)(in_AL - 0x20) * (unsigned int)DAT_0070acc2 + DAT_00709ff4);
        bVar1 = *pbVar4;
        bVar2 = pbVar4[1];
        uStack0000000c = (unsigned int)(unsigned short)bVar1;
        if (((int)DAT_0070acc0 <= (int)((int)(short)DAT_00732e58 + uStack0000000c)) &&
            (DAT_006ffbf0 != 0)) {
            if ((DAT_00730894 != 0) && ((short)DAT_00732e58 < DAT_0070acc0)) {
                DAT_005369d4 = DAT_0070acc0;
                DAT_005369d8 = DAT_00732e58;
                DAT_005369d0 = DAT_006d4a4e + DAT_00732e52;
                DAT_005369dc = DAT_00732e52;
                if ((int)(unsigned int)DAT_00732e58 < (int)(short)DAT_006d9980) {
                    DAT_005369d8 = DAT_006d9980;
                }
                if ((int)(unsigned int)DAT_00732e52 < (int)(short)DAT_00732e8c) {
                    DAT_005369dc = DAT_00732e8c;
                }
                if ((int)(short)DAT_00732a34 < (int)(unsigned int)DAT_005369d0) {
                    DAT_005369d0 = DAT_00732a34;
                }
                if (DAT_005369dc < DAT_005369d0) {
                    FUN_004169b0(0);
                }
            }
            DAT_00732e58 = DAT_006d9980;
            DAT_00732e52 = DAT_00732e52 + bVar2 + 2;
        }
        uVar8 = 0;
        iStack00000008 = (int)(short)DAT_00732e52;
        uStack00000010 = (unsigned int)(unsigned short)bVar2;
        puVar10 = (unsigned int *)(pbVar4 + 2);
        if (iStack00000008 < (int)(uStack00000010 + iStack00000008)) {
            do {
                uVar9 = *puVar10;
                uVar7 = uStack0000000c;
                if (DAT_0071e524 != '\0') {
                    uVar7 = uStack0000000c + 1;
                }
                _cStack00000004 = (int)(short)DAT_00732e58;
                iVar5 = (int)(short)DAT_006d9980;
                if (_cStack00000004 < iVar5) {
                    if ((int)uVar7 <= (int)((unsigned int)iVar5 - (unsigned int)_cStack00000004)) break;
                    uVar7 = (uVar7 - iVar5) + _cStack00000004;
                    cStack00000004 = (char)DAT_00732e58;
                    uVar9 = uVar9 << (((char)DAT_006d9980 - cStack00000004) & 0x1f);
                    _cStack00000004 = iVar5;
                }
                if ((short)DAT_00732a34 <= iStack00000008) break;
                if (iStack00000008 < (short)DAT_00732e8c) {
                    uVar7 = 0;
                }
                if (((int)DAT_0070acc0 < (int)(_cStack00000004 + uVar7)) &&
                    (uVar7 = (unsigned int)(int)DAT_0070acc0 - (unsigned int)_cStack00000004, (int)uVar7 < 1)) break;
                iVar5 = FUN_00461ff0(iStack00000008);
                uVar6 = (unsigned int)iVar5 + (unsigned int)_cStack00000004 * 2u;
                if ((DAT_0052552c != 0x13) && (DAT_00525534 == 0xa0000)) {
                    uVar6 = uVar6 % DAT_0052550c;
                    FUN_004651f0();
                }
                puVar3 = (unsigned short *)(uVar6 + (DAT_00525534 & 0xfffffffe));
                while (uVar7 != 0) {
                    uVar7 = uVar7 - 1;
                    if ((uVar9 & 0x80000000) == 0) {
                        if ((DAT_0071e524 == '\0') || ((uVar8 & 0x80000000) == 0)) {
                            uVar6 = (unsigned int)DAT_00732e50;
                        } else {
                            uVar6 = (unsigned int)DAT_0071b012;
                        }
                    } else {
                        uVar6 = (unsigned int)DAT_006da9fa;
                    }
                    *puVar3 = *(unsigned short *)(DAT_007325e0 + uVar6 * 2u);
                    uVar9 = uVar9 * 2;
                    uVar8 = uVar8 * 2;
                    puVar3 = puVar3 + 1;
                }
                iStack00000008 = (int)((unsigned int)iStack00000008 + 1u);
                uVar8 = *puVar10 >> 1;
                puVar10 = puVar10 + 2;
            } while (iStack00000008 < (int)((unsigned int)(int)(short)DAT_00732e52 + uStack00000010));
        }
        DAT_00732e58 = DAT_00732e58 + bVar1;
        if ((DAT_0070acc0 <= (short)DAT_00732e58) && (DAT_006ffbf0 != 0)) {
            if ((DAT_00730894 != 0) && ((short)DAT_00732e58 < DAT_0070acc0)) {
                DAT_005369d4 = DAT_0070acc0;
                DAT_005369d8 = DAT_00732e58;
                DAT_005369d0 = DAT_006d4a4e + DAT_00732e52;
                DAT_005369dc = DAT_00732e52;
                if ((int)(unsigned int)DAT_00732e58 < (int)(short)DAT_006d9980) {
                    DAT_005369d8 = DAT_006d9980;
                }
                if ((int)(unsigned int)DAT_00732e52 < (int)(short)DAT_00732e8c) {
                    DAT_005369dc = DAT_00732e8c;
                }
                if ((int)(short)DAT_00732a34 < (int)(unsigned int)DAT_005369d0) {
                    DAT_005369d0 = DAT_00732a34;
                }
                if (DAT_005369dc < DAT_005369d0) {
                    FUN_004169b0(0);
                }
            }
            DAT_00732e58 = DAT_006d9980;
            DAT_00732e52 = DAT_00732e52 + bVar2 + 2;
        }
    }
    return;
}
