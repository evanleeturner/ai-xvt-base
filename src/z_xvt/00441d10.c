/* SPDX-License-Identifier: MIT */
/* z_xvt 00441d10 */
extern unsigned int DAT_00703c88;
extern unsigned int DAT_00732a38;
extern unsigned char DAT_0071b730[];
extern unsigned char DAT_0071b73c[];
extern int DAT_0071b704;
extern int DAT_00732904;
extern int DAT_006d4988;
extern int DAT_006d4980;
extern unsigned char DAT_0052687a[];
extern unsigned char DAT_0070ad48[];
extern char DAT_0072f508[];
extern int DAT_00732e54;

unsigned int __cdecl FUN_00441d10(unsigned short param_1, short param_2, int param_3,
                                  unsigned int param_4, unsigned int param_5)
{
    short sVar1;
    short sVar4;
    unsigned short uVar5;
    unsigned short uVar7;
    unsigned int uVar6;
    unsigned int uVar9;
    unsigned int uVar10;
    unsigned int iVar3;
    unsigned char *iVar8;
    unsigned char *ent;
    unsigned char *q;
    unsigned char bVar2;
    int bVar11;
    short local_6;
    unsigned int v32;
    unsigned int f1b;
    unsigned short v16;
    unsigned short s238;

    sVar4 = (short)((short)DAT_00703c88 + (short)DAT_00732a38);
    uVar6 = (((DAT_00703c88 >> 0x10) & 0xffffu) << 0x10) | (unsigned int)param_1;
    local_6 = sVar4;
LAB_00441d36:
    do {
        do {
            sVar1 = (short)(local_6 + -1);
            uVar7 = (unsigned short)(uVar6 >> 0x10);
            if (local_6 == 0) {
                return ((unsigned int)uVar7 << 0x10) | 0xffffu;
            }
            uVar5 = (unsigned short)((short)uVar6 + param_2);
            uVar6 = ((unsigned int)uVar7 << 0x10) | (unsigned int)uVar5;
            if (uVar5 < 0x8000) {
                if (DAT_00703c88 + DAT_00732a38 == (unsigned int)uVar5) {
                    uVar6 = (unsigned int)uVar7 << 0x10;
                }
            } else {
                uVar6 = ((unsigned int)uVar7 << 0x10) | (unsigned int)(unsigned short)(sVar4 + -1);
            }
            uVar10 = uVar6 & 0xffffu;
            iVar3 = (unsigned int)param_3 * 0x5bdu;
            local_6 = sVar1;
            __builtin_memcpy(&v32, DAT_0071b730 + iVar3, 4);
        } while (v32 == uVar10 ||
                 (((param_5 & 4) != 0 && DAT_0071b704 <= (int)uVar10 && (int)uVar10 < DAT_00732904) ||
                  ((param_5 & 2) != 0 && DAT_00732904 <= (int)uVar10)));
        iVar8 = (unsigned char *)(uVar10 * 0x23u + (unsigned int)DAT_006d4988);
    } while (iVar8[3] == 0 ||
             ((DAT_0052687a[(unsigned int)iVar8[3] * 0x16u] & 1) == 0 ||
              ((param_5 & 1) != 0 && iVar8[2] == 8)));

    {
        unsigned int e;
        __builtin_memcpy(&e, iVar8 + 0x1f, 4);
        ent = (unsigned char *)e;
    }
    __builtin_memcpy(&f1b, iVar8 + 0x1b, 4);
    __builtin_memcpy(&v16, DAT_0071b73c + iVar3, 2);

    if (ent == 0) {
        bVar2 = DAT_0070ad48[(unsigned int)iVar8[0x16] * 0x566u];
    } else {
        bVar2 = ent[0x87];
    }
    uVar9 = (unsigned int)bVar2;

    switch (param_4) {
    case 1:
        if ((unsigned int)v16 != uVar9) goto LAB_00441d36;
        break;
    case 2:
        if (uVar9 == (unsigned int)v16) {
            bVar11 = 0;
        } else {
            bVar11 = DAT_0072f508[uVar9 + (unsigned int)v16 * 0x1e5u] == '\0';
        }
        if (bVar11) goto LAB_00441d36;
        break;
    case 3:
        if (uVar9 == (unsigned int)v16) {
            bVar11 = 0;
        } else {
            bVar11 = DAT_0072f508[uVar9 + (unsigned int)v16 * 0x1e5u] == '\0';
        }
        if (!bVar11) goto LAB_00441d36;
        break;
    case 4:
        if (f1b == 0xffffffffu) goto LAB_00441d36;
    }

    if (ent == 0) {
        return uVar6;
    }
    if (iVar8[2] != 0x0d) {
        bVar11 = 0;
        if ((short)uVar6 != -1 && DAT_006d4980 > (int)uVar10 && f1b != 0xffffffffu) {
            unsigned int qp;
            __builtin_memcpy(&qp, ent + 0xa7, 4);
            q = (unsigned char *)qp;
            if (q != 0 && (q[0x1e0] & 1) != 0 && q[0x237] != 0 && q[0x233] == 3) {
                bVar11 = 1;
                __builtin_memcpy(&s238, q + 0x238, 2);
                if (s238 == 0) {
                    bVar11 = 0;
                }
            }
        }
        if (!bVar11) {
            if (*(char *)ent != '\0') {
                return uVar6;
            }
            __builtin_memcpy(&DAT_00732e54, ent + 0xa7, 4);
            if (*(char *)(DAT_00732e54 + 7) != 3 && *(char *)(DAT_00732e54 + 7) != 4) {
                return uVar6;
            }
        }
    }
    goto LAB_00441d36;
}
