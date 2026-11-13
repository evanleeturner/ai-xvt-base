/* SPDX-License-Identifier: MIT */
/* z_bop 004caf60 */
extern int DAT_00b6d962;
extern char *s_background0_0052b1a8;
extern char *s_background1_0052b19c;
extern char *s_frame_0052a6d4;
extern int DAT_00b6a250;
extern char *s_clientactive_0052b69c;
extern char *s_allactive_0052b6ac;
extern char *s_nameoverlay_0052b690;
extern int DAT_00b6a2b0;
extern char *s_regoverlay_0052b684;
extern char *s_awardoverlay_0052b674;
extern char *s_rebplaque_0052b668;
extern char *s_rebmedal_0052b65c;
extern char *s_rebtrophy_0052b650;
extern char *s_rebcitation_0052b644;
extern char *s_impplaque_0052b638;
extern char *s_impmedal_0052b62c;
extern char *s_imptrophy_0052b620;
extern char *s_impcitation_0052b614;
extern char *s_promotionoverlay_0052b600;

extern unsigned int __cdecl FUN_004d4f80(void);
extern unsigned int __cdecl FUN_004d4e20(void);
extern unsigned int __cdecl FUN_004b5de0(char *param_1, int param_2, int param_3);
extern unsigned int __cdecl FUN_004b5570(char *param_1, int param_2, int param_3);
extern unsigned int __cdecl FUN_004b4d40(char *param_1, int param_2, int param_3);
extern unsigned int __cdecl FUN_004d4ec0(int param_1);

unsigned int __stdcall FUN_004caf60(void)
{
  char *pcVar1;
  
  FUN_004d4f80();
  FUN_004d4e20();
  if (DAT_00b6d962 == 0) {
    pcVar1 = s_background0_0052b1a8;
  }
  else {
    pcVar1 = s_background1_0052b19c;
  }
  FUN_004b5de0(pcVar1, 0, 0);
  FUN_004b5570(s_frame_0052a6d4, 0, 0);
  if (DAT_00b6a250 == 0) {
    pcVar1 = s_clientactive_0052b69c;
  }
  else {
    pcVar1 = s_allactive_0052b6ac;
  }
  FUN_004b5570(pcVar1, 0, 0);
  FUN_004b4d40(s_nameoverlay_0052b690, 0, 0);
  switch (DAT_00b6a2b0) {
    case 0:
    case 3:
    case 4:
      pcVar1 = s_regoverlay_0052b684;
      break;
    case 1:
      FUN_004b4d40(s_awardoverlay_0052b674, 0, 0);
      if (DAT_00b6d962 == 0) {
        FUN_004b5570(s_rebplaque_0052b668, 0, 0);
        FUN_004b5570(s_rebmedal_0052b65c, 0, 0);
        FUN_004b5570(s_rebtrophy_0052b650, 0, 0);
        FUN_004b5570(s_rebcitation_0052b644, 0, 0);
      }
      else {
        FUN_004b5570(s_impplaque_0052b638, 0, 0);
        FUN_004b5570(s_impmedal_0052b62c, 0, 0);
        FUN_004b5570(s_imptrophy_0052b620, 0, 0);
        FUN_004b5570(s_impcitation_0052b614, 0, 0);
      }
      goto switchD_004cafdc_default;
    case 2:
      pcVar1 = s_promotionoverlay_0052b600;
      break;
    default:
      goto switchD_004cafdc_default;
  }
  FUN_004b4d40(pcVar1, 0, 0);
switchD_004cafdc_default:
  FUN_004d4ec0(1);
  return 1;
}
