/* SPDX-License-Identifier: MIT */
/* z_bop 004f02f0 */
extern int DAT_00aa6090;
extern int DAT_00aa6094;
extern char DAT_00aa6150[];
extern int DAT_00aa6cfc;
extern char *DAT_0052c818;
extern int DAT_0052c80c;
extern int DAT_00b69cb0;
extern int DAT_00b69cbc;

int FUN_004d11e0(void);
char *FUN_004ddc10(int param_1);
unsigned int FUN_004d6e30(int param_1, int param_2, int param_3);
unsigned int FUN_004d1400(void);
int FUN_004d48e0(void);
void FUN_004d67e0(int *param_1, int param_2, int param_3, short param_4, int param_5);
void FUN_004db640(int param_1, char *param_2, int *param_3, unsigned int param_4);
void FUN_004d4970(void);
void FUN_004eecb0(int param_1, int param_2, int param_3, int param_4);

void __stdcall FUN_004f02f0(void)
{
  char cVar1;
  int iVar2;
  unsigned int uVar3;
  char *piVar4;
  unsigned int uVar5;
  unsigned int uVar6;
  int *piVar7;
  char *pcVar8;
  int iVar9;
  char *pcVar10;
  char *pcVar11;
  unsigned int local_8c;
  struct {
    int local_88;
    int local_84;
    int local_80;
    int local_7c;
  } local_rect;
  unsigned int local_78;
  unsigned int local_74;
  unsigned int local_70;
  char *local_6c[2];
  char local_64[100];
  int iVar_cmp;
  unsigned int uDword;

  piVar7 = &DAT_00aa6090;
  iVar9 = 0;
  do {
    iVar2 = FUN_004d11e0();
    if (iVar2 == *piVar7) break;
    piVar7 = piVar7 + 2;
    iVar9 = iVar9 + 1;
  } while (piVar7 < (int *)0xaa60d0);
  if ((&DAT_00aa6094)[iVar9 * 2] != 1) {
    return;
  }
  local_rect.local_88 = 0;
  local_rect.local_84 = 0;
  local_rect.local_80 = (int)(*(unsigned int *)(DAT_0052c818 + 4) - 1u);
  local_rect.local_7c = (int)((unsigned int)DAT_0052c80c - 1u);
  local_6c[0] = FUN_004ddc10(0x324);
  local_6c[1] = FUN_004ddc10(0x325);
  local_70 = FUN_004d6e30(0xff, 0xff, 0xff);
  local_78 = *(unsigned int *)(DAT_0052c818 + 4) / 0x14;
  local_74 = (*(unsigned int *)(DAT_0052c818 + 4) + local_78 * 0xFFFFFFFEu) >> 2;
  uVar3 = FUN_004d1400();
  DAT_00aa6cfc = FUN_004d48e0();
  iVar2 = -1;
  iVar9 = (int)FUN_004d6e30(0, 0, 0);
  piVar7 = &DAT_00aa6090;
  FUN_004d67e0(&local_rect.local_88, 0, 0, (short)iVar9, iVar2);
  local_8c = 0;
LAB_004f03dc:
  if (*piVar7 != 0) {
    local_rect.local_88 = (int)((local_8c & 3) * local_74 + local_78);
    local_rect.local_84 = (int)((local_8c >> 2) * (unsigned int)(DAT_0052c80c >> 1));
    local_rect.local_80 = (int)((unsigned int)local_rect.local_88 + local_74);
    local_rect.local_7c = (int)((unsigned int)local_rect.local_84 + (unsigned int)(DAT_0052c80c >> 1));
    uVar6 = 0;
    if (uVar3 != 0) {
      piVar4 = DAT_00aa6150 + 0xe;
      do {
        __builtin_memcpy(&iVar_cmp, piVar4, 4);
        if (iVar_cmp == *piVar7) {
          uVar5 = 0xffffffff;
          pcVar8 = DAT_00aa6150 + uVar6 * 0x2a;
          goto code_r0x004f0455;
        }
        piVar4 = piVar4 + 0x2a;
        uVar6 = uVar6 + 1;
      } while (uVar6 < uVar3);
    }
    goto LAB_004f046f;
  }
  goto LAB_004f04bc;
  while (1) {
    uVar5 = uVar5 - 1;
    pcVar11 = pcVar8 + 1;
    cVar1 = *pcVar8;
    pcVar8 = pcVar11;
    if (cVar1 == '\0') break;
code_r0x004f0455:
    pcVar11 = pcVar8;
    if (uVar5 == 0) break;
  }
  uVar5 = ~uVar5;
  pcVar8 = pcVar11 - uVar5;
  pcVar11 = local_64;
  for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
    __builtin_memcpy(&uDword, pcVar8, 4);
    __builtin_memcpy(pcVar11, &uDword, 4);
    pcVar8 = pcVar8 + 4;
    pcVar11 = pcVar11 + 4;
  }
  for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
    *pcVar11 = *pcVar8;
    pcVar8 = pcVar8 + 1;
    pcVar11 = pcVar11 + 1;
  }
LAB_004f046f:
  uVar6 = 0xffffffff;
  pcVar8 = local_6c[piVar7[1]];
  do {
    pcVar11 = pcVar8;
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    pcVar11 = pcVar8 + 1;
    cVar1 = *pcVar8;
    pcVar8 = pcVar11;
  } while (cVar1 != '\0');
  uVar6 = ~uVar6;
  iVar9 = -1;
  pcVar8 = local_64;
  do {
    pcVar10 = pcVar8;
    if (iVar9 == 0) break;
    iVar9 = iVar9 + -1;
    pcVar10 = pcVar8 + 1;
    cVar1 = *pcVar8;
    pcVar8 = pcVar10;
  } while (cVar1 != '\0');
  pcVar8 = pcVar11 - uVar6;
  pcVar11 = pcVar10 - 1;
  for (uVar5 = uVar6 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
    __builtin_memcpy(&uDword, pcVar8, 4);
    __builtin_memcpy(pcVar11, &uDword, 4);
    pcVar8 = pcVar8 + 4;
    pcVar11 = pcVar11 + 4;
  }
  for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
    *pcVar11 = *pcVar8;
    pcVar8 = pcVar8 + 1;
    pcVar11 = pcVar11 + 1;
  }
  FUN_004db640(0xc, local_64, &local_rect.local_88, local_70);
LAB_004f04bc:
  piVar7 = piVar7 + 2;
  local_8c = local_8c + 1;
  if ((int *)0xaa60cf < piVar7) {
    FUN_004d4970();
    if ((DAT_00b69cb0 == 0) && (DAT_00b69cbc == 0)) {
      return;
    }
    FUN_004eecb0(0, 0, *(int *)(DAT_0052c818 + 4), DAT_0052c80c);
    return;
  }
  goto LAB_004f03dc;
}
