/* SPDX-License-Identifier: MIT */
/* z_xvt 00433340 */
extern uint DAT_005251b0;
extern int DAT_005251b4;
extern char DAT_00539a17;
extern char DAT_00539a16;
extern uint DAT_00739290;
extern int DAT_00733798;
extern uint DAT_00539a08;
extern char DAT_00539a15;

uint __cdecl FUN_00433340(uint param_1, uint param_2, int param_3, int param_4, uint param_5)
{
  uint uVar1;
  int iVar2;
  uint uVar3;
  int iVar4;
  uint uVar5;
  int *piVar6;
  
  uVar1 = DAT_005251b0;
  iVar4 = DAT_005251b4;
  do {
    if (iVar4 == 0) {
      return 0xffffffff;
    }
    iVar2 = (int)(uVar1 * 0x210);
    if ((&DAT_00539a17)[iVar2] != '\0') {
      uVar5 = (uint)(unsigned char)(&DAT_00539a16)[iVar2];
      uVar3 = 0;
      if (DAT_00739290 != 0) {
        piVar6 = &DAT_00733798;
        do {
          if ((uint)*piVar6 == (&DAT_00539a08)[uVar1 * 0x84]) break;
          piVar6 = piVar6 + 0x8e;
          uVar3 = uVar3 + 1;
        } while (uVar3 < DAT_00739290);
      }
      if (uVar3 == param_5) {
        if (param_3 == 0) {
          if (param_4 == 0) {
            if ((((&DAT_00539a15)[iVar2] != '\0') && ((&DAT_00539a15)[iVar2] != '\x02')) &&
               (uVar5 == param_2)) {
              return uVar1;
            }
          }
          else if (((&DAT_00539a15)[iVar2] == '\x02') && (uVar5 == param_2)) {
            return uVar1;
          }
        }
        else if (((&DAT_00539a15)[iVar2] == '\0') && (uVar5 == param_2)) {
          return uVar1;
        }
      }
    }
    uVar1 = uVar1 + 1;
    if (0x3ff < uVar1) {
      uVar1 = 0;
    }
    iVar4 = iVar4 + -1;
  } while( 1 );
}
