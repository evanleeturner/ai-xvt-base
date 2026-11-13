/* SPDX-License-Identifier: MIT */
/* z_xvt 00442720 */
extern unsigned int DAT_00732910;
extern char DAT_0071b749;
extern unsigned int DAT_0071ecf4;
extern int DAT_00732a2c;
extern char DAT_0071bb1b;
extern int DAT_00732a28;
extern char DAT_0071bb1f;
extern int DAT_00732a24;
extern char DAT_0071bb23;
extern unsigned int DAT_00732f3c;
extern unsigned int DAT_0052552c;
extern unsigned int DAT_006da538;
extern char DAT_0071b743;

// Prototypes for called functions
void __cdecl FUN_00421c70(unsigned int param_1, int param_2);
unsigned int __cdecl FUN_0040ca60(int param_1, int param_2, int param_3);
unsigned int __cdecl FUN_0040ca00(int param_1, int param_2, int param_3);
unsigned int __cdecl FUN_0040ca30(int param_1, int param_2, int param_3);
int __cdecl FUN_0040ca90(unsigned int param_1, unsigned int param_2);
int __cdecl FUN_0040cad0(unsigned int param_1, unsigned int param_2);
unsigned int __cdecl FUN_004428e0(unsigned int param_1);
void __cdecl FUN_004354e0(int param_1, int param_2, unsigned int param_3, unsigned int param_4, unsigned char param_5);
void __cdecl FUN_00477c90(int param_1, int param_2, unsigned int param_3, unsigned int param_4, unsigned int param_5, unsigned int param_6);

void __cdecl FUN_00442720(unsigned short param_1, unsigned char param_2)
{
  unsigned int uVar1;
  unsigned int uVar2;
  unsigned int uVar3;
  int iVar4;
  unsigned int uVar5;
  int iVar6;
  int iVar7;
  int local_bb1b;
  int local_bb1f;
  int local_bb23;

  if (((param_1 != 0xffff) && (DAT_00732910 == 0)) &&
     ((&DAT_0071b749)[DAT_0071ecf4 * 0x5bd] != '\0')) {
    FUN_00421c70((unsigned int)param_1, 0);
    __builtin_memcpy(&local_bb1b, (void*)((unsigned char*)&DAT_0071bb1b + DAT_0071ecf4 * 0x5bd), sizeof(int));
    __builtin_memcpy(&local_bb1f, (void*)((unsigned char*)&DAT_0071bb1f + DAT_0071ecf4 * 0x5bd), sizeof(int));
    __builtin_memcpy(&local_bb23, (void*)((unsigned char*)&DAT_0071bb23 + DAT_0071ecf4 * 0x5bd), sizeof(int));
    iVar6 = DAT_00732a2c - local_bb1b;
    iVar7 = DAT_00732a28 - local_bb1f;
    iVar4 = DAT_00732a24 - local_bb23;
    uVar1 = FUN_0040ca60(iVar6, iVar7, iVar4);
    if (0 < (int)uVar1) {
      uVar2 = FUN_0040ca00(iVar6, iVar7, iVar4);
      uVar3 = FUN_0040ca30(iVar6, iVar7, iVar4);
      iVar4 = FUN_0040ca90(uVar2, uVar1);
      iVar6 = FUN_0040cad0(uVar3, uVar1);
      uVar2 = FUN_004428e0((unsigned int)param_1);
      uVar2 = (unsigned int)((unsigned int)DAT_00732f3c * uVar2) / uVar1;
      if (DAT_0052552c == 0x13) {
        uVar3 = 4;
      }
      else {
        uVar3 = 8;
      }
      uVar5 = (DAT_006da538 >> 1) + (DAT_006da538 >> 2);
      if ((int)uVar2 < (int)uVar3) {
        uVar2 = uVar3;
      }
      if ((int)uVar5 < (int)uVar2) {
        uVar2 = uVar5;
      }
      uVar2 = uVar2 + 4;
      if ((&DAT_0071b743)[DAT_0071ecf4 * 0x5bd] != '\0') {
        FUN_004354e0(iVar4 - (int)uVar2 / 2, iVar6 - (int)uVar2 / 2, uVar2, uVar2, param_2);
        return;
      }
      FUN_00477c90(iVar4 - (int)uVar2 / 2, iVar6 - (int)uVar2 / 2, uVar2, uVar2, (unsigned int)param_2, uVar1);
    }
  }
  return;
}
