/* SPDX-License-Identifier: MIT */
/* z_xvt 00447240 */
extern unsigned int DAT_006d4988;
extern unsigned int DAT_00732e54;
extern int DAT_0071ecf4;
extern unsigned int DAT_0071b704;
extern unsigned int DAT_006db798;
extern unsigned char DAT_0052b0a8[];
extern unsigned char DAT_0052b0aa[];
extern unsigned char DAT_0052b0ac[];
extern unsigned char DAT_0070ad44[];
extern unsigned char DAT_0070b215[];
extern unsigned char DAT_0071b74a[];
extern unsigned char DAT_0071b7b5[];
extern unsigned char DAT_0071b7b9[];

int FUN_0042b150(int param_1);
void FUN_004a1d70(int param_1, int param_2);
unsigned int FUN_00447cf0(int param_1, unsigned int param_2, unsigned int param_3);
unsigned int FUN_004a7aa0(int param_1);
void FUN_0040a3c0(unsigned int param_1, int param_2);

void __cdecl FUN_00447240(int param_1, int param_2)
{
  unsigned char *pbVar1;
  short *psVar2;
  unsigned char bVar3;
  unsigned char bVar4;
  int iVar5;
  unsigned short uVar6;
  unsigned short uVar8;
  int iVar9;
  unsigned int uVar10;
  unsigned int uVar11;
  char cVar12;
  unsigned short uVar13;
  int iVar14;
  unsigned char *pbVar15;
  int iVar16;
  short sVar17;
  unsigned int uVar18;
  unsigned int uVar19;
  unsigned int local_16;
  unsigned short local_12 = 0;
  unsigned short uStack_e = 0;
  unsigned short uStack_c = 0;

  local_16 = 0;
  iVar9 = (int)(DAT_006d4988 + (unsigned int)param_1 * 0x23u);
  __builtin_memcpy(&iVar5, (char *)((unsigned int)iVar9 + 0x1bu), 4);
  __builtin_memcpy(&iVar14, (char *)((unsigned int)iVar9 + 0x1fu), 4);
  __builtin_memcpy(&DAT_00732e54, (char *)((unsigned int)iVar14 + 0xa7u), 4);
  iVar9 = FUN_0042b150((int)DAT_00732e54);
  bVar3 = *(unsigned char *)(DAT_00732e54 + 4);
  uVar6 = (unsigned short)bVar3;
  if (*(char *)(DAT_00732e54 + 0x29) == '\0') {
    uVar18 = 0;
    iVar14 = (int)(DAT_00732e54 + (unsigned int)param_2);
    cVar12 = *(char *)((unsigned int)iVar14 + 0x216u);
    if (cVar12 == '\x01') {
      iVar16 = (int)((unsigned int)uVar6 * 0xefu + (unsigned int)param_2);
      if ((*(unsigned char *)((unsigned int)iVar14 + 0x21au) < DAT_0052b0aa[iVar16]) ||
          (DAT_0052b0ac[iVar16] < *(unsigned char *)((unsigned int)iVar14 + 0x21au))) {
        *(unsigned char *)((unsigned int)iVar14 + 0x21au) = DAT_0052b0aa[iVar16];
      }
      bVar4 = *(unsigned char *)(DAT_00732e54 + 0x21au + (unsigned int)param_2);
      uStack_e = (unsigned short)bVar4;
      *(unsigned char *)(DAT_00732e54 + 0x21au + (unsigned int)param_2) = (unsigned char)(bVar4 + 1);
      if (DAT_0052b0ac[iVar16] < *(unsigned char *)(DAT_00732e54 + 0x21au + (unsigned int)param_2)) {
        *(unsigned char *)(DAT_00732e54 + 0x21au + (unsigned int)param_2) = DAT_0052b0aa[iVar16];
      }
      local_16 = (unsigned int)1 << 16;
      uStack_c = 1;
      uVar13 = uStack_e;
    }
    else if (cVar12 == '\x02') {
      pbVar15 = (unsigned char *)((unsigned int)iVar14 + 0x21au);
      iVar14 = (int)((unsigned int)uVar6 * 0xefu + (unsigned int)param_2);
      pbVar1 = &DAT_0052b0aa[iVar14];
      if ((*pbVar15 < DAT_0052b0aa[iVar14]) || (DAT_0052b0ac[iVar14] < *pbVar15)) {
        *pbVar15 = *pbVar1;
      }
      pbVar15 = (unsigned char *)(DAT_00732e54 + 0x21au + (unsigned int)param_2);
      bVar4 = *pbVar15;
      *pbVar15 = *pbVar15 ^ 1;
      pbVar15 = (unsigned char *)(DAT_00732e54 + 0x21au + (unsigned int)param_2);
      if (DAT_0052b0ac[iVar14] < *pbVar15) {
        *pbVar15 = *pbVar1;
      }
      uStack_e = (unsigned short)DAT_0052b0ac[iVar14];
      uStack_c = 2;
      local_16 = (unsigned int)(unsigned short)((int)(((unsigned int)uStack_e - (unsigned int)*pbVar1) + 1u) / 2) << 16;
      uVar13 = (unsigned short)bVar4;
    }
    else {
      uVar13 = uStack_c;
      if (cVar12 == '\x03') {
        iVar14 = (int)((unsigned int)param_2 + (unsigned int)uVar6 * 0xefu);
        uStack_c = 1;
        uStack_e = (unsigned short)DAT_0052b0ac[iVar14];
        local_16 = (unsigned int)(unsigned short)((uStack_e - DAT_0052b0aa[iVar14]) + 1) << 16;
        uVar13 = (unsigned short)DAT_0052b0aa[iVar14];
      }
    }
    local_16 = (local_16 & 0xffff0000u) | (unsigned int)uVar13;
    while (uVar13 <= uStack_e) {
      uVar19 = local_16 & 0xffffu;
      if ((*(char *)(DAT_00732e54 + 0x344u + uVar19 * 4u) != '\0') &&
          (cVar12 = *(char *)(DAT_00732e54 + uVar19 * 4u + 0x345u), '\0' < cVar12)) {
        local_12 = (unsigned short)DAT_0052b0a8[(unsigned int)param_2 + (unsigned int)uVar6 * 0xefu];
        if ('?' < cVar12) {
          local_12 = (unsigned short)(local_12 + 1);
        }
        uVar10 = FUN_00447cf0(param_1, uVar19, (unsigned int)local_12);
        if (uVar10 != 0xffffffff) {
          iVar14 = (int)((unsigned int)*(unsigned char *)(DAT_006d4988 + 0x16u + (unsigned int)param_1 * 0x23u) * 0x566u);
          if ((DAT_0070ad44[iVar14] != 0x15) && (DAT_0070b215[iVar14] != 0x15)) {
            if (iVar5 == -1) {
              cVar12 = (char)(*(char *)(DAT_00732e54 + 0x345u + uVar19 * 4u) - 1);
            }
            else {
              uVar11 = FUN_004a7aa0(5);
              if (((unsigned short)uVar11 == uVar6) ||
                  (uVar11 = FUN_004a7aa0(7), (unsigned short)uVar11 == uVar6)) {
                cVar12 = (char)(*(char *)(DAT_00732e54 + 0x345u + uVar19 * 4u) - 3);
              }
              else {
                cVar12 = (char)(*(char *)(DAT_00732e54 + 0x345u + uVar19 * 4u) - 4);
              }
            }
            *(char *)(DAT_00732e54 + 0x345u + uVar19 * 4u) = cVar12;
          }
          if ((local_16 >> 16) < 2) {
            if ((unsigned short)uVar18 < 2) goto LAB_004475f5;
          }
          else if ((uVar18 & 1) == 0) {
LAB_004475f5:
            FUN_0040a3c0(uVar10, iVar5);
          }
          if (*(char *)(DAT_00732e54 + 0x345u + uVar19 * 4u) < '\0') {
            *(char *)(DAT_00732e54 + 0x345u + uVar19 * 4u) = 0;
          }
          iVar14 = (int)((uVar10 - DAT_0071b704) * 8u);
          if (iVar5 == -1) {
            uVar8 = *(unsigned short *)((unsigned int)iVar9 + 0x18u);
          }
          else {
            __builtin_memcpy(&uVar8, &DAT_0071b74a[(unsigned int)iVar5 * 0x5bdu], 2);
          }
          uVar18 = (unsigned int)(unsigned short)((unsigned short)uVar18 + 1);
          *(unsigned short *)((unsigned int)iVar14 + 4u + DAT_006db798) = uVar8;
          *(char *)((unsigned int)iVar14 + DAT_006db798) = (char)iVar5;
        }
      }
      if ((unsigned short)((local_16 >> 16) - 1) == 0) break;
      uVar13 = (unsigned short)(((unsigned short)local_16) + uStack_c);
      local_16 = ((unsigned int)(unsigned short)((local_16 >> 16) - 1) << 16) | (unsigned int)uVar13;
    }
    sVar17 = (short)uVar18;
    if ((local_12 == 0x8d) || (local_12 == 0x8e)) {
      *(short *)(DAT_00732e54 + 0x246u) = (short)(*(short *)(DAT_00732e54 + 0x246u) + sVar17);
      if (iVar5 != -1) {
        __builtin_memcpy(&uVar8, &DAT_0071b7b9[(unsigned int)iVar5 * 0x5bdu], 2);
        uVar8 = (unsigned short)(uVar8 + sVar17);
        __builtin_memcpy(&DAT_0071b7b9[(unsigned int)iVar5 * 0x5bdu], &uVar8, 2);
      }
    }
    else {
      *(short *)(DAT_00732e54 + 0x242u) = (short)(*(short *)(DAT_00732e54 + 0x242u) + sVar17);
      if (iVar5 != -1) {
        __builtin_memcpy(&uVar8, &DAT_0071b7b5[(unsigned int)iVar5 * 0x5bdu], 2);
        uVar8 = (unsigned short)(uVar8 + sVar17);
        __builtin_memcpy(&DAT_0071b7b5[(unsigned int)iVar5 * 0x5bdu], &uVar8, 2);
      }
    }
    psVar2 = (short *)(DAT_00732e54 + 0x21cu + (unsigned int)param_2 * 2u);
    *psVar2 = (short)(*psVar2 + sVar17 * 0x2f + 2);
    *(unsigned int *)(DAT_00732e54 + 0x220u + (unsigned int)param_2 * 4u) =
        *(unsigned int *)(DAT_00732e54 + 0x220u + (unsigned int)param_2 * 4u) + uVar18 * 0x2fu + 2u;
  }
  else if (iVar5 == DAT_0071ecf4) {
    FUN_004a1d70(0x81, DAT_0071ecf4);
    return;
  }
  return;
}
