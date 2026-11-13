/* SPDX-License-Identifier: MIT */
/* z_bop 004195a0 */
extern unsigned int DAT_00550b8c;
extern unsigned int DAT_0051bf40;
extern unsigned int *DAT_00550880;
extern unsigned int *DAT_00550b88;
extern unsigned int DAT_00550bd8;
extern unsigned int DAT_00a07cc8;
extern unsigned int DAT_00550b98;
extern unsigned int DAT_00550840;
extern unsigned int DAT_0052341c;

void __cdecl FUN_00417450(void);
void __cdecl FUN_004650e0(unsigned int *param_1, unsigned int *param_2, unsigned int param_3);
void __stdcall FUN_00419620(void);
void __cdecl FUN_004197d0(void);

void __cdecl FUN_004195a0(unsigned int param_1, unsigned int param_2)
{
  unsigned int uVar1;
  unsigned int *puVar2;
  unsigned int *puVar3;
  
  DAT_00550b8c = param_1;
  DAT_0051bf40 = 1;
  puVar2 = DAT_00550880;
  puVar3 = DAT_00550b88;
  for (uVar1 = param_1 >> 2; uVar1 != 0; uVar1 = uVar1 - 1) {
    *puVar3 = *puVar2;
    puVar2 = puVar2 + 1;
    puVar3 = puVar3 + 1;
  }
  for (uVar1 = param_1 & 3; uVar1 != 0; uVar1 = uVar1 - 1) {
    *(unsigned char *)puVar3 = *(unsigned char *)puVar2;
    puVar2 = (unsigned int *)((int)puVar2 + 1);
    puVar3 = (unsigned int *)((int)puVar3 + 1);
  }
  DAT_00550bd8 = DAT_00550b8c;
  DAT_00a07cc8 = param_2;
  FUN_00417450();
  FUN_004650e0(&DAT_00550b98, &DAT_00550840, 0x10);
  FUN_00419620();
  DAT_0052341c = 0;
  FUN_004197d0();
  return;
}
