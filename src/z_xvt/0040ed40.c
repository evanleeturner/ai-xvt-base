/* SPDX-License-Identifier: MIT */
/* z_xvt 0040ed40 */
/* Unresolved: the callee's exact return/argument types are inferred from the decompile.
   The second call passes CONCAT22(<undefined upper half of ESI>, param_2); the upper 16 bits
   are not recoverable from the evidence. I pass (uint)param_2 and assume the callee uses only
   the low word. */
extern unsigned char *DAT_006d4988;
extern unsigned char DAT_0052686c[];

int __cdecl FUN_0040efa0(unsigned int param_1);

#define LD32(p, v) __builtin_memcpy(&(v), (p), 4)
#define ST32(p, v) __builtin_memcpy((p), &(v), 4)
#define LD16(p, v) __builtin_memcpy(&(v), (p), 2)
#define ST16(p, v) __builtin_memcpy((p), &(v), 2)

void __cdecl FUN_0040ed40(unsigned char *param_1, unsigned short param_2, unsigned int param_3)
{
    unsigned char bVar1;
    int iVar2;
    int iVar5;
    int iVar9;
    unsigned char *iVar4;
    unsigned char *iVar3;
    unsigned int uVar6;
    unsigned int uVar10;
    unsigned char *pbVar7;
    unsigned char *pbVar8;
    int a, b, t32;
    short t16;
    unsigned short us;
    short s3;

    iVar4 = DAT_006d4988 + (unsigned int)param_2 * 0x23u;
    iVar3 = DAT_006d4988 + (param_3 & 0xffff) * 0x23u;

    LD32(iVar4 + 0xc, a);
    LD32(iVar3 + 0xc, b);
    iVar2 = (int)((unsigned int)a - (unsigned int)b);
    LD32(iVar4 + 8, a);
    LD32(iVar3 + 8, b);
    iVar9 = (int)((unsigned int)a - (unsigned int)b);
    LD32(iVar4 + 4, a);
    LD32(iVar3 + 4, b);
    iVar5 = (int)((unsigned int)a - (unsigned int)b);

    if (iVar5 < 0) {
        iVar5 = (int)(0u - (unsigned int)iVar5);
    }
    if (iVar9 < 0) {
        iVar9 = (int)(0u - (unsigned int)iVar9);
    }
    if (iVar2 < 0) {
        iVar2 = (int)(0u - (unsigned int)iVar2);
    }

    if ((iVar9 < iVar5) && (iVar2 < iVar5)) {
        iVar5 = (int)((unsigned int)(iVar9 >> 2) + (unsigned int)(iVar2 >> 2) + (unsigned int)iVar5);
    }
    else if ((iVar5 < iVar9) && (iVar2 < iVar9)) {
        iVar5 = (int)((unsigned int)iVar9 + (unsigned int)(iVar5 >> 2) + (unsigned int)(iVar2 >> 2));
    }
    else {
        iVar5 = (int)((unsigned int)(iVar5 >> 2) + (unsigned int)(iVar9 >> 2) + (unsigned int)iVar2);
    }

    LD16(DAT_0052686c + (unsigned int)iVar3[3] * 0x16, us);
    iVar5 = (int)((unsigned int)iVar5 - (unsigned int)us);
    LD16(DAT_0052686c + (unsigned int)iVar4[3] * 0x16, us);
    iVar5 = (int)((unsigned int)iVar5 - (unsigned int)us);

    if (iVar5 < 0) {
        iVar2 = 0;
    }
    else {
        iVar2 = FUN_0040efa0(param_3);
        a = FUN_0040efa0((unsigned int)param_2);
        iVar2 = (int)((unsigned int)iVar2 + (unsigned int)a) >> 8;
        if (iVar2 == 0) {
            return;
        }
        iVar2 = (int)((unsigned int)(iVar5 >> 8) * 0x33dbu) / iVar2;
    }

    s3 = (short)param_3;
    uVar10 = 0;
    bVar1 = *param_1;
    if (bVar1 != 0) {
        pbVar7 = param_1 + 1;
        pbVar8 = param_1 + 0x41;
        do {
            LD16(pbVar8, t16);
            if (t16 == s3) {
                uVar6 = uVar10 + 1;
                ST32(param_1 + uVar10 * 4 + 1, iVar2);
                if ((int)(unsigned int)*param_1 <= (int)uVar6) goto LAB_0040eeeb;
                pbVar7 = param_1 + uVar6 * 2 + 0x41;
                pbVar8 = param_1 + uVar6 * 4 + 1;
                goto LAB_0040eebe;
            }
            LD32(pbVar7, t32);
            if (iVar2 < t32) break;
            pbVar7 = pbVar7 + 4;
            pbVar8 = pbVar8 + 2;
            uVar10 = uVar10 + 1;
        } while ((int)uVar10 < (int)(unsigned int)bVar1);
    }
    uVar6 = (unsigned int)bVar1;
    if (uVar10 != uVar6) {
        if (uVar6 == 0x10) {
            LD32(param_1 + 0x3d, a);
            LD32(param_1 + 0x61, b);
            if (a < b) {
                ST32(param_1 + 0x61, a);
            }
            uVar6 = 0xf;
            *param_1 = (unsigned char)(bVar1 - 1);
        }
        if ((int)uVar10 < (int)uVar6) {
            pbVar7 = param_1 + uVar6 * 4 + 1;
            iVar5 = (int)(uVar6 - uVar10);
            pbVar8 = param_1 + uVar6 * 2 + 0x41;
            do {
                LD32(pbVar7 - 4, t32);
                ST32(pbVar7, t32);
                pbVar7 = pbVar7 - 4;
                iVar5 = iVar5 - 1;
                LD16(pbVar8 - 2, t16);
                ST16(pbVar8, t16);
                pbVar8 = pbVar8 - 2;
            } while (iVar5 != 0);
        }
        ST32(param_1 + uVar10 * 4 + 1, iVar2);
        ST16(param_1 + uVar10 * 2 + 0x41, s3);
        *param_1 = (unsigned char)(*param_1 + 1);
        return;
    }
    if (bVar1 == 0x10) {
        LD32(param_1 + 0x61, t32);
        if (t32 <= iVar2) {
            return;
        }
        ST32(param_1 + 0x61, iVar2);
        return;
    }
    ST32(param_1 + uVar10 * 4 + 1, iVar2);
    ST16(param_1 + uVar10 * 2 + 0x41, s3);
    *param_1 = (unsigned char)(*param_1 + 1);
    return;

    while (1) {
        LD32(pbVar8, t32);
        ST32(pbVar8 - 4, t32);
        LD16(pbVar7, t16);
        ST16(pbVar7 - 2, t16);
        uVar6 = uVar6 + 1;
        ST32(pbVar8, iVar2);
        ST16(pbVar7, s3);
        pbVar7 = pbVar7 + 2;
        pbVar8 = pbVar8 + 4;
        if ((unsigned int)*param_1 == uVar6) break;
LAB_0040eebe:
        LD32(pbVar8, t32);
        if (iVar2 <= t32) break;
    }
    uVar10 = uVar6 - 1;
LAB_0040eeeb:
    if (0 < (int)uVar10) {
        iVar5 = (int)(uVar10 - 1);
        pbVar7 = param_1 + uVar10 * 2 + 0x3f;
        pbVar8 = param_1 + uVar10 * 4 - 3;
        for (;;) {
            LD32(pbVar8, t32);
            if (!(iVar2 < t32)) break;
            ST32(pbVar8 + 4, t32);
            LD16(pbVar7, t16);
            ST16(pbVar7 + 2, t16);
            iVar5 = iVar5 - 1;
            ST32(pbVar8, iVar2);
            ST16(pbVar7, s3);
            pbVar7 = pbVar7 - 2;
            pbVar8 = pbVar8 - 4;
            if (iVar5 < 0) {
                return;
            }
        }
    }
    return;
}
