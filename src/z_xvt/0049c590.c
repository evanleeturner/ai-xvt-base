/* SPDX-License-Identifier: MIT */
/* z_xvt 0049c590 */
extern unsigned short DAT_0071f714;
extern int DAT_006d4988;
extern int DAT_0071ecf4;
extern int DAT_0071bb1b;
extern int DAT_0071bb1f;
extern int DAT_0071bb23;

extern unsigned int __cdecl FUN_00413cc0(int, int, int, int, int, int);
extern unsigned int __cdecl FUN_00419100(unsigned int, unsigned int);
extern void __cdecl FUN_004832a0(int, int, int, int, int);
extern void __cdecl FUN_00486310(int);

void __cdecl FUN_0049c590(unsigned short param_1)
{
  short sVar1;
  int iVar2;
  unsigned int uVar3;
  unsigned int uVar4;
  unsigned int uVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int local_int_1f;
  short local_short_97;
  short local_short_99;
  short local_short_9b;
  short local_short_9d;
  short local_short_9f;
  short local_short_a1;
  
  DAT_0071f714 = param_1;
  iVar8 = (unsigned int)param_1 * 0x23 + DAT_006d4988;
  iVar9 = *(int *)((char *)&DAT_0071bb1b + DAT_0071ecf4 * 0x5bd) - *(int *)(iVar8 + 4);
  __builtin_memcpy(&local_int_1f, (void *)(iVar8 + 0x1f), 4);
  iVar2 = local_int_1f;
  iVar6 = *(int *)((char *)&DAT_0071bb1f + DAT_0071ecf4 * 0x5bd) - *(int *)(iVar8 + 8);
  iVar7 = *(int *)((char *)&DAT_0071bb23 + DAT_0071ecf4 * 0x5bd) - *(int *)(iVar8 + 0xc);
  __builtin_memcpy(&local_short_97, (void *)(iVar2 + 0x97), 2);
  __builtin_memcpy(&local_short_99, (void *)(iVar2 + 0x99), 2);
  __builtin_memcpy(&local_short_9b, (void *)(iVar2 + 0x9b), 2);
  uVar3 = FUN_00413cc0((int)local_short_97,(int)local_short_99,
                       (int)local_short_9b,iVar9,iVar6,iVar7);
  __builtin_memcpy(&local_int_1f, (void *)(iVar8 + 0x1f), 4);
  iVar2 = local_int_1f;
  __builtin_memcpy(&local_short_9d, (void *)(iVar2 + 0x9d), 2);
  __builtin_memcpy(&local_short_9f, (void *)(iVar2 + 0x9f), 2);
  __builtin_memcpy(&local_short_a1, (void *)(iVar2 + 0xa1), 2);
  uVar4 = FUN_00413cc0((int)local_short_9d,(int)local_short_9f,
                       (int)local_short_a1,iVar9,iVar6,iVar7);
  sVar1 = *(short *)(iVar8 + 0x14);
  uVar5 = FUN_00419100(uVar4,uVar3);
  *(short *)(iVar8 + 0x14) = (short)uVar5 + sVar1;
  *(short *)(iVar8 + 0x14) = *(short *)(iVar8 + 0x14) + -0x4000;
  __builtin_memcpy(&local_int_1f, (void *)(iVar8 + 0x1f), 4);
  *(unsigned char *)(local_int_1f + 0x90) = 1;
  FUN_004832a0(*(short *)(iVar8 + 0x14),*(short *)(iVar8 + 0x12),*(short *)(iVar8 + 0x10),0,iVar8);
  FUN_00486310(iVar8);
  *(short *)(iVar8 + 0x14) = sVar1;
  __builtin_memcpy(&local_int_1f, (void *)(iVar8 + 0x1f), 4);
  *(unsigned char *)(local_int_1f + 0x90) = 1;
  return;
}
