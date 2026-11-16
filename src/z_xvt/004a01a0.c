/* SPDX-License-Identifier: MIT */
/* z_xvt 004a01a0 */
extern int DAT_006d4988;
extern int DAT_006d49a0;
extern unsigned char DAT_0070ad48[];
extern char DAT_0070ad40[];
extern unsigned char DAT_0070ad49[];
extern unsigned char DAT_0070ad42[];
extern unsigned char DAT_0070b252[];
extern unsigned char DAT_0071b738[];
extern unsigned char DAT_0071b73c[];
extern unsigned char DAT_0071b7a5[];
extern unsigned char DAT_0071b9e9[];
extern unsigned char DAT_0071ba49[];
extern unsigned char DAT_0071e522[];
extern char DAT_0072f508[];
extern unsigned char DAT_0052b07f[];
extern unsigned char DAT_005247e8[];
extern unsigned char DAT_0052ad08[];
extern char DAT_00703ca0[];
extern char DAT_00732800[];
extern char DAT_00524948[];
extern char s_stationaryldrpln_00524934[];
extern char DAT_005250b4[];
extern char DAT_00525b80[];
extern char s_formevadeldr1pln_005250a0[];
extern char s_formevadeflw1pln_00525b6c[];
extern char s_exithangarpln_00523c08[];
extern char s_enterhangarpln_00523be4[];
extern char s_disabledpln_00524914[];
extern char s_selfdestroypln_00528360[];
extern char s_Rating_points_awarded___d_to_pla_0052ad48[];
extern char s_Rating_points_awarded___d_to_pla_0052ad84[];

unsigned int __cdecl FUN_00478420(unsigned int a, unsigned int b);
void __cdecl FUN_004a0880(unsigned int a, unsigned int b, unsigned int c, int d, int e, unsigned int f);
int __cdecl FUN_0042b150(int a);
unsigned int __cdecl FUN_004a7aa0(unsigned int a);
int __cdecl _sprintf(char *buf, const char *fmt, ...);
void __cdecl FUN_004a1400(unsigned int a, unsigned int b, unsigned int c);
void __cdecl FUN_004a0cb0(unsigned int a, unsigned int b, unsigned int c, unsigned int d);
void __stdcall RaiseException(unsigned long, unsigned long, unsigned long, const unsigned long *);

void __cdecl FUN_004a01a0(unsigned int param_1, unsigned int param_2)
{
  unsigned int uVar1;
  int iVar2;
  unsigned short uVar3;
  unsigned int uVar4;
  unsigned int uVar5;
  int iVar6;
  unsigned int uVar7;
  int iVar8;
  int iVar9;
  unsigned char *pbVar10;
  unsigned char *puVar11;
  unsigned char *puVar12;
  char *pcVar13;
  char *pcVar15;
  unsigned int uVar16;
  int bVar17;
  char *pcVar18;
  int local_50;
  int local_4c;
  int local_48;
  unsigned int local_44;
  int local_40;
  unsigned int local_34;
  unsigned char *local_30;
  unsigned int local_2c;
  int local_28[10];
  unsigned int divisor;
  unsigned int t32;
  unsigned int t32b;
  unsigned short t16;
  unsigned short v0;
  unsigned short v1;
  unsigned short vm2;
  int s32;
  int k;

  for (k = 0; k < 10; k++) {
    local_28[k] = 0;
  }
  local_4c = 0;
  uVar4 = param_2 & 0xffff;
  iVar2 = (int)(uVar4 * 0x23u);
  __builtin_memcpy(&iVar8, (unsigned char *)DAT_006d4988 + iVar2 + 0x1f, 4);
  if (iVar8 != 0) {
    __builtin_memcpy(&local_4c, (unsigned char *)iVar8 + 0xa7, 4);
  }
  t32 = 0;
  if (local_4c != 0) {
    __builtin_memcpy(&t32, (unsigned char *)local_4c + 0xcd, 4);
  }
  if ((local_4c == 0) || (t32 == 0)) {
    iVar6 = (int)((param_1 & 0xffff) * 0x23u);
    __builtin_memcpy(&iVar8, (unsigned char *)DAT_006d4988 + 0x1b + iVar6, 4);
    if (iVar8 != -1) {
      FUN_004a0880(param_2, 0, 3, iVar8, -1, 0);
      if (*((char *)DAT_006d4988 + 2 + iVar2) == '\b') {
        __builtin_memcpy(&s32, (unsigned char *)DAT_006d4988 + 0x1b + iVar6, 4);
        puVar11 = DAT_0071b7a5 + (unsigned int)s32 * 0x5bdu;
        __builtin_memcpy(&t32, puVar11, 4);
        t32 = t32 + 4;
        __builtin_memcpy(puVar11, &t32, 4);
      }
    }
    FUN_004a0cb0(param_2, 0, 3,
                 (unsigned int)DAT_0070ad48[(unsigned int)*((unsigned char *)DAT_006d4988 + 0x16 + iVar6) * 0x566u]);
    return;
  }
  iVar8 = (int)((unsigned int)*((unsigned char *)DAT_006d4988 + iVar2 + 0x16) * 0x566u);
  local_2c = (unsigned int)(DAT_0070ad40[iVar8] == *((char *)local_4c + 0xb0));
  local_50 = -1;
  __builtin_memcpy(&t32, (unsigned char *)local_4c + 0xd1, 4);
  __builtin_memcpy(&t32b, (unsigned char *)local_4c + 0xcd, 4);
  uVar5 = FUN_00478420(t32, t32b);
  if (0x7fff < (unsigned short)uVar5) {
    __builtin_memcpy(&local_50, DAT_0070b252 + iVar8, 4);
  }
  if (local_50 == -1) {
    local_34 = (unsigned int)DAT_0070ad49[(unsigned int)*((unsigned char *)DAT_006d4988 + 0x16 + iVar2) * 0x566u];
  }
  else {
    __builtin_memcpy(&t16, DAT_0071b738 + (unsigned int)local_50 * 0x5bdu, 2);
    local_34 = (unsigned int)t16;
  }
  puVar11 = DAT_0071b73c;
  local_40 = 0;
  local_30 = (unsigned char *)local_4c + 0x1a1;
  do {
    if (puVar11[4] != 0) {
      __builtin_memcpy(&t32, local_30, 4);
      __builtin_memcpy(&t32b, (unsigned char *)local_4c + 0xcd, 4);
      uVar5 = FUN_00478420(t32, t32b);
      uVar3 = (unsigned short)uVar5;
      if (uVar3 < 0xaaaa) {
        if (uVar3 < 0x5999) {
          uVar5 = (unsigned int)(0xccb < uVar3);
        }
        else {
          uVar5 = 2;
        }
      }
      else {
        uVar5 = 3;
      }
      if (uVar5 != 0) {
        FUN_004a0880(param_2, local_2c, uVar5, local_40, local_50, local_34);
        __builtin_memcpy(&v0, puVar11, 2);
        if (DAT_0072f508[(unsigned int)v0 * 0x1e5u +
                         (unsigned int)DAT_0070ad48[(unsigned int)*((unsigned char *)DAT_006d4988 + 0x16 + iVar2) * 0x566u]] == '\0') {
          iVar6 = FUN_0042b150(local_4c);
          bVar17 = 0;
          pcVar18 = DAT_00703ca0 + (unsigned int)*((unsigned char *)iVar6 + 0xb) * 0x55u;
          iVar6 = 8;
          pcVar13 = pcVar18;
          pcVar15 = DAT_00524948;
          do {
            if (iVar6 == 0) break;
            iVar6 = iVar6 + -1;
            bVar17 = *pcVar13 == *pcVar15;
            pcVar13 = pcVar13 + 1;
            pcVar15 = pcVar15 + 1;
          } while (bVar17);
          if (bVar17) {
LAB_004a0469:
            bVar17 = 1;
            if (local_50 != -1) goto LAB_004a0477;
          }
          else {
            iVar6 = 0x11;
            bVar17 = 0;
            pcVar13 = pcVar18;
            pcVar15 = s_stationaryldrpln_00524934;
            do {
              if (iVar6 == 0) break;
              iVar6 = iVar6 + -1;
              bVar17 = *pcVar13 == *pcVar15;
              pcVar13 = pcVar13 + 1;
              pcVar15 = pcVar15 + 1;
            } while (bVar17);
            if (bVar17) goto LAB_004a0469;
            iVar6 = 0xc;
            bVar17 = 0;
            pcVar13 = pcVar18;
            pcVar15 = DAT_005250b4;
            do {
              if (iVar6 == 0) break;
              iVar6 = iVar6 + -1;
              bVar17 = *pcVar13 == *pcVar15;
              pcVar13 = pcVar13 + 1;
              pcVar15 = pcVar15 + 1;
            } while (bVar17);
            if (bVar17) goto LAB_004a0469;
            iVar6 = 0xc;
            bVar17 = 0;
            pcVar13 = pcVar18;
            pcVar15 = DAT_00525b80;
            do {
              if (iVar6 == 0) break;
              iVar6 = iVar6 + -1;
              bVar17 = *pcVar13 == *pcVar15;
              pcVar13 = pcVar13 + 1;
              pcVar15 = pcVar15 + 1;
            } while (bVar17);
            if (bVar17) goto LAB_004a0469;
            iVar6 = 0x11;
            bVar17 = 0;
            pcVar13 = pcVar18;
            pcVar15 = s_formevadeldr1pln_005250a0;
            do {
              if (iVar6 == 0) break;
              iVar6 = iVar6 + -1;
              bVar17 = *pcVar13 == *pcVar15;
              pcVar13 = pcVar13 + 1;
              pcVar15 = pcVar15 + 1;
            } while (bVar17);
            if (bVar17) goto LAB_004a0469;
            iVar6 = 0x11;
            bVar17 = 0;
            pcVar13 = pcVar18;
            pcVar15 = s_formevadeflw1pln_00525b6c;
            do {
              if (iVar6 == 0) break;
              iVar6 = iVar6 + -1;
              bVar17 = *pcVar13 == *pcVar15;
              pcVar13 = pcVar13 + 1;
              pcVar15 = pcVar15 + 1;
            } while (bVar17);
            if (bVar17) goto LAB_004a0469;
            iVar6 = 0xe;
            bVar17 = 0;
            pcVar13 = pcVar18;
            pcVar15 = s_exithangarpln_00523c08;
            do {
              if (iVar6 == 0) break;
              iVar6 = iVar6 + -1;
              bVar17 = *pcVar13 == *pcVar15;
              pcVar13 = pcVar13 + 1;
              pcVar15 = pcVar15 + 1;
            } while (bVar17);
            if (bVar17) goto LAB_004a0469;
            iVar6 = 0xf;
            bVar17 = 0;
            pcVar13 = pcVar18;
            pcVar15 = s_enterhangarpln_00523be4;
            do {
              if (iVar6 == 0) break;
              iVar6 = iVar6 + -1;
              bVar17 = *pcVar13 == *pcVar15;
              pcVar13 = pcVar13 + 1;
              pcVar15 = pcVar15 + 1;
            } while (bVar17);
            if (bVar17) goto LAB_004a0469;
            iVar6 = 0xc;
            bVar17 = 0;
            pcVar13 = pcVar18;
            pcVar15 = s_disabledpln_00524914;
            do {
              if (iVar6 == 0) break;
              iVar6 = iVar6 + -1;
              bVar17 = *pcVar13 == *pcVar15;
              pcVar13 = pcVar13 + 1;
              pcVar15 = pcVar15 + 1;
            } while (bVar17);
            if (bVar17) goto LAB_004a0469;
            iVar6 = 0xf;
            bVar17 = 0;
            pcVar13 = s_selfdestroypln_00528360;
            do {
              if (iVar6 == 0) break;
              iVar6 = iVar6 + -1;
              bVar17 = *pcVar18 == *pcVar13;
              pcVar18 = pcVar18 + 1;
              pcVar13 = pcVar13 + 1;
            } while (bVar17);
            if (bVar17) goto LAB_004a0469;
LAB_004a0477:
            bVar17 = 0;
          }
          uVar7 = FUN_004a7aa0((((unsigned int)DAT_006d4988 & 0xffffff00u) |
                                (unsigned int)*((unsigned char *)DAT_006d4988 + 3 + iVar2)) & 0xffff00ffu);
          local_44 = (unsigned int)DAT_0052b07f[(uVar7 & 0xffff) * 0xefu];
          if (local_44 == 0) {
            bVar17 = 1;
          }
          __builtin_memcpy(&v1, puVar11 + 2, 2);
          uVar7 = FUN_004a7aa0((unsigned int)DAT_005247e8[DAT_0070ad42[(unsigned int)v1 * 0x566u]]);
          if (local_50 == -1) {
            __builtin_memcpy(&uVar16, DAT_0052ad08 + (unsigned int)DAT_0070ad49[iVar8] * 4u, 4);
          }
          else {
            __builtin_memcpy(&t16, DAT_0071b738 + (unsigned int)local_50 * 0x5bdu, 2);
            uVar16 = (unsigned int)t16;
          }
          __builtin_memcpy(&vm2, puVar11 - 4, 2);
          iVar6 = (int)(uVar16 - (unsigned int)vm2);
          if (bVar17) {
            uVar16 = 1;
            if (local_44 == 0) {
              local_44 = 1;
            }
LAB_004a05bf:
            if (uVar16 == 0) {
              uVar16 = 1;
            }
            divisor = (unsigned int)DAT_0052b07f[(uVar7 & 0xffff) * 0xefu];
            if (divisor == 0) {
              RaiseException(0xC0000094UL, 0, 0, 0);
            }
            else {
              iVar6 = (int)(uVar16 * local_44) / (int)divisor;
            }
            if (uVar5 == 2) {
              iVar6 = iVar6 / 2;
            }
            else if (uVar5 == 1) {
              iVar6 = iVar6 / 10;
            }
            if (iVar6 == 0) {
              iVar6 = 1;
            }
            __builtin_memcpy(&s32, puVar11 + 0x69, 4);
            iVar9 = (int)((unsigned int)s32 + (unsigned int)iVar6);
            __builtin_memcpy(puVar11 + 0x69, &iVar9, 4);
            pcVar18 = s_Rating_points_awarded___d_to_pla_0052ad48;
          }
          else {
            if (iVar6 < -4) goto LAB_004a05bf;
            iVar6 = (int)((unsigned int)iVar6 + 4u);
            __builtin_memcpy(&vm2, puVar11 - 4, 2);
            if (vm2 < 0xf) {
              if (iVar6 < 4) {
                iVar6 = 4;
              }
            }
            else if (iVar6 < 3) {
              iVar6 = 3;
            }
            divisor = (unsigned int)DAT_0052b07f[(uVar7 & 0xffff) * 0xefu];
            if (divisor == 0) {
              RaiseException(0xC0000094UL, 0, 0, 0);
            }
            else {
              iVar6 = (int)(uVar16 * (unsigned int)iVar6 * local_44) / (int)divisor;
            }
            if (uVar5 == 2) {
              iVar6 = iVar6 / 2;
            }
            else if (uVar5 == 1) {
              iVar6 = iVar6 / 10;
            }
            __builtin_memcpy(&s32, puVar11 + 0x65, 4);
            iVar9 = (int)((unsigned int)s32 + (unsigned int)iVar6);
            __builtin_memcpy(puVar11 + 0x65, &iVar9, 4);
            pcVar18 = s_Rating_points_awarded___d_to_pla_0052ad84;
          }
          _sprintf(DAT_00732800, pcVar18, iVar6, local_40, iVar9);
          if ((uVar5 == 2) || (uVar5 == 3)) {
            __builtin_memcpy(&v1, puVar11 + 2, 2);
            FUN_004a1400((unsigned int)v1, uVar4, uVar5);
            __builtin_memcpy(&v0, puVar11, 2);
            local_28[v0] = (int)((unsigned int)local_28[v0] + (2u - (unsigned int)(uVar5 == 2)));
          }
        }
      }
    }
    puVar11 = puVar11 + 0x5bd;
    local_30 = local_30 + 4;
    local_40 = local_40 + 1;
    if ((unsigned int)DAT_0071e522 + 1u < (unsigned int)puVar11) {
      uVar5 = 0;
      do {
        uVar16 = 0;
        uVar7 = 0;
        iVar8 = 0;
        local_48 = 0;
        if (0 < DAT_006d49a0) {
          pbVar10 = DAT_0070ad48;
          puVar12 = (unsigned char *)local_4c + 0xe1;
          do {
            if ((unsigned int)*pbVar10 == uVar5) {
              __builtin_memcpy(&uVar1, puVar12, 4);
              uVar16 = uVar16 + uVar1;
              if (uVar7 < uVar1) {
                uVar7 = uVar1;
                local_48 = iVar8;
              }
            }
            puVar12 = puVar12 + 4;
            pbVar10 = pbVar10 + 0x566;
            iVar8 = iVar8 + 1;
          } while (iVar8 < DAT_006d49a0);
        }
        __builtin_memcpy(&t32b, (unsigned char *)local_4c + 0xcd, 4);
        uVar7 = FUN_00478420(uVar16, t32b);
        uVar3 = (unsigned short)uVar7;
        if (uVar3 < 0xaaaa) {
          if (uVar3 < 0x5999) {
            uVar7 = (unsigned int)(0xccb < uVar3);
          }
          else {
            uVar7 = 2;
          }
        }
        else {
          uVar7 = 3;
        }
        if (uVar7 != 0) {
          FUN_004a0cb0(param_2, local_2c, uVar7, uVar5);
        }
        if (((uVar7 == 2) || (uVar7 == 3)) && (local_28[uVar5] < 2)) {
          iVar8 = (int)((2u - (unsigned int)(uVar7 == 2)) - (unsigned int)local_28[uVar5]);
          if (iVar8 == 2) {
            FUN_004a1400((unsigned int)local_48, uVar4, 3);
            if (local_50 != -1) {
              puVar12 = DAT_0071b9e9 + (unsigned int)local_48 * 2u + (unsigned int)local_50 * 0x5bdu;
              __builtin_memcpy(&t16, puVar12, 2);
              t16 = (unsigned short)(t16 + 1);
              __builtin_memcpy(puVar12, &t16, 2);
            }
          }
          else if (iVar8 == 1) {
            FUN_004a1400((unsigned int)local_48, uVar4, 2);
            if (local_50 != -1) {
              puVar12 = DAT_0071ba49 + (unsigned int)local_48 * 2u + (unsigned int)local_50 * 0x5bdu;
              __builtin_memcpy(&t16, puVar12, 2);
              t16 = (unsigned short)(t16 + 1);
              __builtin_memcpy(puVar12, &t16, 2);
            }
          }
        }
        uVar5 = uVar5 + 1;
      } while ((int)uVar5 < 10);
      return;
    }
  } while (1);
}
