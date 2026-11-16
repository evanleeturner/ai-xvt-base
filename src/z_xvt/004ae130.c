/* SPDX-License-Identifier: MIT */
/* z_xvt 004ae130 */
/* Function pointers: calling convention not evidenced; assumed __stdcall. */
extern void (__stdcall *DAT_0070a184)(int, short, short, short, int);
extern void (__stdcall *DAT_00703c8c)(int, short, short, short, unsigned int, short);

extern unsigned short DAT_00655184;
extern int DAT_0071ecf4;
extern unsigned char DAT_0071b730[];
extern int DAT_006d4988;
extern unsigned char DAT_00656c90[];
extern unsigned char DAT_00655ab0[];
extern unsigned char DAT_0071f2f0[];
extern unsigned short DAT_00656cd6;
extern unsigned char DAT_0052f120[];
extern unsigned char DAT_0052f123;
extern int DAT_0052552c;
extern unsigned char DAT_0052f128[];
extern unsigned char DAT_0052f150[];
extern unsigned char DAT_00655c58[];

void __stdcall FUN_004ae130(void)
{
    short sVar1;
    short uVar2;
    int iVar3;
    unsigned int uVar4;
    unsigned short uVar5;
    int iVar6;
    unsigned short uVar7;
    unsigned int uVar9;
    unsigned int uVar8;
    short sVar10;
    unsigned short uVar11;
    unsigned short uVar12;
    short sVar13;
    short local_e;
    int t0;
    unsigned short us;

    uVar12 = (unsigned short)(DAT_00655184 + 0x23);

    __builtin_memcpy(&t0, DAT_0071b730 + (unsigned int)(DAT_0071ecf4 * 0x5bd), 4);
    __builtin_memcpy(&t0, (unsigned char *)(unsigned int)((unsigned int)t0 * 0x23u + 0x1fu + (unsigned int)DAT_006d4988), 4);
    __builtin_memcpy(&iVar3, (unsigned char *)(unsigned int)((unsigned int)t0 + 0xa7u), 4);

    if ((*(unsigned char *)(iVar3 + 0x1db) & 0x10) != 0) {
        __builtin_memcpy(&uVar11, (unsigned char *)(iVar3 + 0x235), 2);
        if ((short)uVar11 < 0) {
            uVar11 = 0;
        }
        __builtin_memcpy(&us, (unsigned char *)(iVar3 + 0x1df), 2);
        uVar7 = us & 0x100;
        if (uVar7 == 0) {
            uVar11 = 0;
        }
        uVar5 = (unsigned short)(*(char *)(iVar3 + 0x237) != '\0');
        if (uVar7 == 0) {
            uVar5 = 0;
        }
        iVar3 = DAT_00655184 + 0x5c;
        if (*(unsigned short *)(DAT_00656c90 + iVar3 * 2) != uVar5) {
            *(unsigned short *)(DAT_00656c90 + iVar3 * 2) = uVar5;
            iVar3 = iVar3 * 0xc;
            (*DAT_0070a184)(*(int *)(DAT_0071f2f0 + ((unsigned int)*(unsigned short *)(DAT_00655ab0 + 4 + iVar3) + (unsigned int)uVar5) * 4),
                            *(short *)(DAT_00655ab0 + iVar3),
                            *(short *)(DAT_00655ab0 + 2 + iVar3),
                            *(short *)(DAT_00655ab0 + 6 + iVar3),
                            0);
        }
        uVar4 = (unsigned int)(int)(short)uVar11;
        if ((unsigned int)DAT_00656cd6 != uVar4) {
            local_e = 0;
            DAT_00656cd6 = uVar11;
            do {
                iVar6 = (int)local_e;
                iVar3 = (int)((unsigned int)iVar6 * 5u + 5u);
                uVar9 = (((unsigned int)iVar3 * 5u) >> 8) & 0xffffffu;
                if ((int)((unsigned int)iVar3 * 200u) < (int)uVar4) {
                    uVar8 = (uVar9 << 8) | (unsigned int)DAT_0052f123;
                }
                else if ((short)uVar11 < 0) {
                    uVar8 = (uVar9 << 8) | (unsigned int)DAT_0052f120[0];
                }
                else {
                    uVar7 = uVar11;
                    if (1000 < (short)uVar11) {
                        uVar7 = 1000;
                    }
                    uVar8 = ((uVar9 << 8) | (unsigned int)DAT_0052f120[(short)uVar7 / 0x14d]) & 0xffff00ffu;
                }
                sVar13 = *(short *)(DAT_00655ab0 + (unsigned int)uVar12 * 0xc);
                sVar10 = *(short *)(DAT_00655ab0 + 2 + (unsigned int)uVar12 * 0xc);
                if (DAT_0052552c == 0x101) {
                    sVar1 = (short)((8 - local_e) * 3);
                    sVar13 = (short)(sVar13 + sVar1);
                    sVar10 = (short)(sVar10 + sVar1);
                }
                else if (DAT_0052552c == 0x112) {
                    sVar13 = (short)(sVar13 + *(short *)(DAT_0052f128 + iVar6 * 4));
                    sVar10 = (short)(sVar10 + *(short *)(DAT_0052f128 + 2 + iVar6 * 4));
                }
                else {
                    sVar13 = (short)(sVar13 + *(short *)(DAT_0052f150 + iVar6 * 4));
                    sVar10 = (short)(sVar10 + *(short *)(DAT_0052f150 + 2 + iVar6 * 4));
                }
                if ((unsigned int)DAT_0052f120[0] == (uVar8 & 0xff)) {
                    uVar2 = 0;
                }
                else {
                    uVar2 = *(short *)(DAT_00655c58 + 4 + (unsigned int)DAT_00655184 * 0xc);
                }
                uVar11 = (unsigned short)(uVar11 - 1000);
                (*DAT_00703c8c)(*(int *)(DAT_0071f2f0 +
                                         ((unsigned int)*(unsigned short *)(DAT_00655c58 + (unsigned int)DAT_00655184 * 0xc) + (unsigned int)iVar6) * 4),
                                sVar13, sVar10,
                                *(short *)(DAT_00655c58 + 2 + (unsigned int)DAT_00655184 * 0xc),
                                uVar8 & 0xffff00ffu, uVar2);
                local_e = (short)(local_e + 1);
            } while (local_e < 9);
        }
    }
}
