/* SPDX-License-Identifier: MIT */
/* z_xvt 0048ee20 */
extern int DAT_0052552c;
extern unsigned char DAT_006d4998;
extern unsigned char DAT_006d4a4e;
extern int DAT_006da1a0;
extern int DAT_00709ff4;
extern char DAT_0070a188;
extern short DAT_0070acc2;
extern int DAT_0071b720;
extern unsigned char DAT_0071f2e2;
extern int DAT_00732a10;

void __cdecl FUN_0048ee20(char param_1)
{
  DAT_0070a188 = param_1;
  if (param_1 == '\0') {
    if (DAT_0052552c != 0x13) {
      if (DAT_0052552c == 0x101) {
        DAT_006d4a4e = 8;
        DAT_006d4998 = 1;
        DAT_0071f2e2 = 4;
        DAT_0070acc2 = 0x42;
        DAT_00709ff4 = DAT_006da1a0;
        return;
      }
      if (DAT_0052552c != 0x112) {
        return;
      }
    }
    DAT_006d4a4e = 5;
    DAT_006d4998 = 0;
    DAT_0071f2e2 = 3;
    DAT_0070acc2 = 0x2a;
    DAT_00709ff4 = DAT_00732a10;
    return;
  }
  if (param_1 == '\x01') {
    if (DAT_0052552c == 0x13) {
      DAT_006d4a4e = 5;
      DAT_006d4998 = 0;
      DAT_0071f2e2 = 3;
      DAT_0070acc2 = 0x2a;
      DAT_00709ff4 = DAT_00732a10;
      return;
    }
    if (DAT_0052552c == 0x101) {
      DAT_006d4a4e = 10;
      DAT_006d4998 = 1;
      DAT_0071f2e2 = 5;
      DAT_0070acc2 = 0x52;
      DAT_00709ff4 = DAT_0071b720;
      return;
    }
    if (DAT_0052552c == 0x112) {
      DAT_006d4a4e = 8;
      DAT_0071f2e2 = 4;
      DAT_0070acc2 = 0x42;
      DAT_00709ff4 = DAT_006da1a0;
      return;
    }
    return;
  }
  if (param_1 != '\x02') {
    return;
  }
  if (DAT_0052552c != 0x13) {
    if (DAT_0052552c == 0x101) {
      DAT_006d4a4e = 10;
      DAT_006d4998 = 1;
      DAT_0071f2e2 = 5;
      DAT_0070acc2 = 0x52;
      DAT_00709ff4 = DAT_0071b720;
      return;
    }
    if (DAT_0052552c == 0x112) {
      DAT_0071f2e2 = 4;
      DAT_0070acc2 = 0x42;
      DAT_00709ff4 = DAT_006da1a0;
      DAT_006d4a4e = 8;
      return;
    }
    return;
  }
  DAT_006d4a4e = 5;
  DAT_006d4998 = 0;
  DAT_0071f2e2 = 3;
  DAT_0070acc2 = 0x2a;
  DAT_00709ff4 = DAT_00732a10;
  return;
}
