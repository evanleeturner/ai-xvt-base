/* SPDX-License-Identifier: MIT */
/* z_xvt 00491ad0 */
extern unsigned char DAT_00732e54[];
extern unsigned char DAT_0070ada4[];
extern unsigned int DAT_006d9d60;
extern unsigned int DAT_006d9d62;
extern unsigned char DAT_006d9d56[];
extern void (*PTR_caseD_0_0052a6f0[])(void);
extern void (*_DAT_0065864c)(void);

void __stdcall FUN_00491ad0(void)
{
    unsigned int v_b1;
    unsigned int v_b5;
    unsigned int v_b9;
    unsigned short v_231;
    unsigned short v_c1;
    short v_c1_s;

    __builtin_memcpy(&v_b1, DAT_00732e54 + 0xb1, 4);
    v_b1 = 0;
    __builtin_memcpy(DAT_00732e54 + 0xb1, &v_b1, 4);

    __builtin_memcpy(&v_b5, DAT_00732e54 + 0xb1, 4);
    __builtin_memcpy(DAT_00732e54 + 0xb5, &v_b5, 4);

    __builtin_memcpy(&v_b9, DAT_00732e54 + 0xb5, 4);
    __builtin_memcpy(DAT_00732e54 + 0xb9, &v_b9, 4);

    *(unsigned char *)(DAT_00732e54 + 0x79) = 0;
    *(unsigned char *)(DAT_00732e54 + 0x78) = 0;

    __builtin_memcpy(&v_231, DAT_00732e54 + 0x231, 2);
    v_231 = 0;
    __builtin_memcpy(DAT_00732e54 + 0x231, &v_231, 2);

    v_c1 = (unsigned short)DAT_0070ada4[(unsigned int)DAT_006d9d60 * 0x566 + (unsigned int)DAT_006d9d62 * 0x52];
    __builtin_memcpy(DAT_00732e54 + 0xc1, &v_c1, 2);

    __builtin_memcpy(&v_c1_s, DAT_00732e54 + 0xc1, 2);
    v_c1_s = v_c1_s * 5;
    __builtin_memcpy(DAT_00732e54 + 0xc1, &v_c1_s, 2);

    *(unsigned char *)(DAT_00732e54 + 0xa4) = 4;
    *(unsigned char *)(DAT_006d9d56 + 0x35) = 0;
    _DAT_0065864c = PTR_caseD_0_0052a6f0[*(unsigned char *)(DAT_006d9d56 + 0x34)];
    _DAT_0065864c();
}
