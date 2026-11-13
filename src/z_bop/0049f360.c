/* SPDX-License-Identifier: MIT */
/* z_bop 0049f360 */
extern unsigned char DAT_00a08104[];
extern unsigned char DAT_009d8ce4[];
extern unsigned int DAT_009a73c0;
extern unsigned int DAT_009a73c2;
extern unsigned char DAT_009a73b6[];
extern void (__stdcall * _DAT_009993f8)(void);
extern void (__stdcall * PTR_caseD_0_005278b0[])(void);

void __stdcall FUN_0049f360(void)
{
  unsigned int v1;
  unsigned short v2;
  unsigned short v3;
  short v4;

  __builtin_memcpy(&v1, DAT_00a08104 + 0xb1, 4);
  v1 = 0;
  __builtin_memcpy(DAT_00a08104 + 0xb1, &v1, 4);

  __builtin_memcpy(&v1, DAT_00a08104 + 0xb1, 4);
  __builtin_memcpy(DAT_00a08104 + 0xb5, &v1, 4);

  __builtin_memcpy(&v1, DAT_00a08104 + 0xb5, 4);
  __builtin_memcpy(DAT_00a08104 + 0xb9, &v1, 4);

  *(unsigned char *)(DAT_00a08104 + 0x79) = 0;
  *(unsigned char *)(DAT_00a08104 + 0x78) = 0;

  __builtin_memcpy(&v2, DAT_00a08104 + 0x23b, 2);
  v2 = 0;
  __builtin_memcpy(DAT_00a08104 + 0x23b, &v2, 2);

  v3 = (unsigned short)(unsigned char)DAT_009d8ce4[(unsigned int)DAT_009a73c0 * 0x566 + (unsigned int)DAT_009a73c2 * 0x52];
  __builtin_memcpy(DAT_00a08104 + 0xc1, &v3, 2);

  __builtin_memcpy(&v4, DAT_00a08104 + 0xc1, 2);
  v4 = v4 * 5;
  __builtin_memcpy(DAT_00a08104 + 0xc1, &v4, 2);

  *(unsigned char *)(DAT_00a08104 + 0xa4) = 4;
  *(unsigned char *)(DAT_009a73b6 + 0x35) = 0;
  _DAT_009993f8 = PTR_caseD_0_005278b0[*(unsigned char *)(DAT_009a73b6 + 0x34)];
  (*_DAT_009993f8)();
  return;
}
