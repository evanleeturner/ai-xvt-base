/* SPDX-License-Identifier: MIT */
/* z_bop 0046dd80 */
extern int *DAT_009994c4;
extern int DAT_00999548;
extern int DAT_0052702c;
extern int DAT_00527024;
extern unsigned int DAT_005599a8[];
extern unsigned int DAT_005599ac[];
extern unsigned int DAT_005599b0[];
extern unsigned char DAT_005599b4[];
extern unsigned char DAT_005599b5[];
extern unsigned char DAT_005599b6[];
extern unsigned char DAT_005599b7[];
extern unsigned int DAT_005599b8[];

int  __cdecl FUN_0046fad0(void);
void __cdecl FUN_00470060(void);

unsigned char __cdecl FUN_0046dd80(int param_1, unsigned char param_2, unsigned char param_3,
                                   unsigned int *param_4, unsigned int param_5)
{
    unsigned char bVar1;
    int iVar2;
    unsigned int uVar3;
    int iVar4;
    unsigned int *puVar5;
    unsigned int *puVar6;
    unsigned int *puVar7;
    int local_40c;
    int local_404;
    /* frame: local_400 @ +0 (u16), local_3fe @ +2, local_3fd @ +3 (s16),
       local_3fb @ +5 (254 dwords); odd offsets so accessed via memcpy */
    unsigned char local_400[1021];
    unsigned short tmp_w;
    short tmp_s;

    local_404 = 0;
    if (DAT_009994c4 != (int *)0) {
        uVar3 = *param_4;
        if ((uVar3 < 0x3c) || (0x3f < uVar3)) {
            bVar1 = 1;
        } else {
            bVar1 = 0;
        }
        puVar5 = (unsigned int *)(local_400 + 3);
        local_40c = 3;
        tmp_w = (unsigned short)(((unsigned short)uVar3 & 0x7f) | ((param_3 & 0x7f) << 8) | 0x80);
        __builtin_memcpy(local_400, &tmp_w, 2);
        local_400[2] = param_2;
        if ((bVar1) && (iVar2 = FUN_0046fad0(), iVar2 == 0)) {
            tmp_s = (short)((unsigned int)(int)(short)param_5 - 4);
            __builtin_memcpy(local_400 + 3, &tmp_s, 2);
            puVar5 = (unsigned int *)(local_400 + 5);
            local_40c = 5;
        }
        puVar7 = puVar5;
        puVar6 = param_4;
        for (uVar3 = (param_5 - 4) >> 2; puVar6 = puVar6 + 1, uVar3 != 0; uVar3 = uVar3 - 1) {
            __builtin_memcpy(puVar7, puVar6, 4);
            puVar7 = puVar7 + 1;
        }
        for (uVar3 = (param_5 - 4) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
            *(char *)puVar7 = (char)*puVar6;
            puVar6 = (unsigned int *)((int)puVar6 + 1);
            puVar7 = (unsigned int *)((int)puVar7 + 1);
        }
        local_40c = (int)((param_5 - 4) + (unsigned int)local_40c);
        if (bVar1) {
            local_40c = (int)((unsigned int)local_40c + 1);
            *(unsigned char *)((param_5 - 4) + (unsigned int)puVar5) = 0x39;
        }
        if (DAT_00999548 == param_1) {
            if (0x3ff < DAT_0052702c) {
                FUN_00470060();
            }
            if (DAT_0052702c < 0x400) {
                puVar5 = DAT_005599b8 + (int)((unsigned int)DAT_00527024 * 0x84);
                for (uVar3 = param_5 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
                    *puVar5 = *param_4;
                    param_4 = param_4 + 1;
                    puVar5 = puVar5 + 1;
                }
                for (uVar3 = param_5 & 3; iVar2 = DAT_00527024, uVar3 != 0; uVar3 = uVar3 - 1) {
                    *(char *)puVar5 = (char)*param_4;
                    param_4 = (unsigned int *)((int)param_4 + 1);
                    puVar5 = (unsigned int *)((int)puVar5 + 1);
                }
                iVar4 = (int)((unsigned int)DAT_00527024 * 0x210);
                DAT_005599a8[(int)((unsigned int)DAT_00527024 * 0x84)] = (unsigned int)DAT_00999548;
                DAT_005599ac[(int)((unsigned int)iVar2 * 0x84)] = param_5;
                DAT_005599b5[iVar4] = param_2;
                DAT_0052702c = (int)((unsigned int)DAT_0052702c + 1);
                DAT_005599b6[iVar4] = param_3;
                DAT_005599b7[iVar4] = 1;
                DAT_005599b0[(int)((unsigned int)iVar2 * 0x84)] = 0;
                DAT_005599b4[iVar4] = 0;
                DAT_00527024 = (int)((unsigned int)DAT_00527024 + 1);
                if (0x3ff < DAT_00527024) {
                    DAT_00527024 = 0;
                }
            }
        } else {
            local_404 = (*(int (__stdcall **)(int *, int, int, int, void *, int))(*DAT_009994c4 + 0x68))
                            (DAT_009994c4, DAT_00999548, param_1, 0, local_400, local_40c);
        }
        return local_404 == 0;
    }
    return 1;
}
