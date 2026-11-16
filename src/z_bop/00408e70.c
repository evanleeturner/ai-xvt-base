/* SPDX-License-Identifier: MIT */
/* z_bop 00408e70 */
extern int DAT_0099945c;
extern unsigned char DAT_0099946a[];
extern unsigned char DAT_0099944a[];
extern int DAT_0054f98c;
extern int DAT_0054f99c;
extern int DAT_0054f978;
extern int DAT_0054f998;
extern int DAT_0054f994;
extern int DAT_0054f9a0;
extern int DAT_0054f988;
extern int DAT_00528c5c;
extern int DAT_00999478;
extern float _DAT_00518070;
extern int DAT_0053f8f0[];
extern int DAT_0053f8ec[];
extern int DAT_0053f8f4[];
extern int DAT_0052f870[];
extern int DAT_0052f86c[];
extern unsigned int DAT_009a1fe4;
extern int DAT_0053f970;
extern int DAT_00527e94;
extern int DAT_0051a54c;

void __cdecl FUN_0040ac80(int, int, int);
void __cdecl FUN_004096d0(int, int, int);
void __cdecl FUN_00409c10(int, int, int);
void __cdecl FUN_0040a1a0(int, int, int);
void __cdecl FUN_0040a6e0(int, int, int);
unsigned int __cdecl FUN_0040b350(int, int);
long long __cdecl __ftol(void);
unsigned char * __cdecl FUN_00407e40(int, int, unsigned int *, unsigned int);
unsigned char * __cdecl FUN_00407f10(int, int, unsigned short *, unsigned char *);

void __cdecl FUN_00408e70(int *param_1)
{
  int iVar1;
  float *pfVar2;
  unsigned int *puVar3;
  unsigned int *puVar4;
  unsigned int uVar5;
  int iVar6;
  int iVar7;
  int *piVar8;
  int iVar9;
  unsigned char *pbVar10;
  int iVar11;
  int iVar12;
  int *piVar13;
  int iVar14;
  unsigned int *puVar15;
  long long lVar16;
  float local_30;
  int *local_28;
  float local_24;
  float local_20;
  int *local_1c;
  unsigned char *local_18;
  int local_10;
  int local_c;
  unsigned char *local_8;
  unsigned char *local_4;

  __builtin_memcpy(&puVar3, DAT_0099946a, 4);
  iVar7 = 0;
  local_8 = (unsigned char *)0;
  iVar1 = (int)((unsigned int)DAT_0099945c + (unsigned int)param_1[0x2f] * 0x18u);
  DAT_0054f98c = (int)((unsigned int)param_1[0x32] + (unsigned int)param_1[0x2f]);
  __builtin_memcpy(&iVar9, DAT_0099944a, 4);
  local_28 = (int *)((unsigned int)param_1[0x2e] * 0x70u + (unsigned int)iVar9);
  puVar4 = puVar3;
  DAT_0054f99c = DAT_0054f98c;
  if (0 < DAT_0054f98c) {
    do {
      *puVar4 = 0xffffffff;
      iVar7 = (int)((unsigned int)iVar7 + 1u);
      puVar4 = puVar4 + 1;
    } while (iVar7 < DAT_0054f98c);
  }
  local_10 = 0;
  if (0 < param_1[0x31]) {
    do {
      iVar7 = (int)((unsigned int)*local_28 * 0x40u + (unsigned int)param_1[0x28]);
      local_c = (int)(4u - (unsigned int)(*(int *)((unsigned int)iVar7 + 0x1cu) == -1));
      DAT_0054f978 = local_c;
      if (*(int *)((unsigned int)DAT_00528c5c + 0x1cu) == 0) {
        if (0 < local_c) {
          piVar8 = (int *)((unsigned int)iVar7 + 0x20u);
          piVar13 = &DAT_0053f8f0[0];
          do {
            iVar7 = *(int *)((unsigned int)DAT_00999478 + (unsigned int)piVar8[-8] * 4u);
            *piVar13 = iVar7;
            iVar11 = DAT_0054f99c;
            pfVar2 = (float *)((unsigned int)param_1[0x20] + (unsigned int)*piVar8 * 8u);
            puVar4 = (unsigned int *)((unsigned int)iVar1 + (unsigned int)iVar7 * 0x18u);
            if ((*(float *)((unsigned int)iVar1 + 0x10u + (unsigned int)iVar7 * 0x18u) != *pfVar2) ||
                (*(float *)&puVar4[5] != pfVar2[1])) {
              puVar15 = (unsigned int *)((unsigned int)iVar1 + (unsigned int)DAT_0054f99c * 0x18u);
              DAT_0054f99c = (int)((unsigned int)DAT_0054f99c + 1u);
              *puVar15 = *puVar4;
              puVar15[1] = puVar4[1];
              puVar15[3] = puVar4[3];
              puVar15[2] = puVar4[2];
              puVar15[4] = *(unsigned int *)((unsigned int)param_1[0x20] + (unsigned int)*piVar8 * 8u);
              puVar15[5] = *(unsigned int *)((unsigned int)param_1[0x20] + 4u + (unsigned int)*piVar8 * 8u);
              *piVar13 = iVar11;
            }
            piVar8 = piVar8 + 1;
            piVar13 = piVar13 + 1;
            local_c = (int)((unsigned int)local_c - 1u);
          } while (local_c != 0);
        }
      }
      else {
        local_30 = 1.0f;
        iVar11 = *(int *)((unsigned int)param_1[0x2a] + 0x10u);
        iVar12 = *(int *)((unsigned int)param_1[0x2a] + 0x14u);
        iVar14 = iVar12;
        if (iVar12 < iVar11) {
          do {
            local_30 = local_30 * _DAT_00518070;
            iVar14 = (int)((unsigned int)iVar14 * 2u);
          } while (iVar14 < iVar11);
        }
        else if (iVar11 < iVar12) {
          local_30 = 1.0f;
          iVar14 = iVar11;
          do {
            local_30 = local_30 * _DAT_00518070;
            iVar14 = (int)((unsigned int)iVar14 * 2u);
          } while (iVar14 < iVar12);
        }
        if (0 < local_c) {
          piVar8 = (int *)((unsigned int)iVar7 + 0x20u);
          local_1c = &DAT_0053f8f0[0];
          do {
            local_24 = *(float *)((unsigned int)param_1[0x20] + (unsigned int)*piVar8 * 8u);
            local_20 = *(float *)((unsigned int)param_1[0x20] + (unsigned int)*piVar8 * 8u + 4u);
            if (iVar12 < iVar11) {
              local_20 = local_30 * local_20;
            }
            else if (iVar11 < iVar12) {
              local_24 = local_30 * local_24;
            }
            iVar7 = *(int *)((unsigned int)DAT_00999478 + (unsigned int)piVar8[-8] * 4u);
            *local_1c = iVar7;
            iVar14 = DAT_0054f99c;
            puVar4 = (unsigned int *)((unsigned int)iVar1 + (unsigned int)iVar7 * 0x18u);
            if ((*(float *)((unsigned int)iVar1 + 0x10u + (unsigned int)iVar7 * 0x18u) != local_24) ||
                (*(float *)&puVar4[5] != local_20)) {
              puVar15 = (unsigned int *)((unsigned int)iVar1 + (unsigned int)DAT_0054f99c * 0x18u);
              DAT_0054f99c = (int)((unsigned int)DAT_0054f99c + 1u);
              *puVar15 = *puVar4;
              puVar15[1] = puVar4[1];
              puVar15[3] = puVar4[3];
              puVar15[2] = puVar4[2];
              *(float *)&puVar15[4] = local_24;
              *(float *)&puVar15[5] = local_20;
              *local_1c = iVar14;
            }
            piVar8 = piVar8 + 1;
            local_1c = local_1c + 1;
            local_c = (int)((unsigned int)local_c - 1u);
          } while (local_c != 0);
        }
      }
      if (local_28[3] == -1) {
        if (0 < DAT_0054f978) {
          puVar4 = (unsigned int *)&DAT_0053f8f0[0];
          puVar15 = (unsigned int *)&DAT_0052f870[0];
          for (iVar7 = DAT_0054f978; iVar7 != 0; iVar7 = (int)((unsigned int)iVar7 - 1u)) {
            *puVar15 = *puVar4;
            puVar4 = puVar4 + 1;
            puVar15 = puVar15 + 1;
          }
        }
        iVar7 = DAT_0054f978;
        iVar11 = 0;
        DAT_0054f998 = DAT_0054f978;
        DAT_0054f978 = 0;
        if (0 < iVar7) {
          piVar8 = &DAT_0052f870[0];
          iVar7 = DAT_0052f86c[iVar7];
          do {
            iVar12 = *piVar8;
            piVar8 = piVar8 + 1;
            iVar11 = (int)((unsigned int)iVar11 + 1u);
            FUN_0040ac80(iVar7, iVar12, iVar1);
            iVar7 = iVar12;
          } while (iVar11 < DAT_0054f998);
        }
      }
      iVar7 = 0;
      local_28[3] = (int)DAT_009a1fe4;
      DAT_0054f998 = 0;
      if (0 < DAT_0054f978) {
        piVar8 = &DAT_0053f8f0[0];
        iVar11 = DAT_0053f8ec[DAT_0054f978];
        do {
          iVar12 = *piVar8;
          piVar8 = piVar8 + 1;
          iVar7 = (int)((unsigned int)iVar7 + 1u);
          FUN_004096d0(iVar11, iVar12, iVar1);
          iVar11 = iVar12;
        } while (iVar7 < DAT_0054f978);
      }
      iVar7 = 0;
      DAT_0054f978 = 0;
      if (0 < DAT_0054f998) {
        piVar8 = &DAT_0052f870[0];
        iVar11 = DAT_0052f86c[DAT_0054f998];
        do {
          iVar12 = *piVar8;
          piVar8 = piVar8 + 1;
          iVar7 = (int)((unsigned int)iVar7 + 1u);
          FUN_00409c10(iVar11, iVar12, iVar1);
          iVar11 = iVar12;
        } while (iVar7 < DAT_0054f998);
      }
      iVar7 = 0;
      DAT_0054f998 = 0;
      if (0 < DAT_0054f978) {
        piVar8 = &DAT_0053f8f0[0];
        iVar11 = DAT_0053f8ec[DAT_0054f978];
        do {
          iVar12 = *piVar8;
          piVar8 = piVar8 + 1;
          iVar7 = (int)((unsigned int)iVar7 + 1u);
          FUN_0040a1a0(iVar11, iVar12, iVar1);
          iVar11 = iVar12;
        } while (iVar7 < DAT_0054f978);
      }
      iVar7 = 0;
      DAT_0054f978 = 0;
      if (0 < DAT_0054f998) {
        piVar8 = &DAT_0052f870[0];
        iVar11 = DAT_0052f86c[DAT_0054f998];
        do {
          iVar12 = *piVar8;
          piVar8 = piVar8 + 1;
          iVar7 = (int)((unsigned int)iVar7 + 1u);
          FUN_0040a6e0(iVar11, iVar12, iVar1);
          iVar11 = iVar12;
        } while (iVar7 < DAT_0054f998);
      }
      iVar7 = 0;
      if (0 < DAT_0054f978) {
        piVar8 = &DAT_0053f8f0[0];
        do {
          iVar11 = *piVar8;
          if (iVar11 < DAT_0054f98c) {
            if (puVar3[iVar11] == 0xffffffff) {
              uVar5 = FUN_0040b350(iVar11, iVar1);
              puVar3[iVar11] = uVar5;
            }
            *piVar8 = (int)puVar3[*piVar8];
          }
          else {
            iVar11 = (int)FUN_0040b350(iVar11, iVar1);
            *piVar8 = iVar11;
          }
          piVar8 = piVar8 + 1;
          iVar7 = (int)((unsigned int)iVar7 + 1u);
        } while (iVar7 < DAT_0054f978);
      }
      if (2 < DAT_0054f978) {
        iVar9 = 0;
        iVar7 = param_1[0x2b];
        iVar11 = param_1[0x2a];
        iVar12 = *(int *)((unsigned int)iVar11 + 0x10u);
        iVar14 = *(int *)((unsigned int)iVar11 + 0x14u);
        if ((int)((unsigned int)iVar14 * (unsigned int)iVar12 -
                  (unsigned int)*(int *)((unsigned int)iVar11 + 8u)) == 0) {
          lVar16 = __ftol();
          iVar11 = (int)lVar16;
          for (; ((0x100 < iVar11) && (iVar12 != 8)) && (iVar14 != 8); iVar14 = iVar14 >> 1) {
            iVar11 = iVar11 >> 2;
            iVar6 = (int)((unsigned int)iVar14 * (unsigned int)iVar12);
            iVar12 = iVar12 >> 1;
            iVar9 = (int)((unsigned int)iVar9 + (unsigned int)iVar6);
          }
        }
        pbVar10 = (unsigned char *)((unsigned int)iVar7 + (unsigned int)iVar9);
        if (local_8 != pbVar10) {
          iVar7 = param_1[0x2d];
          local_4 = FUN_00407e40(iVar12, iVar14, (unsigned int *)((unsigned int)iVar7 + 0x1000u),
                                 (unsigned int)pbVar10);
          local_18 = (unsigned char *)0;
          local_8 = pbVar10;
          if (*(short *)((unsigned int)iVar7 + 0x1200u) != 0) {
            if ((*(char *)((unsigned int)*param_1 + 2u) == '\x06') ||
                (*(char *)((unsigned int)*param_1 + 2u) == '\a')) {
              *(unsigned short *)((unsigned int)iVar7 + 0x1200u) = 0;
            }
            else if ((((char *)param_1[0x29] != (char *)0) && (*(char *)param_1[0x29] != '_')) &&
                     ((local_18 = FUN_00407f10(iVar12, iVar14, (unsigned short *)param_1[0x2d], pbVar10),
                       local_18 == (unsigned char *)0 &&
                       ((*(int *)((unsigned int)param_1[0x2a] + 0x10u) == iVar12 &&
                         (*(int *)((unsigned int)param_1[0x2a] + 0x14u) == iVar14)))))) {
              *(unsigned char *)param_1[0x29] = 0x5f;
            }
          }
        }
      }
      if (local_18 != (unsigned char *)0) {
        iVar7 = 0;
        if (0 < DAT_0054f978) {
          do {
            puVar4 = (unsigned int *)((unsigned int)DAT_0053f8f0[iVar7] * 0x20u +
                                      (unsigned int)DAT_0053f970);
            puVar15 = (unsigned int *)(((unsigned int)DAT_0054f994 + (unsigned int)iVar7) * 0x20u +
                                       (unsigned int)DAT_0053f970);
            for (iVar11 = 8; iVar11 != 0; iVar11 = (int)((unsigned int)iVar11 - 1u)) {
              *puVar15 = *puVar4;
              puVar4 = puVar4 + 1;
              puVar15 = puVar15 + 1;
            }
            iVar11 = (int)((unsigned int)DAT_0054f994 + (unsigned int)iVar7);
            iVar7 = (int)((unsigned int)iVar7 + 1u);
            *(unsigned int *)((unsigned int)iVar11 * 0x20u + 0x10u + (unsigned int)DAT_0053f970) =
                0xffffffff;
          } while (iVar7 < DAT_0054f978);
        }
        iVar7 = DAT_0054f994;
        iVar11 = 2;
        DAT_0054f994 = (int)((unsigned int)DAT_0054f978 + (unsigned int)DAT_0054f994);
        if (2 < DAT_0054f978) {
          do {
            *(int *)((unsigned int)DAT_0054f9a0 * 0x14u + (unsigned int)DAT_0054f988) = iVar7;
            *(int *)((unsigned int)DAT_0054f9a0 * 0x14u + 4u + (unsigned int)DAT_0054f988) =
                (int)((unsigned int)iVar7 + (unsigned int)iVar11 - 1u);
            *(int *)((unsigned int)DAT_0054f9a0 * 0x14u + 8u + (unsigned int)DAT_0054f988) =
                (int)((unsigned int)iVar11 + (unsigned int)iVar7);
            *(unsigned char **)((unsigned int)DAT_0054f9a0 * 0x14u + 0x10u + (unsigned int)DAT_0054f988) =
                local_18;
            *(unsigned int *)((unsigned int)DAT_0054f9a0 * 0x14u + 0xcu + (unsigned int)DAT_0054f988) =
                0x9813u;
            if (DAT_00527e94 != 0) {
              piVar8 = (int *)((unsigned int)DAT_0054f9a0 * 0x14u + 0xcu + (unsigned int)DAT_0054f988);
              *piVar8 = (int)((unsigned int)*piVar8 + 0x180u);
            }
            iVar11 = (int)((unsigned int)iVar11 + 1u);
            *(int *)((unsigned int)DAT_0054f9a0 * 0x14u + 0xcu + (unsigned int)DAT_0054f988) =
                (int)((unsigned int)*(int *)((unsigned int)DAT_0054f9a0 * 0x14u + 0xcu +
                                             (unsigned int)DAT_0054f988) + 0x200u);
            DAT_0054f9a0 = (int)((unsigned int)DAT_0054f9a0 + 1u);
          } while (iVar11 < DAT_0054f978);
        }
      }
      iVar7 = 2;
      if (2 < DAT_0054f978) {
        puVar4 = (unsigned int *)&DAT_0053f8f4[0];
        do {
          *(int *)((unsigned int)DAT_0054f9a0 * 0x14u + (unsigned int)DAT_0054f988) = DAT_0053f8f0[0];
          *(unsigned int *)((unsigned int)DAT_0054f9a0 * 0x14u + 4u + (unsigned int)DAT_0054f988) =
              *puVar4;
          *(unsigned int *)((unsigned int)DAT_0054f9a0 * 0x14u + 8u + (unsigned int)DAT_0054f988) =
              puVar4[1];
          *(unsigned char **)((unsigned int)DAT_0054f9a0 * 0x14u + 0x10u + (unsigned int)DAT_0054f988) =
              local_4;
          *(unsigned int *)((unsigned int)DAT_0054f9a0 * 0x14u + 0xcu + (unsigned int)DAT_0054f988) =
              0x9813u;
          if (DAT_00527e94 != 0) {
            piVar8 = (int *)((unsigned int)DAT_0054f9a0 * 0x14u + 0xcu + (unsigned int)DAT_0054f988);
            *piVar8 = (int)((unsigned int)*piVar8 + 0x180u);
          }
          if (DAT_0051a54c != 0) {
            piVar8 = (int *)((unsigned int)DAT_0054f9a0 * 0x14u + 0xcu + (unsigned int)DAT_0054f988);
            *piVar8 = (int)((unsigned int)*piVar8 + 0x200u);
            DAT_0051a54c = 0;
          }
          iVar7 = (int)((unsigned int)iVar7 + 1u);
          DAT_0054f9a0 = (int)((unsigned int)DAT_0054f9a0 + 1u);
          puVar4 = puVar4 + 1;
        } while (iVar7 < DAT_0054f978);
      }
      local_10 = (int)((unsigned int)local_10 + 1u);
      local_28 = local_28 + 0x1c;
    } while (local_10 < param_1[0x31]);
  }
  return;
}
