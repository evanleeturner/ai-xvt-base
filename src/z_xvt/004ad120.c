/* SPDX-License-Identifier: MIT */
/* z_xvt 004ad120 */
extern int DAT_0071ecf4;
extern unsigned int DAT_006d4988;
extern unsigned char DAT_0071b730[];
extern unsigned char DAT_0052b0b4[];
extern unsigned char DAT_0052b0b5[];
extern unsigned char DAT_0052b0b6[];
extern unsigned char DAT_0052b0b7[];
extern unsigned char DAT_0052b0b8[];
extern unsigned char DAT_0052b0b9[];

unsigned int FUN_004a7aa0(unsigned int param_1);
void FUN_004ad3f0(unsigned int param_1, unsigned short param_2, unsigned int param_3);

void __stdcall FUN_004ad120(void)
{
    unsigned char bVar1;
    unsigned int iVar2;
    unsigned int uVar3;
    unsigned int iVar4;
    unsigned short uVar5;
    unsigned int uVar6;
    unsigned int tmp;
    unsigned int flagsPtr;

    __builtin_memcpy(&tmp, DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bdU, 4);
    iVar4 = tmp * 0x23U + DAT_006d4988;
    __builtin_memcpy(&iVar2, (unsigned char *)(iVar4 + 0x1f), 4);
    __builtin_memcpy(&flagsPtr, (unsigned char *)(iVar2 + 0xa7), 4);
    if ((*(unsigned char *)(flagsPtr + 0x1db) & 8) != 0) {
        uVar3 = FUN_004a7aa0((((iVar2 & 0xffffff00U) | *(unsigned char *)(iVar4 + 3)) & 0xffff00ffU));
        bVar1 = DAT_0052b0b8[(uVar3 & 0xffff) * 0xef];

        __builtin_memcpy(&tmp, DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bdU, 4);
        uVar3 = FUN_004a7aa0((((DAT_006d4988 & 0xffffff00U) |
                               *(unsigned char *)(tmp * 0x23U + 3U + DAT_006d4988)) & 0xffff00ffU));
        if ((unsigned short)((unsigned short)DAT_0052b0b9[(uVar3 & 0xffff) * 0xef] + (unsigned short)bVar1) != 0) {
            uVar6 = 0;
            uVar5 = 0;
            __builtin_memcpy(&tmp, DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bdU, 4);
            uVar3 = FUN_004a7aa0((((DAT_006d4988 & 0xffffff00U) |
                                   *(unsigned char *)(tmp * 0x23U + 3U + DAT_006d4988)) & 0xffff00ffU));
            iVar2 = (uVar3 & 0xffff) * 0xef;
            FUN_004ad3f0(((iVar2 & 0xffffff00U) | DAT_0052b0b4[iVar2]) & 0xffff00ffU, uVar5, uVar6);

            uVar6 = 0;
            uVar5 = 1;
            __builtin_memcpy(&tmp, DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bdU, 4);
            uVar3 = FUN_004a7aa0((((DAT_006d4988 & 0xffffff00U) |
                                   *(unsigned char *)(tmp * 0x23U + 3U + DAT_006d4988)) & 0xffff00ffU));
            iVar2 = (uVar3 & 0xffff) * 0xef;
            FUN_004ad3f0(((iVar2 & 0xffffff00U) | DAT_0052b0b6[iVar2]) & 0xffff00ffU, uVar5, uVar6);

            __builtin_memcpy(&tmp, DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bdU, 4);
            uVar3 = FUN_004a7aa0((((DAT_006d4988 & 0xffffff00U) |
                                   *(unsigned char *)(tmp * 0x23U + 3U + DAT_006d4988)) & 0xffff00ffU));
            uVar6 = FUN_004a7aa0(0xc);
            if ((short)uVar3 == (short)uVar6) {
                uVar6 = 1;
                uVar5 = 2;
                __builtin_memcpy(&tmp, DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bdU, 4);
                uVar3 = FUN_004a7aa0((((DAT_006d4988 & 0xffffff00U) |
                                       *(unsigned char *)(tmp * 0x23U + 3U + DAT_006d4988)) & 0xffff00ffU));
                iVar2 = (uVar3 & 0xffff) * 0xef;
                FUN_004ad3f0(((iVar2 & 0xffffff00U) | DAT_0052b0b5[iVar2]) & 0xffff00ffU, uVar5, uVar6);

                uVar6 = 1;
                uVar5 = 3;
                __builtin_memcpy(&tmp, DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bdU, 4);
                uVar3 = FUN_004a7aa0((((DAT_006d4988 & 0xffffff00U) |
                                       *(unsigned char *)(tmp * 0x23U + 3U + DAT_006d4988)) & 0xffff00ffU));
                iVar2 = (uVar3 & 0xffff) * 0xef;
                FUN_004ad3f0(((iVar2 & 0xffffff00U) | DAT_0052b0b7[iVar2]) & 0xffff00ffU, uVar5, uVar6);
            }
        }
    }
    return;
}
