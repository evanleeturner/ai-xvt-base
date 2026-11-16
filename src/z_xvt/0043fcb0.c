/* SPDX-License-Identifier: MIT */
/* z_xvt 0043fcb0 */
extern char DAT_0071b743[];
extern unsigned int DAT_0071ecf4;
extern short DAT_006559b0[];
extern unsigned short DAT_0052f0e4;
extern unsigned short DAT_0052f0e0;
extern short DAT_006559ba;
extern short DAT_006559b8;

void __cdecl FUN_0043fcb0(unsigned short param_1)
{
  short *psVar1;
  unsigned short uVar2;
  unsigned short local_2;
  
  if (DAT_0071b743[DAT_0071ecf4 * 0x5bd] == '\0') {
    psVar1 = DAT_006559b0 + param_1;
    if (*psVar1 == 0) {
      *psVar1 = 1;
      local_2 = DAT_0052f0e4;
      DAT_0052f0e4 = DAT_0052f0e0;
      DAT_0052f0e0 = param_1;
    }
    else {
      *psVar1 = -2;
    }
    if ((DAT_006559ba != 0) && (DAT_006559b8 != 0)) {
      if (param_1 == 5) {
        DAT_006559b8 = -2;
      }
      else {
        DAT_006559ba = -2;
      }
      if ((local_2 != 5) && (local_2 != 4)) {
        DAT_0052f0e4 = local_2;
        return;
      }
    }
  }
  else {
    if (param_1 != 2) {
      uVar2 = 0;
      do {
        if ((((uVar2 != param_1) && (uVar2 != 5)) && (uVar2 != 2)) && (DAT_006559b0[uVar2] != 0))
        {
          DAT_006559b0[uVar2] = -2;
        }
        uVar2 = uVar2 + 1;
      } while (uVar2 < 8);
    }
    psVar1 = DAT_006559b0 + param_1;
    if (*psVar1 != 0) {
      *psVar1 = -2;
      return;
    }
    *psVar1 = 1;
    DAT_0052f0e4 = DAT_0052f0e0;
    DAT_0052f0e0 = param_1;
  }
  return;
}
