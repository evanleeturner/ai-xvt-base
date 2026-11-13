/* SPDX-License-Identifier: MIT */
/* z_xvt 004213b0 */
extern unsigned short DAT_006ccad0;
extern unsigned short DAT_006ccad2;
extern void *DAT_006d4988;
extern void *DAT_006db71c;
extern short DAT_006db6c0;
extern void FUN_0040f050(unsigned short);

void __cdecl FUN_004213b0(int param_1)
{
    int iVar1;
    unsigned short *puVar2;
    int iVar3;
    unsigned int *puVar4;
    int iVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    unsigned short uVar8;
    unsigned short uVar9;
    unsigned short uStack_12;
    int tmp_int;
    unsigned int tmp_ptr;
    unsigned short tmp_short;

    __builtin_memcpy(&tmp_int, (void *)((char *)param_1 + 0x432), 4);
    iVar3 = tmp_int;
    uStack_12 = 0;
    __builtin_memcpy(&tmp_ptr, (void *)((char *)iVar3 + 0x1f), 4);
    __builtin_memcpy(&puVar4, (void *)((char *)tmp_ptr + 0xab), 4);
    do {
        uVar9 = DAT_006ccad0;
        uVar6 = (unsigned int)uStack_12;
        uVar8 = DAT_006ccad0;
        if (*(char *)((char *)param_1 + 0x344 + uVar6 * 4) == '\x02') {
            while ((uVar7 = (unsigned int)uVar8, uVar8 < DAT_006ccad2 &&
                   (*(char *)((char *)DAT_006d4988 + uVar7 * 0x23 + 3) != '\0'))) {
                uVar8 = uVar8 + 1;
            }
            if (DAT_006ccad2 <= uVar8) {
                return;
            }
            FUN_0040f050(uVar8);
            iVar5 = uVar7 * 0x23;
            puVar2 = (unsigned short *)((char *)DAT_006d4988 + iVar5);
            
            unsigned char *p_dest;
            __builtin_memcpy(&p_dest, (void *)((char *)DAT_006d4988 + 0x1f + iVar5), 4);
            *p_dest = 7;

            int base_ptr_1;
            int base_ptr_2;
            __builtin_memcpy(&base_ptr_1, (void *)((char *)puVar2 + 0x1f), 4);
            __builtin_memcpy(&base_ptr_2, (void *)((char *)iVar3 + 0x1f), 4);
            *(unsigned char *)((char *)base_ptr_1 + 0x86) = *(unsigned char *)((char *)base_ptr_2 + 0x86);

            tmp_short = 0;
            __builtin_memcpy((void *)((char *)base_ptr_1 + 0x81), &tmp_short, 2);

            tmp_short = 0;
            __builtin_memcpy((void *)((char *)base_ptr_1 + 0x7f), &tmp_short, 2);

            iVar1 = (int)DAT_006db71c + (uVar7 - uVar9) * 0x4c;

            __builtin_memcpy((void *)((char *)base_ptr_1 + 0xab), &iVar1, 4);

            tmp_int = 0;
            __builtin_memcpy((void *)((char *)base_ptr_1 + 0xa3), &tmp_int, 4);

            tmp_int = 0;
            __builtin_memcpy((void *)((char *)base_ptr_1 + 0xa7), &tmp_int, 4);

            *(unsigned char *)((char *)DAT_006d4988 + 2 + iVar5) = 0x10;
            *(unsigned char *)((char *)DAT_006d4988 + 3 + iVar5) = (unsigned char)200;

            tmp_short = DAT_006db6c0;
            __builtin_memcpy((void *)((char *)DAT_006d4988 + iVar5), &tmp_short, 2);

            DAT_006db6c0 = DAT_006db6c0 + 1;

            *(unsigned int *)((char *)iVar1 + 0x40) = *puVar4;
            *(unsigned short *)((char *)iVar1 + 0x44) = 0xffff;
            *(short *)((char *)iVar1 + 0x46) = (short)((int)((uVar7 & 7) * 0xec) >> 3);

            __builtin_memcpy((void *)((char *)param_1 + 0x3f2 + uVar6 * 4), &puVar2, 4);
            *(unsigned short *)((char *)param_1 + 0x3c6 + uVar6 * 2) = *puVar2;
        }
        else {
            tmp_int = 0;
            __builtin_memcpy((void *)((char *)param_1 + 0x3f2 + uVar6 * 4), &tmp_int, 4);
        }
        uStack_12 = uStack_12 + 1;
        if (0xf < uStack_12) {
            return;
        }
    } while (1);
}
