/* SPDX-License-Identifier: MIT */
/* z_bop 00417a60 */
extern char *DAT_00550880;
extern int DAT_00550b8c;
extern int DAT_009d1308;
extern int DAT_00a07ce8;
extern int DAT_009a8e24;
extern int DAT_009cd274;

void * __cdecl FID_conflict__memcpy(void *dst, const void *src, size_t n);

void __cdecl FUN_00417a60(byte *param_1)
{
  int iVar1;
  byte *pbVar2;
  size_t sVar3;
  uint uVar4;
  byte bVar5;
  ushort uVar6;
  char *pcVar7;
  char *pcVar8;
  char *pcVar9;
  char *pcVar10;
  char *pcVar11;
  int local_10;
  int local_c;
  int iTmp;

  pcVar7 = DAT_00550880 + DAT_00550b8c;
  __builtin_memcpy(&iVar1, param_1, 4);
  local_c = 0;
  local_10 = 0;
  pcVar8 = DAT_00550880;
  param_1 = param_1 + 4;
  if (0 < (int)((uint)DAT_009d1308 + (uint)DAT_00a07ce8)) {
    do {
      if ((local_10 < DAT_009a8e24) || (pcVar10 = pcVar8, DAT_009cd274 <= local_10)) {
        if (local_c == 0) {
          if (iVar1 <= local_10) break;
          pbVar2 = param_1 + 1;
          bVar5 = *param_1;
          param_1 = pbVar2;
          if ((bVar5 & 0x80) != 0) {
            uVar6 = (ushort)bVar5;
            bVar5 = 0;
            local_c = (int)((uint)(uVar6 & 0x7f) - 1u);
          }
        }
        else {
          local_c = (int)((uint)local_c - 1u);
          bVar5 = 0;
        }
        pcVar9 = pcVar8 + 1;
        pcVar10 = pcVar9;
        if (*pcVar8 == '\0') {
          if ((bVar5 & 1) != 0) {
            pcVar10 = pcVar8 + 0x24;
            sVar3 = (size_t)(pcVar7 - pcVar9);
            pcVar7 = pcVar7 + 0x23;
            FID_conflict__memcpy(pcVar10, pcVar9, sVar3);
            pcVar8 = pcVar9;
            for (uVar4 = (uint)(pcVar10 - pcVar9) >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
              pcVar8[0] = '\0';
              pcVar8[1] = '\0';
              pcVar8[2] = '\0';
              pcVar8[3] = '\0';
              pcVar8 = pcVar8 + 4;
            }
            for (uVar4 = (uint)(pcVar10 - pcVar9) & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
              *pcVar8 = '\0';
              pcVar8 = pcVar8 + 1;
            }
          }
        }
        else if ((bVar5 & 1) == 0) {
          FID_conflict__memcpy(pcVar9, pcVar8 + 0x24, (size_t)(pcVar7 - (pcVar8 + 0x24)));
          pcVar7 = pcVar7 + -0x23;
        }
        else {
          pcVar9 = pcVar8 + 0x24;
          pcVar10 = pcVar9;
          __builtin_memcpy(&iTmp, pcVar8 + 0x20, 4);
          if (iTmp == 0) {
            if ((bVar5 & 2) != 0) {
              pcVar10 = pcVar8 + 0xd5;
              sVar3 = (size_t)(pcVar7 - pcVar9);
              pcVar7 = pcVar7 + 0xb1;
              FID_conflict__memcpy(pcVar10, pcVar9, sVar3);
              pcVar8 = pcVar9;
              for (uVar4 = (uint)(pcVar10 - pcVar9) >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
                pcVar8[0] = '\0';
                pcVar8[1] = '\0';
                pcVar8[2] = '\0';
                pcVar8[3] = '\0';
                pcVar8 = pcVar8 + 4;
              }
              for (uVar4 = (uint)(pcVar10 - pcVar9) & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                *pcVar8 = '\0';
                pcVar8 = pcVar8 + 1;
              }
            }
          }
          else if ((bVar5 & 2) == 0) {
            FID_conflict__memcpy(pcVar9, pcVar8 + 0xd5, (size_t)(pcVar7 - (pcVar8 + 0xd5)));
            pcVar7 = pcVar7 + -0xb1;
          }
          else {
            pcVar9 = pcVar8 + 0xd5;
            pcVar10 = pcVar9;
            __builtin_memcpy(&iTmp, pcVar8 + 0xcd, 4);
            if (iTmp == 0) {
              if ((bVar5 & 4) != 0) {
                pcVar10 = pcVar8 + 0x537;
                sVar3 = (size_t)(pcVar7 - pcVar9);
                pcVar7 = pcVar7 + 0x462;
                FID_conflict__memcpy(pcVar10, pcVar9, sVar3);
                pcVar11 = pcVar9;
                for (uVar4 = (uint)(pcVar10 - pcVar9) >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
                  pcVar11[0] = '\0';
                  pcVar11[1] = '\0';
                  pcVar11[2] = '\0';
                  pcVar11[3] = '\0';
                  pcVar11 = pcVar11 + 4;
                }
                for (uVar4 = (uint)(pcVar10 - pcVar9) & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                  *pcVar11 = '\0';
                  pcVar11 = pcVar11 + 1;
                }
              }
            }
            else if ((bVar5 & 4) == 0) {
              FID_conflict__memcpy(pcVar9, pcVar8 + 0x537, (size_t)(pcVar7 - (pcVar8 + 0x537)));
              pcVar7 = pcVar7 + -0x462;
            }
            else {
              pcVar10 = pcVar8 + 0x537;
            }
            pcVar9 = pcVar10;
            __builtin_memcpy(&iTmp, pcVar8 + 0xc9, 4);
            if (iTmp == 0) {
              if ((bVar5 & 8) != 0) {
                pcVar9 = pcVar10 + 10;
                sVar3 = (size_t)(pcVar7 - pcVar10);
                pcVar7 = pcVar7 + 10;
                FID_conflict__memcpy(pcVar9, pcVar10, sVar3);
                pcVar11 = pcVar10;
                for (uVar4 = (uint)(pcVar9 - pcVar10) >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
                  pcVar11[0] = '\0';
                  pcVar11[1] = '\0';
                  pcVar11[2] = '\0';
                  pcVar11[3] = '\0';
                  pcVar11 = pcVar11 + 4;
                }
                for (uVar4 = (uint)(pcVar9 - pcVar10) & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                  *pcVar11 = '\0';
                  pcVar11 = pcVar11 + 1;
                }
              }
            }
            else if ((bVar5 & 8) == 0) {
              FID_conflict__memcpy(pcVar10, pcVar10 + 10, (size_t)(pcVar7 - (pcVar10 + 10)));
              pcVar7 = pcVar7 + -10;
            }
            else {
              pcVar9 = pcVar10 + 10;
            }
            pcVar10 = pcVar9;
            __builtin_memcpy(&iTmp, pcVar8 + 0xd1, 4);
            if (iTmp == 0) {
              if ((bVar5 & 0x10) != 0) {
                pcVar10 = pcVar9 + 0x4c;
                sVar3 = (size_t)(pcVar7 - pcVar9);
                pcVar7 = pcVar7 + 0x4c;
                FID_conflict__memcpy(pcVar10, pcVar9, sVar3);
                pcVar8 = pcVar9;
                for (uVar4 = (uint)(pcVar10 - pcVar9) >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
                  pcVar8[0] = '\0';
                  pcVar8[1] = '\0';
                  pcVar8[2] = '\0';
                  pcVar8[3] = '\0';
                  pcVar8 = pcVar8 + 4;
                }
                for (uVar4 = (uint)(pcVar10 - pcVar9) & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                  *pcVar8 = '\0';
                  pcVar8 = pcVar8 + 1;
                }
              }
            }
            else if ((bVar5 & 0x10) == 0) {
              FID_conflict__memcpy(pcVar9, pcVar9 + 0x4c, (size_t)(pcVar7 - (pcVar9 + 0x4c)));
              pcVar7 = pcVar7 + -0x4c;
            }
            else {
              pcVar10 = pcVar9 + 0x4c;
            }
          }
        }
      }
      local_10 = (int)((uint)local_10 + 1u);
      pcVar8 = pcVar10;
    } while (local_10 < (int)((uint)DAT_009d1308 + (uint)DAT_00a07ce8));
  }
  DAT_00550b8c = (int)(pcVar7 - DAT_00550880);
  return;
}
