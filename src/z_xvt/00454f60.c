/* SPDX-License-Identifier: MIT */
/* z_xvt 00454f60 */
extern unsigned char *DAT_00732e54;
extern unsigned short DAT_006d9d60;
extern unsigned short DAT_006d9d50;
extern unsigned char *DAT_006d4988;
extern unsigned char DAT_0070ad85[];
extern unsigned char DAT_0070ad40[];
extern unsigned char DAT_0052b0b4[];
extern unsigned char DAT_0052b0b6[];
extern unsigned char DAT_006d4a94[];
extern unsigned char DAT_006d4a96[];
extern unsigned char DAT_006d4ab7[];
extern unsigned char DAT_006d4ab8[];
extern unsigned char DAT_0070b252[];
extern unsigned char DAT_0070acf0[];
extern unsigned short DAT_00732fe2;
extern unsigned short _DAT_00732952;
extern unsigned short _DAT_00732954;

unsigned int __cdecl FUN_00478320(unsigned int param_1, unsigned short param_2);
short __cdecl FUN_00446fa0(unsigned int param_1);
unsigned int __cdecl FUN_0040c070(int param_1, int param_2, int param_3, unsigned short param_4);
void __cdecl FUN_004a3490(int param_1, int param_2);
unsigned int __cdecl FUN_004ac6f0(unsigned int param_1, void *param_2);
void __cdecl FUN_004a1d70(int param_1, int param_2);

unsigned char __stdcall FUN_00454f60(void)
{
  unsigned char bVar1;
  char cVar2;
  unsigned char bVar3;
  unsigned short uVar4;
  short sVar5;
  unsigned short uVar10;
  int iVar6;
  unsigned int uVar7;
  unsigned char *pcVar8;
  unsigned int uVar9;
  unsigned char *pcVar12;
  unsigned char bVar13;
  unsigned short local_6;
  unsigned short uStack_4 = 0;
  unsigned short uVar11;
  unsigned short extraout_var = 0;
  unsigned short extraout_var_00 = 0;
  short s_20b;
  short s_20d;
  unsigned short u_c7;
  unsigned short u_c3;
  int i_252;
  unsigned char *p_1f;

  if (*(short *)(DAT_00732e54 + 0x92) != 0) {
    bVar13 = 0;
    uVar10 = (unsigned short)(((unsigned int)DAT_006d9d60 * 0x2b3u) >> 0x10);
    uVar11 = (unsigned short)((unsigned int)DAT_00732e54 >> 0x10);
    switch (DAT_0070ad85[(unsigned int)DAT_006d9d60 * 0x566u]) {
    case 1:
      bVar13 = (DAT_00732e54[0x1df] & 1) == 0;
      cVar2 = *(char *)(DAT_006d4988 + ((unsigned int)DAT_006d9d50 * 0x23u + 2));
      __builtin_memcpy(&s_20b, DAT_00732e54 + 0x20b, 2);
      __builtin_memcpy(&s_20d, DAT_00732e54 + 0x20d, 2);
      if (((cVar2 == 3) || (cVar2 == 4)) &&
          ((int)s_20b + (int)s_20d == 0)) {
        bVar13 = 1;
      }
      uStack_4 = 0x17c;
      break;
    case 2:
      bVar13 = (DAT_00732e54[0x1df] & 0x10) == 0;
      uStack_4 = 0x180;
      break;
    case 3:
      bVar13 = 0;
      local_6 = 0;
      bVar1 = DAT_00732e54[0x228];
      bVar3 = 0;
      if (bVar1 != 0) {
        do {
          bVar13 = bVar3;
          uVar7 = (unsigned int)local_6;
          if (DAT_00732e54[uVar7 + 0x229] != 0) {
            iVar6 = (int)((unsigned int)DAT_00732e54[4] * 0xefu + uVar7);
            uVar7 = (unsigned int)DAT_0052b0b4[iVar6];
            uVar4 = (unsigned short)DAT_0052b0b4[iVar6];
            while (uVar4 <= DAT_0052b0b6[iVar6]) {
              if (DAT_00732e54[uVar7 * 4u + 0x346] != 0) {
                bVar13 = 1;
              }
              uVar4 = (unsigned short)((short)uVar7 + 1);
              uVar7 = (unsigned int)uVar4;
            }
          }
          local_6 = (unsigned short)(local_6 + 1);
          bVar3 = bVar13;
        } while (local_6 < bVar1);
      }
      bVar13 = !bVar13 || (DAT_00732e54[0x1df] & 8) == 0;
      uStack_4 = 0x180;
      break;
    case 4:
      __builtin_memcpy(&u_c7, DAT_00732e54 + 199, 2);
      __builtin_memcpy(&u_c3, DAT_00732e54 + 0xc3, 2);
      uVar7 = FUN_00478320(((unsigned int)uVar11 << 16) | (unsigned int)u_c7, 0x8000);
      bVar13 = (unsigned short)uVar7 <= u_c3;
      uStack_4 = 0x17e;
      break;
    case 5:
      uVar4 = 0;
      do {
        if (DAT_00732e54[(unsigned int)uVar4 + 0x1e6] != 0) {
          bVar13 = 1;
        }
        uVar4 = (unsigned short)(uVar4 + 1);
      } while (uVar4 < 10);
      uStack_4 = 0x181;
      break;
    case 6:
      uVar4 = 0x8000;
      sVar5 = FUN_00446fa0(((unsigned int)uVar10 << 16) | (unsigned int)DAT_006d9d50);
      uVar7 = FUN_00478320(((unsigned int)extraout_var << 16) | (unsigned int)(unsigned short)sVar5, uVar4);
      __builtin_memcpy(&s_20b, DAT_00732e54 + 0x20b, 2);
      __builtin_memcpy(&s_20d, DAT_00732e54 + 0x20d, 2);
      bVar13 = (int)s_20b + (int)s_20d <= (int)(uVar7 & 0xffff);
      uStack_4 = 0x17a;
      break;
    case 7:
      uVar4 = 0x4000;
      sVar5 = FUN_00446fa0(((unsigned int)uVar10 << 16) | (unsigned int)DAT_006d9d50);
      uVar7 = FUN_00478320(((unsigned int)extraout_var_00 << 16) | (unsigned int)(unsigned short)sVar5, uVar4);
      __builtin_memcpy(&s_20b, DAT_00732e54 + 0x20b, 2);
      __builtin_memcpy(&s_20d, DAT_00732e54 + 0x20d, 2);
      bVar13 = (int)s_20b + (int)s_20d <= (int)(uVar7 & 0xffff);
      uStack_4 = 0x17b;
      break;
    case 8:
      __builtin_memcpy(&u_c7, DAT_00732e54 + 199, 2);
      __builtin_memcpy(&u_c3, DAT_00732e54 + 0xc3, 2);
      uVar7 = FUN_00478320(((unsigned int)uVar11 << 16) | (unsigned int)u_c7, 0x4000);
      bVar13 = (unsigned short)uVar7 <= u_c3;
      uStack_4 = 0x17d;
      break;
    case 9:
      __builtin_memcpy(&u_c7, DAT_00732e54 + 199, 2);
      __builtin_memcpy(&u_c3, DAT_00732e54 + 0xc3, 2);
      uVar7 = FUN_00478320(((unsigned int)uVar11 << 16) | (unsigned int)u_c7, 0xc000);
      bVar13 = (unsigned short)uVar7 <= u_c3;
      uStack_4 = 0x17f;
    }
    if (bVar13) {
      if (DAT_00732e54[0x73] == 0) {
        uVar7 = (unsigned int)DAT_006d9d60;
        *(short *)(DAT_006d4a94 + uVar7 * 0x126u) =
            (short)(*(short *)(DAT_006d4a94 + uVar7 * 0x126u) + 1);
        iVar6 = (int)(uVar7 * 0x126u);
        pcVar12 = DAT_00732e54 + 0xb0;
        cVar2 = (char)DAT_0070ad40[uVar7 * 0x566u];
        if ((char)*pcVar12 == cVar2) {
          DAT_006d4ab7[iVar6] = 1;
        }
        pcVar8 = DAT_00732e54 + 0x74;
        if (*pcVar8 != 0) {
          *(short *)(DAT_006d4a96 + iVar6) = (short)(*(short *)(DAT_006d4a96 + iVar6) + -1);
          if ((char)*pcVar12 == cVar2) {
            DAT_006d4ab8[iVar6] = 0;
          }
          *pcVar8 = 0;
        }
        FUN_0040c070(1, 0xf, (int)(unsigned int)DAT_006d9d50, 0xffff);
        __builtin_memcpy(&i_252, DAT_0070b252 + (unsigned int)DAT_006d9d60 * 0x566u, 4);
        if (i_252 != -1) {
          __builtin_memcpy(&p_1f, DAT_006d4988 + ((unsigned int)DAT_006d9d50 * 0x23u + 0x1f), 4);
          DAT_00732fe2 = (unsigned short)p_1f[0x86];
          FUN_004a3490(0, (int)(DAT_0070acf0 + (unsigned int)DAT_006d9d60 * 0x566u));
          uVar9 = FUN_004ac6f0((unsigned int)DAT_006d9d60, DAT_00732e54);
          _DAT_00732952 = (unsigned short)uVar9;
          _DAT_00732954 = uStack_4;
          __builtin_memcpy(&i_252, DAT_0070b252 + (unsigned int)DAT_006d9d60 * 0x566u, 4);
          FUN_004a1d70(0x179, i_252);
        }
      }
      DAT_00732e54[0x73] = 1;
    }
    return bVar13;
  }
  return 0;
}
