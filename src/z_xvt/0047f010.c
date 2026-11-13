/* SPDX-License-Identifier: MIT */
/* z_xvt 0047f010 */
extern int DAT_006da1a0;
extern char **DAT_006a89e0;
extern char DAT_0070ab20;

void __cdecl FUN_0047ea40(char *param_1);
void __cdecl FUN_0048fce0(char *param_1, int param_2);

void __cdecl FUN_0047f010(unsigned short param_1)
{
  char cVar1;
  char *pcVar2;
  unsigned short uVar3;
  unsigned short uVar4;
  unsigned int uVar5;
  char local_80[128];
  
  if (param_1 < 4) {
    if (DAT_006da1a0 == 0) {
      local_80[0] = '\0';
    }
    else {
      pcVar2 = (char *)(&DAT_006a89e0)[param_1];
      FUN_0047ea40(pcVar2);
      uVar5 = 0;
      do {
        cVar1 = pcVar2[uVar5];
        local_80[uVar5] = cVar1;
        if (cVar1 == '\0') break;
        uVar3 = (short)uVar5 + 1;
        uVar5 = (unsigned int)uVar3;
      } while (uVar3 < 0x80);
      if (param_1 == 1) {
        uVar4 = (unsigned short)uVar5;
        uVar3 = 0;
        while (uVar4 < 0x80) {
          cVar1 = *(char *)((int)&DAT_0070ab20 + (unsigned int)uVar3);
          local_80[uVar5] = cVar1;
          if (cVar1 == '\0') break;
          uVar4 = (short)uVar5 + 1;
          uVar5 = (unsigned int)uVar4;
          uVar3 = uVar3 + 1;
        }
        local_80[uVar5] = '\n';
        local_80[(unsigned short)((short)uVar5 + 1)] = '\0';
      }
    }
  }
  FUN_0048fce0(local_80, -0xff - (unsigned int)param_1);
  return;
}
