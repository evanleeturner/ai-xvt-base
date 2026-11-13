/* SPDX-License-Identifier: MIT */
/* z_xvt 00421df0 */
extern int DAT_006d4988;

unsigned int __cdecl FUN_004490f0(unsigned int param_1);
unsigned int __cdecl FUN_0044a600(unsigned int param_1, unsigned int param_2);
unsigned int __cdecl FUN_00421ff0(unsigned int param_1, char param_2);
unsigned int __cdecl FUN_00478470(void);

void __cdecl FUN_00421df0(unsigned int param_1, short param_2)
{
  unsigned short local_10;
  unsigned int iVar5;
  unsigned int uVar10;
  unsigned short uVar8;
  int iVar3;
  unsigned int uVar7;
  unsigned int uVar11;
  unsigned short uVar9;
  short sVar14;
  short sVar13;
  short sVar15;
  unsigned int uVar12;
  int iVar6;
  int iVar4;

  iVar5 = (param_1 & 0xffff) * 0x23;
  uVar10 = FUN_004490f0(*(unsigned char *)(DAT_006d4988 + 3 + iVar5));
  uVar8 = (unsigned short)uVar10;
  if (1 < uVar8) {
    local_10 = 0;
    iVar3 = *(int *)(*(int *)(DAT_006d4988 + 0x1f + iVar5) + 0xa7);
    if (uVar8 != 0) {
      do {
        uVar7 = (unsigned int)local_10;
        if ((*(char *)(uVar7 + 0x2ca + iVar3) == '\0') &&
           (uVar11 = FUN_0044a600(*(unsigned char *)(DAT_006d4988 + 3 + iVar5), uVar7), uVar11 != 0)) {
          uVar11 = FUN_00421ff0(param_1, (char)local_10);
          if ((short)uVar11 != -1) {
            uVar9 = FUN_00478470();
            sVar14 = (uVar9 & 0x3fff) + 0x4000;
            uVar9 = FUN_00478470();
            sVar13 = (uVar9 & 0x7ff) + 0x400;
            uVar9 = FUN_00478470();
            sVar15 = (uVar9 & 0xfff) + 0x400;
            uVar12 = FUN_00478470();
            if ((uVar12 & 1) != 0) {
              sVar14 = -sVar14;
              sVar13 = -sVar13;
            }
            uVar12 = FUN_00478470();
            if ((uVar12 & 1) != 0) {
              sVar15 = -sVar15;
            }
            iVar6 = (uVar11 & 0xffff) * 0x23;
            *(short *)(*(int *)(DAT_006d4988 + 0x1f + iVar6) + 0x77) = sVar14;
            *(short *)(DAT_006d4988 + 0x10 + iVar6) += sVar13;
            *(short *)(DAT_006d4988 + 0x12 + iVar6) += sVar15;
            uVar9 = *(unsigned short *)(DAT_006d4988 + 0x12 + iVar6);
            if (0x7fff < uVar9) {
              *(unsigned short *)(DAT_006d4988 + 0x12 + iVar6) = -uVar9;
              *(short *)(DAT_006d4988 + 0x10 + iVar6) += -0x8000;
            }
            *(unsigned char *)(*(int *)(DAT_006d4988 + 0x1f + iVar6) + 0x90) = 1;
            iVar4 = *(int *)(DAT_006d4988 + 0x1f + iVar6);
            *(unsigned char *)(iVar4 + 0x89) = *(unsigned char *)(iVar4 + 0x90);
            uVar9 = FUN_00478470();
            *(unsigned short *)(*(int *)(DAT_006d4988 + 0x1f + iVar6) + 0x7f) = ((uVar9 & 1) + 1) * 0xec;
            *(unsigned char *)(DAT_006d4988 + 0x1a + iVar6) = 2;
            *(unsigned char *)(uVar7 + 0x2ca + iVar3) = 4;
            *(unsigned char *)((uVar10 & 0xffff) + 0x2ca + iVar3) = 2;
            if (param_2 == 0) {
              return;
            }
          }
        }
        local_10 = local_10 + 1;
      } while (local_10 < uVar8);
    }
  }
  return;
}
