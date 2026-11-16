/* SPDX-License-Identifier: MIT */
/* z_xvt 00413010 */
extern int DAT_006d4988;
extern int DAT_006d4980;
extern int DAT_006d4974;
extern int DAT_006d4978;
extern int DAT_006d4970;
extern int DAT_0070a180;
extern int DAT_0070acd8;
extern int DAT_00703ad8;
extern int DAT_0071e6c4;
extern int DAT_00732f38;
extern int DAT_00732fe8;
extern int DAT_0073294c;
extern int DAT_007329b0;
extern int DAT_007329b4;
extern int DAT_00732a2c;
extern int DAT_00732a28;
extern int DAT_00732a24;
extern unsigned char DAT_0052686c[];

unsigned int __cdecl FUN_0042b150(int param_1);
void __cdecl FUN_00421c70(unsigned int param_1, int param_2);
unsigned int __cdecl FUN_004127d0(unsigned int param_1, unsigned int param_2, unsigned int param_3);
unsigned int __cdecl FUN_0040fb70(int param_1);
unsigned int __cdecl FUN_00419190(unsigned short param_1, unsigned short param_2);

unsigned int __cdecl FUN_00413010(unsigned short param_1, unsigned short param_2)
{
  char cVar1;
  unsigned char bVar2;
  int iVar3;
  int iVar4;
  unsigned int uVar5;
  unsigned int uVar6;
  unsigned short uVar7;
  unsigned int uVar8;
  unsigned int uVar9;
  unsigned int uVar10;
  unsigned int uVar11;
  unsigned int uVar12;
  unsigned int uVar13;
  unsigned int uVar14;
  int pTmp;
  int iTmp;
  unsigned short sTmp;
  unsigned int uTmp;

  uVar6 = (unsigned int)param_2;
  __builtin_memcpy(&pTmp, (unsigned char *)(unsigned int)(((unsigned int)param_1 * 0x23u + 0x1fu) + (unsigned int)DAT_006d4988), 4);
  __builtin_memcpy(&sTmp, (unsigned char *)(unsigned int)((unsigned int)pTmp + 0x83u), 2);
  uVar5 = (unsigned int)sTmp;
  if (uVar6 <= uVar5) {
    return (unsigned int)DAT_006d4988 & 0xffff0000u;
  }
  iVar3 = (int)(uVar6 * 0x23u);
  cVar1 = *(char *)(unsigned int)((unsigned int)DAT_006d4988 + 2u + (unsigned int)iVar3);
  if (cVar1 == '\x0e') {
    return 0;
  }
  if (cVar1 == '\v') {
    return 0;
  }
  if (((cVar1 != '\n') && ((int)uVar5 < DAT_006d4980)) &&
      (iVar4 = (int)(uVar5 * 0x23u + (unsigned int)DAT_006d4988),
       (__builtin_memcpy(&iTmp, (unsigned char *)(unsigned int)((unsigned int)iVar4 + 0x1bu), 4), iTmp == -1))) {
    __builtin_memcpy(&pTmp, (unsigned char *)(unsigned int)((unsigned int)iVar4 + 0x1fu), 4);
    __builtin_memcpy(&iTmp, (unsigned char *)(unsigned int)((unsigned int)pTmp + 0xa7u), 4);
    uVar5 = FUN_0042b150(iTmp);
    if (*(unsigned short *)(unsigned int)(uVar5 + 0x18u) != param_2) {
      return uVar5 & 0xffff0000u;
    }
  }
  bVar2 = *(unsigned char *)(unsigned int)((unsigned int)DAT_006d4988 + 3u + (unsigned int)iVar3);
  FUN_00421c70(uVar6, 0);
  DAT_0070a180 = DAT_00732a2c;
  DAT_0073294c = DAT_00732a2c;
  DAT_0070acd8 = DAT_00732a28;
  DAT_007329b0 = DAT_00732a28;
  DAT_00703ad8 = DAT_00732a24;
  DAT_007329b4 = DAT_00732a24;
  uVar6 = (unsigned int)DAT_006d4974 - (unsigned int)DAT_00732a2c;
  uVar5 = (unsigned int)*(unsigned short *)(&DAT_0052686c[0] + (unsigned int)bVar2 * 0x16u);
  if ((int)uVar6 < 0) {
    uVar6 = 0u - uVar6;
  }
  if (0x20000 < (int)uVar6) {
    return uVar6 & 0xffff0000u;
  }
  uVar8 = (unsigned int)DAT_006d4978 - (unsigned int)DAT_00732a28;
  if ((int)uVar8 < 0) {
    uVar8 = 0u - uVar8;
  }
  uVar7 = (unsigned short)(uVar6 >> 0x10);
  if (0x20000 < (int)uVar8) {
    return (unsigned int)uVar7 << 0x10;
  }
  uVar10 = (unsigned int)DAT_006d4970 - (unsigned int)DAT_00732a24;
  if ((int)uVar10 < 0) {
    uVar10 = 0u - uVar10;
  }
  if (0x20000 < (int)uVar10) {
    return (unsigned int)uVar7 << 0x10;
  }
  uVar6 = FUN_004127d0(uVar6, uVar8, uVar10);
  if (0x20000 < (int)uVar6) {
    return uVar6 & 0xffff0000u;
  }
  uVar9 = (unsigned int)DAT_006d4974 - (unsigned int)DAT_0071e6c4;
  if ((int)uVar9 < 0) {
    uVar9 = 0u - uVar9;
  }
  uVar11 = (unsigned int)DAT_006d4978 - (unsigned int)DAT_00732f38;
  if ((int)uVar11 < 0) {
    uVar11 = 0u - uVar11;
  }
  uVar12 = (unsigned int)DAT_006d4970 - (unsigned int)DAT_00732fe8;
  if ((int)uVar12 < 0) {
    uVar12 = 0u - uVar12;
  }
  uVar13 = (unsigned int)DAT_0073294c - (unsigned int)DAT_0070a180;
  if ((int)uVar13 < 0) {
    uVar13 = 0u - uVar13;
  }
  uVar14 = (unsigned int)DAT_007329b0 - (unsigned int)DAT_0070acd8;
  if ((int)uVar14 < 0) {
    uVar14 = 0u - uVar14;
  }
  uTmp = (unsigned int)DAT_007329b4 - (unsigned int)DAT_00703ad8;
  if ((int)uTmp < 0) {
    uTmp = 0u - uTmp;
  }
  uVar8 = FUN_004127d0(uVar9 + uVar13 + uVar5, uVar11 + uVar14 + uVar5, uVar12 + uTmp + uVar5);
  if (uVar8 < uVar6) {
    return uVar8 & 0xffff0000u;
  }
  if ((uVar5 < 0x447u) && (*(char *)(unsigned int)((unsigned int)DAT_006d4988 + 3u + (unsigned int)iVar3) != ':')) {
    uVar6 = FUN_0040fb70((int)((((int)uVar5 >> 3) + ((int)uVar5 >> 2))));
    return uVar6;
  }
  uVar6 = FUN_00419190(param_1, param_2);
  return uVar6;
}
