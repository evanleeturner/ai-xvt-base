/* SPDX-License-Identifier: MIT */
/* z_xvt 004351e0 */
extern int DAT_006d4988;
extern unsigned char DAT_0070ad47[];
extern const char *DAT_0071e538;
extern char s_RESOURCE_icons640_ico_005256f8[];
extern char s_RESOURCE_mapicons_ico_005256c8[];
extern char s_RESOURCE_mapicons_ico_005256e0[];
extern unsigned char DAT_005251d8[];
extern unsigned char DAT_00525288[];
extern unsigned char DAT_00525248[];
extern unsigned char DAT_005253c8[];
extern unsigned char DAT_00525480[];
extern unsigned char DAT_00525438[];
extern unsigned char DAT_005252c8[];
extern unsigned char DAT_00525380[];
extern unsigned char DAT_00525338[];
extern int DAT_006db7b8;
extern short DAT_006d9980;
extern short DAT_0070acc0;
extern short DAT_00732e8c;
extern short DAT_00732a34;
extern int DAT_0071ed08;
extern void (__cdecl *DAT_0070a184)(void);

int __cdecl FUN_0040ca90(unsigned int a, unsigned int b);
int __cdecl FUN_0040cad0(unsigned int a, unsigned int b);
void __cdecl FUN_004a6a80(void);
void __cdecl FUN_00462d30(unsigned char *p, unsigned int x, unsigned int y, int a, int b);
void __cdecl FUN_004a6f90(void);

void __cdecl FUN_004351e0(int param_1, unsigned int param_2, unsigned int param_3, unsigned int param_4)
{
    unsigned char uVar1;
    int iVar2;
    int iVar3;
    int iVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    unsigned short uVar7;
    unsigned short uVar8;
    unsigned int local_8;
    unsigned int local_4;
    unsigned char *rec;
    int ptr1f;

    rec = (unsigned char *)(unsigned int)((unsigned int)param_1 * 0x23u + (unsigned int)DAT_006d4988);
    local_8 = (unsigned int)rec[3];
    __builtin_memcpy(&ptr1f, rec + 0x1f, sizeof(ptr1f));
    if (ptr1f == 0) {
        uVar1 = DAT_0070ad47[(unsigned int)rec[0x16] * 0x566u];
    } else {
        uVar1 = *(unsigned char *)((unsigned int)ptr1f + 0x86);
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
    iVar2 = FUN_0040ca90(param_2, param_4);
    iVar3 = FUN_0040cad0(param_3, param_4);
    if (DAT_0071e538 == s_RESOURCE_icons640_ico_005256f8) {
        uVar6 = 0x13;
        if (local_8 < 0x6a) {
            uVar6 = (unsigned int)DAT_005251d8[local_8];
        }
        local_8 = (unsigned int)DAT_00525288[uVar6];
        uVar5 = (unsigned int)DAT_00525248[uVar6];
    } else if (DAT_0071e538 == s_RESOURCE_mapicons_ico_005256c8) {
        uVar6 = 0x13;
        if (local_8 < 0x6a) {
            uVar6 = (unsigned int)DAT_005253c8[local_8];
        }
        local_8 = (unsigned int)DAT_00525480[uVar6];
        uVar5 = (unsigned int)DAT_00525438[uVar6];
    } else {
        uVar5 = local_4;
        uVar6 = local_4;
        if (DAT_0071e538 == s_RESOURCE_mapicons_ico_005256e0) {
            uVar6 = 0x13;
            if (local_8 < 0x6a) {
                uVar6 = (unsigned int)DAT_005252c8[local_8];
            }
            local_8 = (unsigned int)DAT_00525380[uVar6];
            uVar5 = (unsigned int)DAT_00525338[uVar6];
        }
    }
    iVar4 = (int)((unsigned int)DAT_006db7b8 * (unsigned int)iVar4);
    uVar8 = (unsigned short)(((short)iVar2 + DAT_006d9980) - (short)(uVar5 / 2));
    uVar7 = (unsigned short)((DAT_00732e8c + (short)iVar3) - (short)(local_8 / 2));
    if ((((DAT_006d9980 <= (short)uVar8) && ((short)((short)uVar5 + uVar8) < DAT_0070acc0)) &&
         (DAT_00732e8c <= (short)uVar7)) && ((short)(uVar7 + (short)local_8) < DAT_00732a34)) {
        FUN_004a6a80();
        if (local_4 == 0) {
            (*DAT_0070a184)();
        } else {
            unsigned int adj = (unsigned int)iVar4 + ((unsigned int)(iVar4 >> 0x1f) & 3U);
            unsigned int idx = uVar6 + (unsigned int)((int)adj >> 2);
            FUN_00462d30(*(unsigned char **)((unsigned int)DAT_0071ed08 + idx * 4u),
                         uVar8, uVar7, 0, 0);
        }
        FUN_004a6f90();
    }
    return;
}
