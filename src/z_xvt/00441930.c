/* SPDX-License-Identifier: MIT */
/* z_xvt 00441930 */
extern unsigned short DAT_00732a3c;
extern int DAT_00732a38;
extern unsigned int DAT_006d4988;
extern int DAT_006d4980;
extern int DAT_00703c88;
extern unsigned char DAT_0071b730[];
extern char DAT_0071b743[];
extern unsigned char DAT_0052687a[];
extern unsigned short DAT_006d9d40;
extern unsigned int DAT_006d499c;

unsigned long long __cdecl FUN_00441fc0(unsigned int, unsigned int, unsigned int, int, unsigned int);
unsigned int __cdecl FUN_004a1d70(unsigned int, unsigned int);

unsigned int __cdecl FUN_00441930(unsigned int param_1)
{
    unsigned char bVar2;
    unsigned int uVar3;
    unsigned short uVar4;
    unsigned int uVar5;
    unsigned short uVar6;
    unsigned int unaff_ESI = 0;
    unsigned int uVar7;
    unsigned long long uVar8;
    unsigned short local_4;
    unsigned short local_2;
    unsigned int tmp;
    int iVar;
    unsigned int p;
    unsigned int q;
    unsigned short sv;

    uVar7 = ((unaff_ESI >> 16) << 16) | (unsigned int)DAT_00732a3c;
    uVar4 = 0xffff;
    local_4 = 0xffff;
    uVar5 = 0xffffffffU;
    local_2 = 0xffff;
    if ((int)(unsigned int)DAT_00732a3c < DAT_00732a38) {
        do {
            bVar2 = *(unsigned char *)((uVar7 & 0xffff) * 0x23U + 3U + DAT_006d4988);
            uVar6 = (unsigned short)uVar7;
            if (bVar2 != 0) {
                __builtin_memcpy(&tmp, &DAT_0071b730[param_1 * 0x5bdU], 4);
                if (tmp != (uVar7 & 0xffff)) {
                    uVar3 = (unsigned int)bVar2;
                    if ((DAT_0052687a[uVar3 * 0x16U] & 1) != 0) {
                        uVar8 = FUN_00441fc0(uVar3 * 0xbU, uVar3 * 5U, uVar7, 1, param_1);
                        if ((short)uVar8 == 0) {
                            if (DAT_006d9d40 < uVar4) {
                                local_4 = uVar6;
                                uVar4 = DAT_006d9d40;
                            }
                        }
                        else if (DAT_006d499c < uVar5) {
                            uVar5 = DAT_006d499c;
                            local_2 = uVar6;
                        }
                    }
                }
            }
            uVar7 = (uVar7 & 0xffff0000U) | (unsigned int)(unsigned short)(uVar6 + 1);
        } while ((int)(unsigned int)(unsigned short)(uVar6 + 1) < DAT_00732a38);
    }
    uVar7 = (uVar7 & 0xffff0000U) | (unsigned int)(unsigned short)DAT_00732a38;
    if ((int)((unsigned int)DAT_00732a38 & 0xffff) < (int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38)) {
        do {
            bVar2 = *(unsigned char *)((uVar7 & 0xffff) * 0x23U + 3U + DAT_006d4988);
            uVar6 = (unsigned short)uVar7;
            if (bVar2 != 0) {
                uVar3 = (unsigned int)bVar2;
                if ((DAT_0052687a[uVar3 * 0x16U] & 1) != 0) {
                    uVar8 = FUN_00441fc0(uVar3 * 0xbU, uVar3 * 5U, uVar7, 1, param_1);
                    if ((short)uVar8 == 0) {
                        if (DAT_006d9d40 < uVar4) {
                            local_4 = uVar6;
                            uVar4 = DAT_006d9d40;
                        }
                    }
                    else if (DAT_006d499c < uVar5) {
                        uVar5 = DAT_006d499c;
                        local_2 = uVar6;
                    }
                }
            }
            uVar7 = (uVar7 & 0xffff0000U) | (unsigned int)(unsigned short)(uVar6 + 1);
        } while ((int)(unsigned int)(unsigned short)(uVar6 + 1) < (int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38));
    }
    if (local_2 == 0xffff) {
        if (DAT_0071b743[param_1 * 0x5bdU] == '\0') {
            if (0x31 < uVar4) goto LAB_00441aed;
        }
        else if (0xf9 < uVar4) goto LAB_00441aed;
        local_2 = local_4;
    }
LAB_00441aed:
    uVar5 = param_1 * 0x16fU;
    if (DAT_0071b743[param_1 * 0x5bdU] == '\0') {
        if (local_2 == 0xffff) goto clear;
        uVar5 = (unsigned int)DAT_006d4980;
        if (DAT_006d4980 <= (int)(unsigned int)local_2) goto clear;
        uVar5 = (unsigned int)local_2 * 0x23U + DAT_006d4988;
        __builtin_memcpy(&iVar, (unsigned char *)uVar5 + 0x1b, 4);
        if (iVar == -1) goto clear;
        __builtin_memcpy(&p, (unsigned char *)uVar5 + 0x1f, 4);
        __builtin_memcpy(&q, (unsigned char *)(p + 0xa7U), 4);
        uVar5 = q;
        if (uVar5 == 0) goto clear;
        if ((*(unsigned char *)(uVar5 + 0x1e0U) & 1) == 0) goto clear;
        if (*(char *)(uVar5 + 0x237U) == '\0') goto clear;
        if (*(char *)(uVar5 + 0x233U) != '\x03') goto clear;
        __builtin_memcpy(&sv, (unsigned char *)(uVar5 + 0x238U), 2);
        uVar5 = (uVar5 & 0xffffff00U) | 1U;
        if (sv == 0) goto clear;
        goto check;
clear:
        uVar5 = uVar5 & 0xffffff00U;
check:
        if ((char)uVar5 == '\x01') {
            local_2 = 0xffff;
            uVar5 = FUN_004a1d70(0x100, param_1);
        }
    }
    return ((uVar5 >> 16) << 16) | (unsigned int)local_2;
}
