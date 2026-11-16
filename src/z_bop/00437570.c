/* SPDX-License-Identifier: MIT */
/* z_bop 00437570 */
extern int DAT_009a1fe8;
extern unsigned char DAT_009d8c87[];
extern char *DAT_009ec478;
extern char s_RESOURCE_icons640_ico_005235b8[];
extern char s_RESOURCE_mapicons_ico_00523588[];
extern char s_RESOURCE_mapicons_ico_005235a0[];
extern unsigned char DAT_00521240[];
extern unsigned char DAT_005212f8[];
extern unsigned char DAT_005212b0[];
extern unsigned char DAT_00521440[];
extern unsigned char DAT_005214f8[];
extern unsigned char DAT_005214b0[];
extern unsigned char DAT_00521340[];
extern unsigned char DAT_005213f8[];
extern unsigned char DAT_005213b0[];
extern int DAT_009a8e38;
extern short DAT_009a6fe0;
extern short DAT_00a0813c;
extern short DAT_009d8c00;
extern short DAT_00a07ce4;
extern int DAT_009ecc4c;
extern void (__cdecl *DAT_009d80c4)(void);

short __cdecl FUN_00427570(unsigned int param_1, unsigned int param_2);
short __cdecl FUN_00427590(unsigned int param_1, unsigned int param_2);
void __cdecl FUN_004aba50(void);
void __cdecl FUN_0040ece0(unsigned char *param_1, unsigned int param_2, unsigned int param_3, unsigned int param_4, unsigned int param_5);
void __cdecl FUN_004abe50(void);

void __cdecl FUN_00437570(int param_1, unsigned int param_2, unsigned int param_3, unsigned int param_4)
{
    unsigned char uVar1;
    short sVar2;
    short sVar3;
    int iVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    unsigned short uVar7;
    unsigned short uVar8;
    unsigned int local_8;
    unsigned int local_4;
    unsigned char *pEntry;
    unsigned int pExtra;

    pEntry = (unsigned char *)((unsigned int)param_1 * 0x23u + (unsigned int)DAT_009a1fe8);
    local_8 = (unsigned int)pEntry[3];
    __builtin_memcpy(&pExtra, pEntry + 0x1f, 4);
    if (pExtra == 0) {
        uVar1 = DAT_009d8c87[(unsigned int)pEntry[0x16] * 0x566u];
    } else {
        uVar1 = *(unsigned char *)(pExtra + 0x88);
    }
    local_4 = 0;
    switch (uVar1) {
    case 0:
        iVar4 = 0;
        break;
    case 1:
    case 4:
        iVar4 = 2;
        break;
    case 2:
        iVar4 = 3;
        break;
    case 3:
        local_4 = 1;
        iVar4 = 2;
        break;
    default:
        iVar4 = 1;
    }
    sVar2 = FUN_00427570(param_2, param_4);
    sVar3 = FUN_00427590(param_3, param_4);
    if (DAT_009ec478 == s_RESOURCE_icons640_ico_005235b8) {
        uVar6 = 0x13;
        if (local_8 < 0x6a) {
            uVar6 = (unsigned int)DAT_00521240[local_8];
        }
        local_8 = (unsigned int)DAT_005212f8[uVar6];
        uVar5 = (unsigned int)DAT_005212b0[uVar6];
    } else if (DAT_009ec478 == s_RESOURCE_mapicons_ico_00523588) {
        uVar6 = 0x13;
        if (local_8 < 0x6a) {
            uVar6 = (unsigned int)DAT_00521440[local_8];
        }
        local_8 = (unsigned int)DAT_005214f8[uVar6];
        uVar5 = (unsigned int)DAT_005214b0[uVar6];
    } else {
        uVar5 = local_4;
        uVar6 = local_4;
        if (DAT_009ec478 == s_RESOURCE_mapicons_ico_005235a0) {
            uVar6 = 0x13;
            if (local_8 < 0x6a) {
                uVar6 = (unsigned int)DAT_00521340[local_8];
            }
            local_8 = (unsigned int)DAT_005213f8[uVar6];
            uVar5 = (unsigned int)DAT_005213b0[uVar6];
        }
    }
    iVar4 = (int)((unsigned int)iVar4 * (unsigned int)DAT_009a8e38);
    uVar8 = (unsigned short)((unsigned int)(int)((int)sVar2 + (int)DAT_009a6fe0) - (unsigned int)(int)(short)(uVar5 / 2));
    uVar7 = (unsigned short)((unsigned int)(int)((int)DAT_00a0813c + (int)sVar3) - (unsigned int)(int)(short)(local_8 / 2));
    if ((((DAT_009a6fe0 <= (short)uVar8) && ((short)((unsigned short)((short)uVar5 + uVar8)) < DAT_009d8c00)) &&
         (DAT_00a0813c <= (short)uVar7)) && ((short)((unsigned short)(uVar7 + (short)local_8)) < DAT_00a07ce4)) {
        FUN_004aba50();
        if (local_4 == 0) {
            (*DAT_009d80c4)();
        } else {
            FUN_0040ece0(*(unsigned char **)((unsigned int)DAT_009ecc4c +
                                              (uVar6 + (unsigned int)(((int)((unsigned int)iVar4 + ((unsigned int)(iVar4 >> 0x1f) & 3u))) >> 2)) * 4u),
                         uVar8, uVar7, 0, 0);
        }
        FUN_004abe50();
    }
    return;
}
