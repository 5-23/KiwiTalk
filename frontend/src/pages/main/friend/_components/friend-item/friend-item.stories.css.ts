import { vars } from '@/features/theme';
import { style } from '@vanilla-extract/css';

export const background = style({
  background: vars.color.blue100,
  borderRadius: '18px',
  height: '300px',
  padding: '24px 10px',
});
