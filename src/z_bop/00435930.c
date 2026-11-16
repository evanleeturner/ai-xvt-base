/* SPDX-License-Identifier: MIT */
/* z_bop 00435930 */
extern char DAT_009d8e27;
extern char DAT_009d8e29;
extern int DAT_009d6968;
extern byte DAT_009ed22c;
extern byte DAT_009ed22d;
extern byte DAT_009ed228__3_1_;

uint __cdecl FUN_00435a10(byte, byte, byte);

void __cdecl FUN_00435930(ushort param_1, uint param_2, int param_3, byte param_4)
{
  int iVar1;
  uint uVar2;
  char *pcVar3;
  int iVar4;
  char *pcVar5;
  int local_4;
  
  iVar1 = (param_2 & 0xffff) * 0x566;
  iVar4 = 8;
  pcVar5 = &DAT_009d8e29 + iVar1 + (uint)param_4;
  pcVar3 = &DAT_009d8e27 + iVar1;
  do {
    if (pcVar3[-2] == '\x02') {
      if ((byte)pcVar3[-1] == param_1) {
        if ((*pcVar3 == '\x12') || ((*pcVar3 == '\x13' && (param_3 == 1)))) {
          uVar2 = FUN_00435a10(DAT_009ed228__3_1_, DAT_009ed22c, DAT_009ed22d);
          if (((uint)(byte)pcVar3[0xc] * 5 == 0) || (uVar2 <= (uint)(byte)pcVar3[0xc] * 5)) {
            local_4 = (int)pcVar3[1];
          }
          local_4 = local_4 * 0xfa;
          if (*pcVar5 != '\0') {
            (&DAT_009d6968)[param_4] = (&DAT_009d6968)[param_4] + local_4;
          }
        }
      }
    }
    pcVar5 = pcVar5 + 0x4e;
    pcVar3 = pcVar3 + 0x4e;
    iVar4 = iVar4 + -1;
  } while (iVar4 != 0);
  return;
}
