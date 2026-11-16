/* SPDX-License-Identifier: MIT */
/* z_bop 00485900 */
void __cdecl FUN_004525a0(unsigned int param_1, short param_2, char *param_3);
void __cdecl FUN_00451bf0(int param_1, void *param_2);
void __cdecl FUN_00450650(int param_1, int param_2);
void __cdecl FUN_00451560(void);
void __cdecl FUN_00427720(int param_1, int param_2);
void __cdecl FUN_0042e4d0(int param_1, int param_2, int param_3);
unsigned char * __cdecl FUN_0046f720(unsigned char *param_1);

extern int DAT_00523410;
extern unsigned short DAT_00a07cec;
extern int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern unsigned char DAT_009e9670[];
extern unsigned char DAT_009e9680[];
extern unsigned char DAT_009e9684[];
extern unsigned char DAT_009e9685[];
extern unsigned char DAT_009e967a[];
extern unsigned char DAT_009e9a8a[];
extern unsigned char DAT_009e9a84[];
extern unsigned char DAT_009e9a80[];
extern unsigned char DAT_009e9a67[];
extern unsigned char DAT_009e9a73[];
extern unsigned char DAT_009e9a75[];
extern char DAT_009d8ccc[];
extern char DAT_009d8ccd[];
extern char DAT_009d8cce[];
extern char DAT_009d8ccf[];
extern unsigned char DAT_009d6940[2];
extern unsigned short DAT_00a07be0;
extern unsigned short DAT_00a07be2;
extern unsigned char *DAT_009ecc34;
extern char DAT_009a1ed0[];
extern char DAT_009ec4d0[];

void __cdecl FUN_00485900(int param_1, unsigned char *param_2)
{
    char cVar1;
    unsigned char bVar2;
    unsigned char bVar3;
    unsigned char *puVar4;
    unsigned int uVar5;
    unsigned short uVar6;
    int iVar7;
    unsigned int uVar8;
    int iVar9;
    unsigned int uVar10;
    unsigned int tmpI;
    unsigned int tmpJ;
    unsigned short tmpS;
    unsigned int zero32;
    unsigned short zero16;

    if (DAT_00523410 == 0) {
        if ((*(unsigned char *)(param_1 + 0x1e3) & 0x80) == 0) {
            uVar8 = (unsigned int)DAT_00a07cec;
            uVar10 = 0xffff;
            uVar5 = 0xffff;
            if ((int)(unsigned int)DAT_00a07cec < DAT_009a1fe0) {
                __builtin_memcpy(&tmpI, &DAT_009e9670[(unsigned int)param_2 * 0x5bdu], 4);
                iVar9 = (int)((unsigned int)*(unsigned char *)(tmpI * 0x23u + 0x16u + (unsigned int)DAT_009a1fe8) * 0x566u);
                do {
                    if (((DAT_009d8ccd[iVar9] != '\0') &&
                         (iVar7 = (int)(uVar8 * 0x23u + (unsigned int)DAT_009a1fe8), *(char *)((unsigned int)iVar7 + 3u) != '\0')) &&
                        (DAT_009d8ccc[iVar9] == *(char *)((unsigned int)iVar7 + 0x16u))) {
                        uVar5 = uVar8;
                    }
                    if (((DAT_009d8ccf[iVar9] != '\0') &&
                         (iVar7 = (int)(uVar8 * 0x23u + (unsigned int)DAT_009a1fe8), *(char *)((unsigned int)iVar7 + 3u) != '\0')) &&
                        (DAT_009d8cce[iVar9] == *(char *)((unsigned int)iVar7 + 0x16u))) {
                        uVar10 = uVar8;
                    }
                    uVar8 = (unsigned int)(unsigned short)((short)uVar8 + 1);
                } while ((int)uVar8 < DAT_009a1fe0);
            }
            if ((short)uVar5 != -1) {
                if ((short)uVar10 != -1) {
                    FUN_004525a0(uVar5, 0, DAT_009a1ed0);
                    FUN_00451bf0(0, DAT_009a1ed0);
                    FUN_004525a0(uVar10, 0, DAT_009ec4d0);
                    FUN_00451bf0(1, DAT_009ec4d0);
                    FUN_00450650(0x6f, (int)param_2);
                    return;
                }
                if ((short)uVar5 != -1) {
                    FUN_004525a0(uVar5, 0, DAT_009a1ed0);
                    FUN_00451bf0(0, DAT_009a1ed0);
                    FUN_00450650(0x6e, (int)param_2);
                    return;
                }
            }
            if ((short)uVar10 != -1) {
                FUN_004525a0(uVar10, 0, DAT_009a1ed0);
                FUN_00451bf0(0, DAT_009a1ed0);
                FUN_00450650(0x6e, (int)param_2);
            }
        }
        else {
            if (DAT_009d6940[1] != '\0') {
                DAT_009d6940[0] = 1;
                DAT_009e9680[(unsigned int)param_2 * 0x5bdu] = 2;
                return;
            }
            if ((*(unsigned char *)(param_1 + 0x1e5) & 0x80) == 0) {
                DAT_00a07be0 = 99;
                DAT_00a07be2 = 0x57;
                FUN_00450650(0x56, (int)param_2);
                return;
            }
            bVar3 = 0;
            for (uVar6 = DAT_00a07cec; (short)uVar6 < DAT_009a1fe0; uVar6 = uVar6 + 1) {
                iVar9 = (int)((unsigned int)(int)(short)uVar6 * 0x23u + (unsigned int)DAT_009a1fe8);
                cVar1 = *(char *)((unsigned int)iVar9 + 3u);
                if ((cVar1 == '3') || (cVar1 == '\\')) {
                    __builtin_memcpy(&iVar9, (unsigned char *)((unsigned int)iVar9 + 0x1fu), 4);
                    __builtin_memcpy(&tmpS, &DAT_009e967a[(unsigned int)param_2 * 0x5bdu], 2);
                    if (tmpS != (unsigned short)*(unsigned char *)((unsigned int)iVar9 + 0x88u)) {
                        __builtin_memcpy(&tmpI, (unsigned char *)((unsigned int)iVar9 + 0xa9u), 4);
                        __builtin_memcpy(&tmpS, (unsigned char *)(tmpI + 0x1e5u), 2);
                        if (tmpS != 0) {
                            bVar3 = 1;
                        }
                    }
                }
            }
            if (bVar3) {
                FUN_00450650(0x6d, (int)param_2);
                return;
            }
            if (param_2 == DAT_009ecc34) {
                FUN_00451560();
            }
            FUN_00450650(0x6a, (int)param_2);
            iVar9 = (int)((unsigned int)param_2 * 0x5bdu);
            zero32 = 0;
            zero16 = 0;
            DAT_009e9684[iVar9] = 1;
            __builtin_memcpy(&DAT_009e9685[iVar9], &zero32, 4);
            __builtin_memcpy(&DAT_009e9a8a[iVar9], &zero16, 2);
            __builtin_memcpy(&DAT_009e9a84[iVar9], &zero16, 2);
            __builtin_memcpy(&DAT_009e9a80[iVar9], &zero16, 2);
            __builtin_memcpy(&tmpI, &DAT_009e9670[iVar9], 4);
            tmpS = (unsigned short)tmpI;
            __builtin_memcpy(&DAT_009e9a67[iVar9], &tmpS, 2);
            FUN_00427720(0, (int)param_2);
            __builtin_memcpy(&DAT_009e9a73[iVar9], &zero16, 2);
            __builtin_memcpy(&DAT_009e9a75[iVar9], &zero16, 2);
            __builtin_memcpy((unsigned char *)(param_1 + 0xbd), &zero16, 2);
            __builtin_memcpy(&tmpJ, &DAT_009e9670[iVar9], 4);
            cVar1 = *(char *)(tmpJ * 0x23u + 3u + (unsigned int)DAT_009a1fe8);
            if (((cVar1 == '\x01') || (cVar1 == '\x04')) &&
                (bVar2 = *(unsigned char *)(param_1 + 0x29), (bVar2 & 2) == 0)) {
                *(unsigned char *)(param_1 + 0x29) = bVar2 | 2;
                *(unsigned char *)(param_1 + 0x29) = bVar2 | 3;
                FUN_0042e4d0(0x55, -1, (int)param_2);
            }
            if (param_2 != DAT_009ecc34) {
                puVar4 = FUN_0046f720(param_2);
                FUN_00451bf0(0, puVar4);
                FUN_00450650(0x71, (int)DAT_009ecc34);
                return;
            }
        }
    }
    return;
}
