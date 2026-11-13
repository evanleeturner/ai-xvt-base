/* SPDX-License-Identifier: MIT */
/* z_bop 0042b380 */
extern unsigned short DAT_009a8078;
extern unsigned short DAT_009ed664;
extern unsigned short DAT_009d8110;
extern unsigned int DAT_009a1fe8;
extern unsigned int DAT_009a1fe6;
extern int DAT_00a00870[];
extern int DAT_00a00874[];
extern short DAT_009cc450;
extern unsigned short DAT_009a20a6;
extern unsigned int unaff_EBP;

void __cdecl FUN_0041fc20(unsigned int param_1);
void __cdecl FUN_00472400(unsigned int param_1);
int __cdecl FUN_004adc40(int param_1);
int __cdecl FUN_004ae2a0(int param_1, int param_2);
void __cdecl FUN_004728d0(unsigned int param_1, unsigned int param_2);

void __cdecl FUN_0042b380(unsigned short param_1)
{
  unsigned char bVar1;
  unsigned char uVar2;
  int iVar3;
  short sVar4;
  int iVar5;
  unsigned int uVar6;
  int iVar7;
  unsigned int uVar8;
  unsigned int uVar9;

  if (param_1 != DAT_009a8078) {
    if ((unsigned int)DAT_009a8078 - (unsigned int)param_1 != 0xFFFFFFFF) {
      DAT_009ed664 = param_1 + 0x7000;
      goto LAB_0042b3f8;
    }
    if (param_1 < DAT_009a8078) {
      DAT_009d8110 = param_1;
    }
  }
  FUN_0041fc20(((unsigned int)(unaff_EBP >> 0x10) << 16) | param_1);
  FUN_00472400(DAT_009a1fe8 + (unsigned int)param_1 * 0x23);
LAB_0042b3f8:
  iVar3 = (unsigned int)param_1 * 0x23;
  DAT_009a1fe6 = 0;
  bVar1 = *(unsigned char *)(DAT_009a1fe8 + 3 + iVar3);
  if (bVar1 < 0x49) {
    iVar5 = DAT_00a00870[(unsigned int)bVar1 * 0x65];
  }
  else {
    iVar5 = FUN_004adc40((unsigned int)bVar1);
  }
  uVar9 = 0;
  if (0 < iVar5) {
    do {
      DAT_009a1fe6 = DAT_009a1fe6 + 1;
      bVar1 = *(unsigned char *)(DAT_009a1fe8 + 3 + iVar3);
      uVar6 = (unsigned int)bVar1;
      if (bVar1 < 0x49) {
        uVar8 = uVar9;
        if (DAT_00a00870[uVar6 * 0x65] <= (int)uVar9) {
          uVar8 = DAT_00a00870[uVar6 * 0x65] - 1;
        }
        iVar7 = DAT_00a00874[uVar8 + uVar6 * 0x65];
      }
      else {
        iVar7 = FUN_004ae2a0(uVar6, uVar9);
      }
    } while ((iVar7 != 1) && (uVar9 = (unsigned int)(unsigned short)((short)uVar9 + 1), (int)uVar9 < iVar5));
  }
  sVar4 = DAT_009cc450;
  DAT_009a1fe6 = DAT_009a1fe6 - 1;
  if (param_1 < DAT_009a8078) {
    DAT_009a20a6 = 1;
    DAT_009cc450 = DAT_009ed664;
  }
  uVar9 = (unsigned int)DAT_009a1fe6;
  iVar5 = *(int *)(DAT_009a1fe8 + 0x1f + iVar3);
  uVar2 = *(unsigned char *)(iVar5 + 0x8a);
  *(unsigned char *)(iVar5 + 0x8a) = 1;
  FUN_004728d0(DAT_009a1fe8 + iVar3, uVar9);
  *(unsigned char *)(*(int *)(DAT_009a1fe8 + 0x1f + iVar3) + 0x8a) = uVar2;
  DAT_009cc450 = sVar4;
  return;
}
