/* SPDX-License-Identifier: MIT */
/* z_bop 004a7480 */
extern unsigned int DAT_009a1fe8;
extern int DAT_00a08104;
extern unsigned int DAT_00a00870[];
extern unsigned int DAT_00a00874[];
extern int DAT_00521f1c[];
extern int DAT_009ecc34;
extern unsigned short DAT_00a082f4;

unsigned int __cdecl FUN_00425cb0(void);
unsigned int __cdecl FUN_00425d10(void);
void __cdecl FUN_00427a60(short a, short b, short c, short d, int e);
int __cdecl FUN_004adc40(unsigned int a);
int __cdecl FUN_004ae2a0(unsigned int a, unsigned int b);
unsigned int __cdecl FUN_004a76d0(int a, unsigned int b, int c, int d);
unsigned int __cdecl FUN_0042e4d0(unsigned int a, unsigned int b, int c);

void __cdecl FUN_004a7480(unsigned int param_1, short param_2)
{
    unsigned char bVar1;
    unsigned short uVar2;
    short sVar3;
    int iVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    int iVar8;
    unsigned short uStack_18;
    unsigned short uStack_16;
    unsigned char local_10[16];

    uVar2 = (unsigned short)FUN_00425cb0();
    if ((uVar2 < 0x1fff) || (param_2 != 0)) {
        iVar8 = (int)((param_1 & 0xffff) * 0x23 + DAT_009a1fe8);
        __builtin_memcpy(&iVar4, (unsigned char *)iVar8 + 0x1f, 4);
        __builtin_memcpy(&DAT_00a08104, (unsigned char *)iVar4 + 0xa9, 4);
        bVar1 = *(unsigned char *)(iVar8 + 3);
        if (*(char *)(iVar4 + 0x92) != '\0') {
            FUN_00427a60(*(short *)(iVar8 + 0x14), *(short *)(iVar8 + 0x12),
                         *(short *)(iVar8 + 0x10), 0, iVar8);
        }
        uVar6 = (unsigned int)bVar1;
        if (bVar1 < 0x49) {
            uStack_18 = (unsigned short)DAT_00a00870[uVar6 * 0x65];
            uStack_16 = (unsigned short)(DAT_00a00870[uVar6 * 0x65] >> 0x10);
        } else {
            iVar4 = FUN_004adc40(uVar6);
            uStack_18 = (unsigned short)iVar4;
            uStack_16 = (unsigned short)((unsigned int)iVar4 >> 0x10);
        }
        uVar5 = 0;
        uVar2 = 0;
        if (0 < (int)(((unsigned int)uStack_16 << 16) | (unsigned int)uStack_18)) {
            do {
                if (bVar1 < 0x49) {
                    uVar7 = uVar5;
                    if ((int)DAT_00a00870[uVar6 * 0x65] <= (int)uVar5) {
                        uVar7 = DAT_00a00870[uVar6 * 0x65] - 1;
                    }
                    iVar4 = (int)DAT_00a00874[uVar7 + uVar6 * 0x65];
                } else {
                    iVar4 = FUN_004ae2a0(uVar6, uVar5);
                }
                if (iVar4 == 1) {
                    local_10[uVar2] = (unsigned char)uVar5;
                    uVar2 = uVar2 + 1;
                }
            } while ((uVar2 != 0x10) &&
                     (uVar5 = (unsigned int)(unsigned short)((short)uVar5 + 1),
                      (int)uVar5 < (int)(((unsigned int)uStack_16 << 16) | (unsigned int)uStack_18)));
        }
        if (uVar2 == 0) {
            local_10[0] = 0;
            uVar2 = 1;
        }
        if (param_2 != 0) {
            sVar3 = 1;
            uVar2 = DAT_00a082f4;
            do {
                sVar3 = (short)(sVar3 - 1);
                *(unsigned char *)((unsigned int)uVar2 * 0x23 + 3 + DAT_009a1fe8) = 0;
                uVar2 = (unsigned short)(uVar2 + 1);
            } while (sVar3 != 0);
            FUN_004a76d0(iVar8, (unsigned int)local_10[0], DAT_00521f1c[uVar6 * 0x1a], 0);
            FUN_0042e4d0(0x15, param_1 & 0xffff, DAT_009ecc34);
            return;
        }
        uVar5 = FUN_00425cb0();
        if ((*(char *)(local_10[(uVar5 & 0xffff) % (unsigned int)uVar2] + 0x33c + DAT_00a08104) != '\0') &&
            (uVar6 = FUN_004a76d0(iVar8, (unsigned int)local_10[(uVar5 & 0xffff) % (unsigned int)uVar2],
                                  DAT_00521f1c[uVar6 * 0x1a] >> 4, 1),
             (short)uVar6 != -1)) {
            uVar6 = uVar6 & 0xffff;
            iVar8 = DAT_009ecc34;
            uVar5 = FUN_00425d10();
            FUN_0042e4d0((uVar5 & 3) + 0x16, uVar6, iVar8);
        }
    }
    return;
}
